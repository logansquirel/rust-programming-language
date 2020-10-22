# chapter09/panic

<!-- markdownlint-disable MD013 -->
## Run

Without backtrace:

```console
$ RUST_BACKTRACE=0 cargo run --quiet --release
thread 'main' panicked at 'crash and burn', chapter09/panic/src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

With backtrace:

```console
$ RUST_BACKTRACE=1 cargo run --release --quiet
thread 'main' panicked at 'crash and burn', chapter09/panic/src/main.rs:2:5
stack backtrace:
   0: std::panicking::begin_panic
   1: panic::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

With **full** backtrace:

```console
$ RUST_BACKTRACE=full cargo run --release --quiet
thread 'main' panicked at 'crash and burn', chapter09/panic/src/main.rs:2:5
stack backtrace:
   0:     0x56322793d440 - std::backtrace_rs::backtrace::libunwind::trace::h622bab51c72c4e69
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/../../backtrace/src/backtrace/libunwind.rs:100:5
   1:     0x56322793d440 - std::backtrace_rs::backtrace::trace_unsynchronized::h7e820b882ebd41ee
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x56322793d440 - std::sys_common::backtrace::_print_fmt::h64d46258114db92f
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x56322793d440 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h47111d0c1b5f0da5
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x5632279559fc - core::fmt::write::h1857a60b204f1b6a
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/core/src/fmt/mod.rs:1076:17
   5:     0x56322793b912 - std::io::Write::write_fmt::h4604516fed3e5731
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/io/mod.rs:1516:15
   6:     0x56322793f5a5 - std::sys_common::backtrace::_print::h785e7a78d5ef272c
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x56322793f5a5 - std::sys_common::backtrace::print::h108047ac5c4555d5
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x56322793f5a5 - std::panicking::default_hook::{{closure}}::h1d0c271f0d087ebf
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/panicking.rs:208:50
   9:     0x56322793f248 - std::panicking::default_hook::h692753e26f11b7b7
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/panicking.rs:227:9
  10:     0x56322793fc21 - std::panicking::rust_panic_with_hook::h74ddc20305301cd9
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/panicking.rs:577:17
  11:     0x56322792c1b4 - std::panicking::begin_panic::{{closure}}::heaa03a4681db9b24
  12:     0x56322792c17c - std::sys_common::backtrace::__rust_end_short_backtrace::hfe7b563c44315164
  13:     0x56322792c22c - std::panicking::begin_panic::ha9a612e0b46878ae
  14:     0x56322792c2e9 - panic::main::h589edb3a8849d204
  15:     0x56322792c183 - std::sys_common::backtrace::__rust_begin_short_backtrace::he86e568adde15beb
  16:     0x56322792c1d9 - std::rt::lang_start::{{closure}}::h21e7d5cbcdbc6231
  17:     0x563227940047 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h5867dc20a6af6cac
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/core/src/ops/function.rs:259:13
  18:     0x563227940047 - std::panicking::try::do_call::h32e8dcce59f53a45
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/panicking.rs:381:40
  19:     0x563227940047 - std::panicking::try::h7d7b073280e6de2f
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/panicking.rs:345:19
  20:     0x563227940047 - std::panic::catch_unwind::h801333b3e4e7d79f
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/panic.rs:382:14
  21:     0x563227940047 - std::rt::lang_start_internal::h2c157bf657c6892a
                               at /rustc/1eaadebb3dee31669c7649b32747381d11614fae/library/std/src/rt.rs:51:25
  22:     0x56322792c312 - main
  23:     0x7f2c3b5f0152 - __libc_start_main
  24:     0x56322792c08e - _start
  25:                0x0 - <unknown>
```
<!-- markdownlint-enable MD013 -->
