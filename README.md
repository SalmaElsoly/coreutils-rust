# Coreutils

## Description

project implementing basic unix commands with rust

## Installation

```bash
git clone https://github.com/SalmaElsoly/coreutils-rust.git
cargo build --release
./target/release/<tool_name> <flags>
```

## Usage

Here are the usage instructions for each implemented command:

### 1. Cat

- `./cat -n <filepath>`
  -n : for numbered lines output

### 2. Echo

`./echo -n <string>`

- Print arguments to standard output.
- Add a -n flag to omit the trailing newline.

### 3. Tree

`./tree -L <num> <filepath>`

- To display a recursive directory tree
- If no file specifed it display tree of current directory
- Print files and directories up to 'num' levels of depth
- Default level is 1.

### 4. Head

`./head -n <num> <filepath>`

- Display the first 'num' lines of a file.
- If 'num' is not specified, the default is to display the first 10 lines.

### 5. Tail

`./tail -n <num> <filepath>`

- Display the last 'num' lines of a file.
- If 'num' is not specified, the default is to display the last 10 lines.

### 6. Wc

`./wc <flags> <filepath>`

- Count lines, words, and characters in the input.
- Add -l, -w, and -c flags to display only lines, words, or characters respectively.
