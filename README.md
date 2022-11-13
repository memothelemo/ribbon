## Ribbon

An alternative program to [lemur](https://github.com/LPGhatguy/lemur)
(not planning to be successor of it) that implements most of Roblox's API in
order to enable Roblox projects to have continuous integration without
depending on Roblox Studio in some CIs (like [run-in-roblox](https://github.com/rojo-rbx/run-in-roblox)).

**WARNING:**
Most deprecated methods and members are not supported (and never will implement them)!

## Demo
You may run this program by cloning the repository and run this command:
```
cargo run
```

**NOTE**: You'll need nightly version of Rust 1.64 (`2022-06-29`) and later because we dependent on nightly features to make Ribbon happen:

- [`trait_upcasting`](https://github.com/rust-lang/rust/issues/65991) - for downcasting Instances.

**Of course, it is nightly version of Rust. Please proceed this program with your own risk!**
