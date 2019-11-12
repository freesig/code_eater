\#S:MODE=code,INCLUDE

Lets test the following function:
```rust
pub fn double(n: i64) -> i64 {
  n + n
}
```
Using an itegration test:

\#S:MODE=test
```rust
#[test]
fn test_add() {
    assert_eq!(doubler::double(5), 10);
}
```
