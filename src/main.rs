// MIT License
//
// Copyright (c) 2017 Matthew Knight
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/*! mybin crate exists to import the mylib library and start the server from mylib::server;

# Usage

This example is one method of using the mylib library. With this method, you can have a binary and
a library in the same crate root.

Your Cargo.toml "can" look something like this to support a combined crate root.

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Your Name <yourname@email.com>"]

[dependencies]
# Project dependencies would go here

[lib]
name = "mylib"
path = "src/lib.rs"

[[bin]]
name = "mybin"
path = "src/main.rs"
```

Then you could import the mylib library and `cargo run` the program.

```rust,norun
extern crate mylib;

use mylib::server::start_server;

fn main() {
    start_server();
}
```
*/

#![warn(missing_docs)]

extern crate mylib;

use mylib::server::start_server;

fn main() {
    start_server();
}
