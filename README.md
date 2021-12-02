# Assignment for @FI_MUNI course "Rust Programming"

This is a Rust solution of implementing a very simple interpreter.
More about the Brainfuck language: [wikipedia](https://en.wikipedia.org/wiki/Brainfuck)

## Usage

```bash
# build a file
cargo build --release

# find the executable
./brainfuck-interpreter --file='<PATH TO FILE>'

# alternatively run via cargo
cargo run --release -- --file='<PATH TO FILE>'
```

To test against provided samples, simply run:

```bash
cargo test
```

## Example

```bash
cargo run --release -- --file=./test_files/hello-world.txt
```
