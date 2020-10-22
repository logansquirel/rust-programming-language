# chapter01/hello_cargo

Simple `hello world!` with `cargo`.

## Initialisation

Initialization of cargo project is done with

```console
cargo new <path>
```

or

```console
cd <path>
cargo init
```

## Configuration

Cargo configuration is in `Cargo.toml` file and use
[TOML](https://github.com/toml-lang/toml) format.

## Modes

Cargo provides two modes:

- *debug* for fast compilation time and debug information
- *release* for fast runtime

All files/executables are generated in a `target` directory.

## Usage

### Check

Compile the program.

```console
$ cd chapter01/hello_cargo
$ cargo check
    Checking hello_cargo v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
```

### Build

Compile the program and produce an executable.

#### Debug Mode Build

```console
$ cd chapter01/hello_cargo
$ cargo build
    Compiling hello_cargo v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
```

#### Release Mode Build

```console
$ cd chapter01/hello_cargo
$ cargo build --release
    Compiling hello_cargo v0.1.0
    Finished release [optimized] target(s)
```

### Running

#### Debug Mode Run

[Build](#debug-mode-build) project in *debug* mode first then:

##### Manual Debug Run

```console
$ cd chapter01/hello_cargo
$ ./target/debug/hello_cargo
Hello, world!
```

##### Cargo Debug Run

```console
$ cd chapter01/hello_cargo
$ cargo run
Hello, world!
```

#### Release Mode Run

[Build](#release-mode-build) project in *release* mode first then:

##### Manual Release Run

```console
$ cd chapter01/hello_cargo
$ ./target/release/hello_cargo
Hello, world!
```

##### Cargo Release Run

```console
$ cd chapter01/hello_cargo
$ cargo run --release
Hello, world!
```

### Clean

Clean `target` directory.

```console
cargo clean
```
