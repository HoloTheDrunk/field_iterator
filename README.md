# Field iterator

Based on the abandoned [struct_iterable](https://github.com/decomoraes/rust_struct_iterable). The rest of the README is taken from it.

This is meant as a lighter alternative to [fields-iter](https://github.com/ChayimFriedman2/fields-iter).

## How to Use

First, add the crate to your `Cargo.toml`:

```toml
[dependencies]
field_iterator = "0.1.1"
```

Next, include the library at the top of your Rust file:

```rust
use field_iterator::Iterable;
```

Finally, add the `#[derive(Iterable)]` attribute to your struct:

```rust
#[derive(Iterable)]
struct MyStruct {
    field1: u32,
    field2: String,
    // etc.
}
```

Now, you can iterate over the fields of an instance of your struct:

```rust
let my_instance = MyStruct {
    field1: 42,
    field2: "Hello, world!".to_string(),
};

for (field_name, field_value) in my_instance.iter() {
    println!("{}: {:?}", field_name, field_value);
}
```

## Limitations

- Only structs with named fields are supported.

## Contributing and License

If you're interested in contributing, please feel free to submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

This crate is distributed under an MIT license.
