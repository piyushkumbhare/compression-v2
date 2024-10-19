# Examples

There are a few notable examples in this directory, here are their stats and how they perform under compression:

1. `full_bible_copy_paste.txt` - Literal full copy paste from [here](https://www.gutenberg.org/cache/epub/8300/pg8300.txt).
    1. Original Size: 5512050 B
    2. Compressed Size: 1410279 B
    3. Total compression: 74.41%

2. `bible_source_code.html` - HTML source code of [this page](https://trulyfreebible.com/).
    1. Original Size: 305635 B
    2. Compressed Size: 90694 B
    3. Total compression: 70.33%

3. `small_example.txt` - A small example text file I made.
    1. Original Size: 274 B
    2. Compressed Size: 778 B
    3. Total compression: -183.94%

4. `compression-v2_sample` - A copy of the debug binary of *this very program*.
    1. Original Size: 17054784 B
    2. Compressed Size: 4595601 B
    3. Total compression: 73.05%

<br>
As you can see, larger files can be compressed to around 25-30% of their original size, (seemingly) regardless of whether they're in written language or binary data.