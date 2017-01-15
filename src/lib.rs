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

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://mattlknight.github.io/rust-playground/")]

/*!
This crate provides a library for experimenting and testing rust features on a
rapid create/test/modify cycle. This crate will provide examples of various
complex use cases for traits, implementations, associated types, etc..

This crate is licensed under the MIT license. You can find a copy of the license at the top of
`src/lib.rs`, `src/main.rs`, in the project root as `LICENSE`, as well as in the root of the
[GitHub Repo - rust-playground](https://github.com/mattlknight/rust-playground)

This crate's documentation provides some simple examples. Check out
[Modules](#modules) for the various types, traits, and other rust features used in this project.

Example enum Error [`SqlError`](enum.SqlError.html) type.

Example trait [`SqlSafe`](trait.SqlSafe.html) type.

# Usage

This crate is NOT [on crates.io](https://crates.io/), but can be obtained from my public
[GitHub Repo - rust-playground](https://github.com/mattlknight/rust-playground). This crate can be
used by adding the below lines to your dependencies in your project's `Cargo.toml`, if you want to
 have the main.rs and lib.rs in the same project src/ directory.

```toml
[lib]
name = "mylib"
path = "src/lib.rs"

[bin]
name = "mybin"
path = "src/main.rs"
```

and this to your crate root:

```rust
extern crate mylib;
```

After you have cloned a copy of this project from the GitHub repo, you can run any of the
following sommands from within the project root:

```text
cargo build     # Build the project from src
cargo run       # Build/Run the project
cargo test      # Test the embedded test cases
cargo bench     # Test the performance of the Bench test cases
cargo doc       # Generate this same documentation locally
```


# Example: Validate a string for SQL Injection Safety

To use a public type from this module, add the below to your crate root. This example `use`s the
 [`SqlSafe`](trait.SqlSafe.html) trait. This trait
overloads the `&str` type with a `is_sql_safe()` method.

```rust
extern crate mylib;

use mylib::types::SqlSafe;
# fn main() { }
```

The example shows how to use the method and handle it's return. You could put this in your
 `fn main() { }`

```rust
# extern crate mylib; use mylib::types::SqlSafe;
# fn main() {
let username = "Robert'); DROP TABLE Students;--?";
//   try valid username
// let username = "Bobby Tables";

match username.is_sql_safe() {
    Ok(name) => println!("Username \"{}\" is safe", name),
    Err(err) => {
        println!("Error: Username \"{}\" is NOT SQL safe!", username);
        println!("Error: {}", err);
        println!("Error: {}", err.description());
    },
};

assert!("username;".is_sql_safe().is_err());
assert!("username".is_sql_safe().is_ok());
# }
```

The result will look something like this in the console output:

```text
Error: Username "Robert'); DROP TABLE Students;--?" is NOT SQL safe!
Error: found unsafe sql characters in sql string `');'
Error: sql syntax not allowed
```

*/


#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;
extern crate regex;

mod sql;
pub mod errors;
pub mod types;
