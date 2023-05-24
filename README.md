<div align = "center">

# SSTTT

More **S**yn **S**yntax **T**ree **T**raversal **T**rait

![GitHub](https://img.shields.io/github/license/madylab/ssttt?style=for-the-badge) [![Crates.io](https://img.shields.io/crates/v/ssttt?style=for-the-badge)](https://crates.io/crates/ssttt) [![docs.rs](https://img.shields.io/docsrs/ssttt?style=for-the-badge)](https://docs.rs/ssttt/0.1.0/ssttt/)

SSTTT crate provide some useful trait for syn syntax tree traversal.
</div>

## Example

fallible [Fold](https://docs.rs/syn/latest/syn/fold/index.html)

```rust
pub trait TryFold {
    type Error;

    fn try_fold_abi(&mut self, t: syn::Abi) -> Result<syn::Abi, Self::Error>
    {
        try_fold_abi(self, t)
    }

    // ...
}
```

fallible take (like fold but without return new value)

```rust
pub trait TryTake {
    type Error;

    fn try_take_abi(&mut self, t: syn::Abi) -> Result<(), Self::Error> {
        try_take_abi(self, t)
    }

    // ...
}
```
