# wk9-mini-rust-jl1188:
A Rust Mini Project command-line tool that evaluates a reverse polish notation expression. 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- evaluate --input 4,13,5,/,+</code>
 
 The command line tool uses the subcommand "<code>evaluate</code>" and takes in one argument, a <code>string</code> named "<code>input</code>," as the above examples shows. Each token of the reverse polish notation expression should be seperated by delimiter ",". The result should be outputted as "The evaluated result is: 6". 