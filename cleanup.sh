# Directory to start searching from (use '.' for the current directory)
start_dir="."

# Find and delete files matching the patterns, but ignore the 'examples/' directory
find "$start_dir" -path "./examples" -prune -o -type f \( -name "*.pkz" -o -name "*.decoded" \) -exec rm -v {} \;
