# Creating a tutorial with a runnable program
## Create a minimal working program.

```rust
fn main() {
    println!("Hello world");
}
```
## Break the code up anywhere you think needs explanation.

### For example
The main function that runs the program.
```rust
fn main() {
```
Print to the console.
```rust
    println!("Hello world");
}
```
## Add in features with explanation.
The mark `\#S:` tells single source that we have some flags for it.
The `INCLUDE` flag tells single source to include any code from here on unless it gets a `SKIP` flag.

### For example
This struct is used to store a static str.
```rust
#[derive(Debug)]
struct HelloWorld {
    text: &'static str,
}
```
## Include / Skip / Hide code.
All rust code will be included at this point in the document but you might want to skip some code from using in your program.

### For example
We could also write this as:
```rust
#[derive(Debug)]
struct HelloWorld {
```
Storing the text as a String.
```rust
    text: String, 
}
```

The previous two block will not be included in the program.
To include code you need to:

```rust
fn main() {
    let hello_world = HelloWorld {
        text: HELLO_WORLD,
    };
    println!("{:?}", hello_world);
}
```

Then maybe you want to add some code that will be included but not visible in the tutorial.

> Note that HIDE will only hide the block directly following.


## Run and build the program.
Build the program.
```bash
single_source code docs/runnable_tutorial.md ~/rust/test_tutorial/src/main.rs rust
```
Build the documentation.
```bash
single_source md docs/runnable_tutorial.md docs/runnable_tutorial_rendered.md
```
You can also render a check in the document to see what the code looks like.

??? question "Check your code"
    ```rust
    #[derive(Debug)]
    struct HelloWorld {
        text: &'static str,
    }
    fn main() {
        let hello_world = HelloWorld {
            text: HELLO_WORLD,
        };
        println!("{:?}", hello_world);
    }
    const HELLO_WORLD: &'static str = "HELLO_WORLD";
    ```

<script id="asciicast-azlIMKjZ5UYecjT35fhPotb9n" src="https://asciinema.org/a/azlIMKjZ5UYecjT35fhPotb9n.js" async></script>

