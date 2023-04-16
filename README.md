# sub_if

A crate to make it so you don't have to do things like:

```rs
let x = if condition {
    a - b
} else {
    a
};
```

And can rather do:

```rs
let x = a.sub_if(condition, &b);
```
