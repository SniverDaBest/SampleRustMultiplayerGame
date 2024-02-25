# What is this?
This is a sample for a multiplayer game written in Rust. It uses TCP on port 1169 for it's connection type, and uses piston_window for it's graphical library. This code is a modified version of an old version of Untitled Pixel Game, a.k.a the game I'm creating.

# Why did you make this?
I have many reasons why I wanted to make this. I will list a few.
- It helps beginners start coding
- It's a good template for a multiplayer game
- It's some code that I can just copy and paste into future projects. You can do the same, but please just put my name into the source code, so you can tell it's mine, and not yours.

# Some answers to some questions
## How do I build this?
To build this, don't use `cargo run` or `cargo build` unless you want the actual game, which I haven't worked on yet. To build this, run `rustc src/client.rs` and `rustc src/server.rs`.
## How do I run this?
To run this, run `.\server` on Windows in a terminal, and in another terminal window, run `.\client [ip] [port] [nickname]`. Obviously replace ip and port, as well as nickname with their respective values. Port should be 1169 *nice*, and if the server is local, do `127.0.0.1`.
## I have two clients open. Why can't I see what one client sends on the other client?
It's easy. I just haven't added it yet.
