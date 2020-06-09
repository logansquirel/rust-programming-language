# Rustup

`rustup` is a command line tool for managing **Rust** versions and associated tools.

## Resources

- Website: <https://rustup.rs/>
- Repository: <https://github.com/rust-lang/rustup/>

## Installation

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Profiles

`rustup` provides multiple profiles for installing only needed tools:

- **minimal** : include `rustc`, `rust-std` and `cargo`
- **default**: include **minimal** components and `rust-doc`, `rust-fmt` and `clippy`
- **complete**

## Toolchains

Common toolchains:

- *stable*
- *beta*
- *nightly*

## Documentation

```console
rustup help
```

## Commands

### Update

```console
rustup update
```

### Install toolchain(s)

```console
rustup install <toolchain>
```

### Change default toolchain

```console
rustup default <toolchain>
```
