This is a distillation of [citeproc-rs](https://github.com/citeproc-rs) to show 
a minimal reproduction of a segfault I found when running in release mode, 
compiled on a Mac using Rust nightly in September 2020.

The code's still a bit messy, but it's reduced from a fairly large project and 
around this point in the minimisation, the segfault started to change a bit, as 
the function was being optimised differently, so I didn't want to do much more. 
**There is no unsafe code, and it has no dependencies.**

Configuration:

- rustc 1.48.0-nightly (0da580074 2020-09-22)
- cargo 1.48.0-nightly (8777a6b1e 2020-09-15)
- macOS 10.14.6
- MacPro5,1


## RUN:

```sh
$ cargo run --release
   Compiling minimal v0.1.0 (/Users/cormac/git/tryout/minimal-sigsegv-rust)
    Finished release [optimized] target(s) in 1.50s
     Running `target/release/minimal`
added LocatorLabel
Edge(Some(fish: 'cargo run --release' terminated by signal SIGSEGV (Address boundary error)

$ cargo miri run # note no errors
```

As you can see, the program segfaults in the middle of a debug print routine. 
It has managed to write part of an enum structure out, but proceeds to access 
some memory it shouldn't. Presumably this happens when it reads an invalid enum 
discriminant, and the switch in the LLVM IR hits the 'unreachable' first 
branch, and starts attempting to debug the `EdgeData::Output(String)` variant 
as it is the first one and therefore the next instruction. The crucial 
functions are in `disamb/mod.rs`: `element_ref_ir_impl` and `ref_sequence`, 
which are mutually recursive and are constructive a recursive enum `RefIR`, 
defined in `ref_ir.rs`.

When I found the original test case this is based on, I found it was crucial 
that (see main.rs) there be a 'macro call' in there, i.e. that the two 
functions  recurse into each other once. It would not trigger a segfault 
without doing that.

## Valgrind output


```
==83923== Memcheck, a memory error detector
==83923== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==83923== Using Valgrind-3.16.0.GIT and LibVEX; rerun with -h for copyright info
==83923== Command: target/release/minimal
==83923==
==83923== Invalid read of size 16
==83923==    at 0x1005A4609: _platform_memchr$VARIANT$Base (in /usr/lib/system/libsystem_platform.dylib)
==83923==    by 0x100013D0B: std::thread::Thread::new (memchr.rs:6)
==83923==    by 0x10001A269: std::rt::lang_start_internal (rt.rs:44)
==83923==    by 0x100004538: main (in target/release/minimal)
==83923==  Address 0x101086e30 is 12 bytes after a block of size 4 alloc'd
==83923==    at 0x10012FAD5: malloc (in /usr/local/Cellar/valgrind/HEAD-60ab74a/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==83923==    by 0x10001A23F: std::rt::lang_start_internal (alloc.rs:74)
==83923==    by 0x100004538: main (in target/release/minimal)
==83923==
==83923== Conditional jump or move depends on uninitialised value(s)
==83923==    at 0x1005A4630: _platform_memchr$VARIANT$Base (in /usr/lib/system/libsystem_platform.dylib)
==83923==    by 0x100013D0B: std::thread::Thread::new (memchr.rs:6)
==83923==    by 0x10001A269: std::rt::lang_start_internal (rt.rs:44)
==83923==    by 0x100004538: main (in target/release/minimal)
==83923==
added LocatorLabel
Edge(==83923== Conditional jump or move depends on uninitialised value(s)
==83923==    at 0x100004551: <&T as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E162: core::fmt::builders::DebugTuple::field (builders.rs:347)
==83923==    by 0x1000037E7: <minimal::ref_ir::RefIR as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E90F: core::fmt::write (mod.rs:1080)
==83923==    by 0x100015033: <&std::io::stdio::Stderr as std::io::Write>::write_fmt (mod.rs:1517)
==83923==    by 0x100015595: std::io::stdio::_eprint (stdio.rs:812)
==83923==    by 0x100002C3A: minimal::disamb::element_ref_ir_impl (in target/release/minimal)
==83923==    by 0x10000423E: minimal::main (in target/release/minimal)
==83923==    by 0x1000022F9: std::sys_common::backtrace::__rust_begin_short_backtrace (in target/release/minimal)
==83923==    by 0x10000235B: _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h254a4aa9b725da66E.llvm.9306634996709519214 (in target/release/minimal)
==83923==    by 0x10001A28F: std::rt::lang_start_internal (function.rs:259)
==83923==    by 0x100004538: main (in target/release/minimal)
==83923==
Some(==83923== Use of uninitialised value of size 8
==83923==    at 0x100002588: <&T as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E162: core::fmt::builders::DebugTuple::field (builders.rs:347)
==83923==    by 0x10000459E: <&T as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E162: core::fmt::builders::DebugTuple::field (builders.rs:347)
==83923==    by 0x1000037E7: <minimal::ref_ir::RefIR as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E90F: core::fmt::write (mod.rs:1080)
==83923==    by 0x100015033: <&std::io::stdio::Stderr as std::io::Write>::write_fmt (mod.rs:1517)
==83923==    by 0x100015595: std::io::stdio::_eprint (stdio.rs:812)
==83923==    by 0x100002C3A: minimal::disamb::element_ref_ir_impl (in target/release/minimal)
==83923==    by 0x10000423E: minimal::main (in target/release/minimal)
==83923==    by 0x1000022F9: std::sys_common::backtrace::__rust_begin_short_backtrace (in target/release/minimal)
==83923==    by 0x10000235B: _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h254a4aa9b725da66E.llvm.9306634996709519214 (in target/release/minimal)
==83923==
==83923== Invalid read of size 4
==83923==    at 0x100002588: <&T as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E162: core::fmt::builders::DebugTuple::field (builders.rs:347)
==83923==    by 0x10000459E: <&T as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E162: core::fmt::builders::DebugTuple::field (builders.rs:347)
==83923==    by 0x1000037E7: <minimal::ref_ir::RefIR as core::fmt::Debug>::fmt (in target/release/minimal)
==83923==    by 0x10002E90F: core::fmt::write (mod.rs:1080)
==83923==    by 0x100015033: <&std::io::stdio::Stderr as std::io::Write>::write_fmt (mod.rs:1517)
==83923==    by 0x100015595: std::io::stdio::_eprint (stdio.rs:812)
==83923==    by 0x100002C3A: minimal::disamb::element_ref_ir_impl (in target/release/minimal)
==83923==    by 0x10000423E: minimal::main (in target/release/minimal)
==83923==    by 0x1000022F9: std::sys_common::backtrace::__rust_begin_short_backtrace (in target/release/minimal)
==83923==    by 0x10000235B: _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h254a4aa9b725da66E.llvm.9306634996709519214 (in target/release/minimal)
==83923==  Address 0x4000022dd8026a8 is not stack'd, malloc'd or (recently) free'd
==83923==
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue
==83923== Signal 11 being dropped from thread 0's queue

(My valgrind does this and loops forever from time to time, I'm not sure what that's all about)
```

