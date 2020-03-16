# Pipeviewer
This is a Rust implementation of the [Unix pv](http://www.ivarch.com/programs/pv.shtml)
utility. I don't intend to make a faithful implementation, I just replicated
some features for fun and learning.

## Usage
```shell
echo "hello" | cargo run -- -o /dev/null # Printing "hello" message to the /dev/null
yes | cargo run | head -n 1000000000 > yes.txt # Generating a large text file
cargo run yes.txt -o yes2.txt # Copying content from file `yes.txt` to `yes2.txt`
```

## Running tests
```shell
cargo test
```
