# Single Source Of Truth
This ius a simple utility that allows you to generate working code from your markdown files.
This is super early alpha so there's probably missing parts and it's a bit hacky.

## Installation
Single source can be installed through cargo like:
```bash
cargo install single_source
```

## Setup
Just put you code in your md file as usual marked with the language like:
\`\`\`\rust
fn my_cool(stuff: ()) {
}
\`\`\`
Then proceding that you mark it with the flag `\#S:` (yeh I know it's a weird marker but try finding something markdown doesn't use) and some tags.
### Reference
[Refence](https://single.nottodaycaesar.com/reference/)

## Usage
### Generate Code
```bash
single_source code path/to/source.md path/to/code_file_to_be.generated lang
```
Example:
```bash
single_source code path/to/my_tutorial.md path/to/lib.rs rust 
```
### Generate Markdown 
```bash
single_source md path/to/source.md path/to/md_file_to_be_generated.md
```
Example:
```bash
single_source md path/to/my_tutorial_source.md path/to/finished_tutorial.md
```
### Tutorials
[Integration tests](https://single.nottodaycaesar.com/tutorial/)
[runnable tutorials](https://single.nottodaycaesar.com/runnable_tutorial/)

## Example
\#S:EXTERNAL=external.rs
(note this is actually `\#S:EXTERNAL=external.rs` but your md render might remove the `\`)
Some tutorial stuff 
Some tutorial stuff 
Some tutorial stuff 

Skip this because it's just for information purposes and should not be included in the generate code file.

\#S:SKIP
```rust
// <---- Add the struct here.

mod something {
```
This code block also gets skipped.
```rust
fn open() {}
```
This one will be included

\#S:INCLUDE
```rust
fn say() {
  println!("hello");
}
```
also included
```rust
fn say2() {
  println!("hello2");
}
```
Going to hide the closing tag for the mod that we opened in external.rs

\#S:HIDE
```rust
}
```

### external.rs:
```rust
mod my_mod {
```
