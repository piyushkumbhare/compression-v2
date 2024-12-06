# Compression-v2

### Note
This project is the second version of my original compression project [here](https://github.com/piyushkumbhare/compression).

The biggest difference between the old and current versions is:
- Program now compressed files based on their byte content (`Vec<u8>`), rather than strings (`&str`)
- Repo is structured much better (no more trait nonsense, most things just stick to simple function calls)
- Current version now has Huffman Encoding, which plays a **huge** role in compression rate

## How to use
The project can be built using regular `cargo` rules.

Optionally, the project can be compiled for fuzzing via `cargo afl build`, though this requires [Cargo support for AFL++](https://rust-fuzz.github.io/book/afl.html).

Here's a list of supported CLI flags, which can be seen with `./compression-v2 --help`:

```
Compresses an input file into an output (with extension .pkz)

Usage: compression-v2 [OPTIONS] <INPUT_PATH>

Arguments:
  <INPUT_PATH>
          File to compress

Options:
  -d, --decompress
          Decompress instead of Compress. Expects a .pkz file as input

  -s, --stdout
          Redirect output to stdout. Does not create a .pkz file

  -q, --quiet
          Hide debug output

  -p, --pipeline <PIPELINE>...
          Provide a custom Encoding Pipeline in a space-separated list. Ignored if --decompress is used.
          
          Possible options (also the default): Bwt Mtf Rle Huff

  -c, --check-integrity
          Performs the compression and verifies that it decodes to the original content. Ignored if --decompress is used

  -h, --help
          Print help (see a summary with '-h')
```

As mentioned above, standard compression outputs a `.pkz` file, while decompression necessarily *requires* a `.pkz` file as input, as to avoid accidental decoding.

## How it works
This section will cover my thought process while implementing each of these algorithms, along with some notes/discoveries I made along the way.
### Encoding Pipeline
This project, like its predecessor, supports a variable encoding pipeline. This means that each stage of (de)compression strictly takes in a `Vec<u8>` and outputs a `Vec<u8>`, allowing for encoders to be chained together.

Custom encoding pipelines can be specified with the `-p, --pipeline` flag, followed by a list of encodings. Encodings can be repeated and used in any order, but must be one of the following:
- BWT (Burrows-Wheeler Transform)
- MTF (Move-To-Front)
- RLE (Run-Length-Encoding)
- HUFF (Huffman Coding)

(In the future, I plan to add `BBWT`, a blocked version of the BWT, which would allow for parallel processing)

### BWT

The [Burrows-Wheeler-Transform](https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform) is a process which permutes a sequence of bytes in such a way that produces long runs of repeated bytes. As you can imagine, this sets up the scene for encoders like MTF and RLE, which benefit from such patterns.

The true BWT relies on an end-of-sequence character, typically denoted by `$`, which is defined to be lexicographically "less than" all other characters. The first challenge while implementing this algorithm was figuring out a way to denote the existence of `$` in the encoded output. 

The only character than can meet this "infimum" property is `0x0`, though in many programs, `0x0` is the most common byte. In fact, regardless of what character I choose here, I will *always* have to face the problem of "escaping". 

To imagine this, consider the input sequence: `banana`. To prep this sequence for the BWT, let's append an arbitrary end-of-sequence character, which for the sake of this example, will happen to be `a`. Thus, what gets inputted to the core BWT algorithm is the string `bananaa`. 

After permuting the string, we will theoretically get, `annb$aa`, but due to our explicitly chosen delimiter, we instead get `annbaaa`. But how can we even begin to decode this?

As mentioned before, we can remedy the situation by "escaping" the character we choose as our delimiter. Hence, to prep `banana`, we need to perform **2 replacements**:
1. Replace all occurrences of `\` with `\\`
2. Replace all occurrences of `a` with `\a`

The first replacement is required to allow the second one to work, as without it, the input `banan\a` would contain a sequence `\\a` post-prep. And by definition of "escape", the first `\` escapes the second `\`, which is NOT what we want.

Turns out, all of this horrible delimiter nonsense can be avoided by using a smarter representation. Since we know there is **only one** delimiter in the BWT output, we can get away with only knowing *where* it is. 

To do this, the in-memory program will view the input as a list of *Tokens*, where a Token Enum is defined as:

```rust
enum BwtToken {
    Delim,
    Byte(u8),
}
```

After it finishes processing, the program will convert the `Vec<BwtToken>` to a `Vec<u8>`, and simply skip the `BwtToken::Delim` case. The 0-indexed position of the delimiter is then prepended to the output in the form: `position|output`, where `position` is written as a base-36 integer.

So, our previous example of `banana` would be encoded as `4|annbaa`.

### MTF

The [Move-To-Front](https://en.wikipedia.org/wiki/Move-to-front_transform) transform is another encoding process, which aims to take advantage of "recently used symbols". The details of this algorithm can be found at the Wikipedia page (liked).

The above link discusses a common approach of using a "pre-defined alphabet", which is manually constructed via textual tendencies within most data. My approach uses the implementation described in Brandon Simmons' blog post [here](brandon.si/code/an-adaptive-move-to-front-algorithm/).

There weren't any interesting things to note from this section, the implementation was pretty one-to-one from Simmons' website, with a few small adjustments due to Rust's syntax.

### RLE

[Run-Length-Encoding](https://en.wikipedia.org/wiki/Run-length_encoding) is a **very** straightforward encoding scheme that I'm sure almost *everyone* is familiar with. The gist of the algorithm is:

*Grunts*. "Caveman see repeated sequence, Caveman replace with number." *Grunts*.

In theory, it's very simple, but implementation has some annoying inconveniences. As the BWT foreshadowed, we have to "escape" characters. It's pretty easy to see why, with the following `input` -> `encoded` -> `decoded` cases:

`aaaaabbbb` -> `5a4b` -> `aaaaabbbb` (correct)

`33333bbbb` -> `534b` -> `bbbbbbbb....b` (incorrect)

The obvious fix is to add a delimiter character before each `count` number, resulting in:
`aaaaabbbb` -> `\5a\4b` -> `aaaaabbbb` (correct)
`33333bbbb` -> `\53\4b` -> ?

Wait, how should the decoder interpret the second case? Is it `3` x 5 followed by `b` x 4? Or is it `\` x 53 followed by `b`?

Additionally, small sequences of characters face the bloating problem (`aa` -> `\2a`), where the replacement *takes more space* than the original. 

In order to solve ambiguous cases like this and other special cases, I created my RLE with the following rule set:
1. Replace all occurrences of `\` with `\\`
2. Replace all occurrences of `a` with `\a`
3. Only perform an RLE replacement if  $4 \leq count \leq 255$ 

Why constrain the replacement requirements to $[4, 255]$? Note that `aaa` -> `\3a` (same size), but `aaaa` -> `\4a` (1 byte less). As for the upper bound of `255` (or `u8::MAX`), this ensures that `count` can be represented with a single byte. Without this, we would have to deal with *double* delimiters (i.e `[count]byte`), which just sound like a pain in the ass, and that's enough for me to avoid it. Besides, a running sequence of *over 255 repeated bytes* is very unlikely, meaning we're really not losing out on much with this approach.

Fun fact:  *Most* of my debugging painfully led me back to this encoder. It has proven time and time again to be a pain in the ass, so perhaps there is a better way to implement it.

### Huffman Coding

Last, but certainly not least, we have [Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding), one of the most popular compression techniques used today. Very much like MTF, the implementation details of this encoding scheme aren't too special. 

The only place where I "freestyled" was with the (de)serialization of the Huffman tree. The size (in nodes) of the Huffman tree is upper bounded by $2 \cdot 255 + 1 = 511$, as can be seen in the picture:

![Image of degenerate Huffman tree](https://www.researchgate.net/publication/306085393/figure/fig4/AS:394610267443204@1471093827215/The-Huffman-Fibonacci-coding-tree-for-the-eight-Fibonacci-numbers-as-symbol-frequencies.png)

(Imagine the bottom-right node starts at `0x00` and goes up to `0xFF`, resulting in 256 leaf nodes and 255 internal nodes)

Since the max tree size is 511 nodes, I decided to (de)serialize the tree by prepending the Pre-Order & In-Order traversals to the Huffman encoded output.

The rest of the implementation is very standard, requiring lots of bit-wise operations and some padding. 

