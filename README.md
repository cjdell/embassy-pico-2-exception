Run with:

    cargo run --bin log

Causes the following error:

```
0.001394 [INFO ] Start (log src/bin/log.rs:25)
Firmware exited unexpectedly: Exception
Core 0
    Frame 0: __euninit @ 0xe012d308> @ 0x00000000e012d308

    Frame 1: HardFault <Cause: Escalated BusFault (Instruction prefetch)> @ 0x9807f866

    Frame 2: __euninit @ 0x9807f866> @ 0x000000009807f866

    Frame 3: External interrupt #0 @ 0x10002e64

    Frame 4: into_iter<&mut cordyceps::stack::Stack<embassy_executor::raw::TaskHeader>> @ 0x10002e64
       /Users/cjdell/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:324:6
    Frame 5: drop_glue<cordyceps::stack::Stack<embassy_executor::raw::TaskHeader>> @ 0x10001c14
       /Users/cjdell/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:825:1
    Frame 6: RunQueue::dequeue_all<embassy_executor::raw::{impl#9}::poll::{closure_env#0}> @ 0x10001bf8
       /Users/cjdell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.10.0/src/raw/run_queue.rs:92:9
    Frame 7: SyncExecutor::poll @ 0x10002ae4
       /Users/cjdell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.10.0/src/raw/mod.rs:472:24
    Frame 8: Executor::poll @ 0x10002b3c
       /Users/cjdell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.10.0/src/raw/mod.rs:580:20
    Frame 9: Executor::run<log::__cortex_m_rt_main::{closure_env#0}> @ 0x1000073e
       /Users/cjdell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embassy-executor-0.10.0/src/platform/cortex_m.rs:105:32
    Frame 10: __cortex_m_rt_main @ 0x100006f4
       /Users/cjdell/Projects/tmp/embassy-pico-2-exception/src/bin/log.rs:21:1
    Frame 11: __cortex_m_rt_main_trampoline @ 0x10000724
       /Users/cjdell/Projects/tmp/embassy-pico-2-exception/src/bin/log.rs:21:1
    Frame 12: Reset @ 0x10000176> @ 0x0000000010000176

Core 1
    Frame 0: <unknown function @ 0x000000da> @ 0x000000da
       /rustc/e50aa6fba4e63ab34c72bf9acfd2c307c1155d1a/library/compiler-builtins/compiler-builtins/src/macros.rs:481:14
    Frame 1: __euninit @ 0xfa04fffe> @ 0x00000000fa04fffe

Error: Exception
```

Needs `probe-rs` installing from: https://github.com/probe-rs/probe-rs/pull/3913

   cargo install --git https://github.com/akiles-dev/probe-rs.git --branch thumbv8m-exception-unwind probe-rs-tools
