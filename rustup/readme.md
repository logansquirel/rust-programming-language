# Rustup

`rustup` is a command line tool for managing **Rust** versions and associated tools.

## Resources

- Website: <https://rustup.rs/>
- Repository: <https://github.com/rust-lang/rustup/>
- Help: `rustup help`

## Installation

Command-line/Terminal installation:

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

## Commands

### Update

Update all installed toolchains:

```console
rustup update
```

### Install toolchain(s)

Install a specific toolchain `stable|beta|nightly|<version>`:

```console
rustup install <toolchain>
```

### Change default toolchain

Change default toolchain to `stable|beta|nightly|<version>`:

```console
rustup default <toolchain>
```

### Documentation

Open local documentation (including books and standard library):

```console
rustup doc
```
