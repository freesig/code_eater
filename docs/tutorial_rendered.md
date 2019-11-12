# Single Source Integration Test Tutorial
In this tutorial I will demonstrate how to use single source to create an integration test from a tutorial.
## Write how to use your new feature.
I want to add the feature to the single source code base that creates integration tests from the source code.
To use this feature create a simple markdown document with the single source preprocessor flags.
We will create a [really simple](../sample_test) to start.
## Create what you think the API should look like as code.
The content of what's in the `rust` blocks is not really important here. It could be any program.  
The important part is the `\#S:` flags.
This is how you tell single source what to do.
You can render this page using:
```
single_source md docs/tutorial.md docs/tutorial_rendered.md
```
Check out what it looks like [here](../tutorial_rendered).

Here we are saying set the mode to `test` and include the blocks that come after this point.


Here we are hiding just the following block from the rendered md.

Here the block continues and is included in the output program but isn't hidden from the rendered markdown.
```rust
#[test]
fn ss_integration() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("single_source")?;
    let tmp_dir = TempDir::new("tmp")?;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let sample_test = format!("{}/{}", manifest_dir, "docs/sample_test.md");
    Command::new("cargo")
        .args(&["new", "doubler", "--lib"])
        .current_dir(tmp_dir.path())
        .output()?;
    cmd.args(&["code", &sample_test, "src/lib.rs", "rust", "code"])
        .current_dir(tmp_dir.path().join("doubler"));
    cmd.assert()
        .success();
```
It's fine to break up blocks to explain sections like this.
```rust
    let mut cmd = Command::cargo_bin("single_source")?;
    cmd.args(&["code", &sample_test, "src/lib.rs", "rust", "test"])
        .current_dir(tmp_dir.path().join("doubler"));
    cmd.assert()
        .success();
    Command::new("cargo")
        .args(&["test"])
        .current_dir(tmp_dir.path().join("doubler"))
        .output()?;

    Ok(())
}
```
## Generate the tests.
Now generate this test by calling single_source on this file:
This says we want to generate the code from the `docs/tutorial.md` and put it into the `tests/integration.rs` file.
We also just want the blocks marked `rust` and set to `MODE=test`.
```bash
single_source code docs_src/tutorial.md tests/integration.rs rust test
cargo test
```
## Make the test pass.
Now you will want to make you're code pass these tests and feedback into the documentation any changes.
