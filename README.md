# sub_if

Doesn't only have `sub_if`!

A crate to make it so you don't have to do things like:

```rust
let x = if condition {
    a - b
} else {
    a
};
```

And can rather do:

```rust
let x = a.sub_if(condition, &b);
```
