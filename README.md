## Ribbon

An alternative program to [lemur](https://github.com/LPGhatguy/lemur) made in [Rust programming language](https://rust-lang.org) that attempts to implement most of Roblox's API in order to enable Roblox projects to have continuous integration without depending on Roblox Studio and allow to run it natively outside Windows and MacOS.

## How is it possible to implement this Rust?

Any object implementation is copied and inspired from [anyhow's source code using vtables](https://github.com/dtolnay/anyhow/blob/master/src/error.rs#L591) (all thanks to dtolany) to safely reference the object without potential memory issues.

In the OOP side, I implemented it in my own by relying on the ClassName
object and work way in to the parent of the inherited classes until `BaseInstance`.

## Warning

- Most deprecated methods and members are not supported except commonly used ones like `:SetPrimaryPartCFrame()`.

- This program is experimental and should not be used with games or CIs that
are critical or in production.

    *Please use at your own risk if you decided to use this in program in production and critical builds because there might be unexpected crashes. (especially segmentation faults)*
    
    *If you encounter unexpected crashes to the engine, please file an issue and I will try to fix it. (You can also send a pull request to if you're willing to fix the problem)**

## Demo

You may run this program by cloning the repository and run this command:
```
cargo run
```
