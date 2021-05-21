# unitest

> *small and compact testing framework*

## Usage

```rust
fn main() {}

unit!(
  test!(should_be_eq, must!(eq: 0, 0));
  test!(should_be_ne, must!(ne: 1, 0));
  test!(should_be_true, must!(truthy: true));
  test!(should_be_false, must!(falsy: false));
  test!(should_be_catched, must!(die: { panic!() } ));
);
```
