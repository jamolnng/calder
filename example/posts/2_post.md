---
title: 2222
date: 24-03-2022
desc: asdf things
type: _templates/post
tags: [asdf, things, stuff]
---

# My first post

My first post

```rust
fn main() {
  let args: Vec<String> = std::env::args().collect();
  let build = args.iter().find(|s| s.to_lowercase() == "build").is_some();
  let minify = args.iter().find(|s| s.to_lowercase() == "minify").is_some();
  let host = args.iter().find(|s| s.to_lowercase() == "host").is_some();
  let do_something = build || minify || host;
  if !do_something {
    println!("Please provide command line arguments to build, minify, or host");
    return;
  }
  if build {
    println!("=====\tBuilding...\t=====");
    match core::builder::build() {
      Ok(_) => println!("Ok"),
      Err(_) => println!("Err"),
    }
  }
  if minify {
    println!("=====\tMinifying...\t=====");
    match core::minifyer::minify() {
      Ok(_) => println!("Ok"),
      Err(_) => println!("Err"),
    }
  }
  if host {
    println!("=====\tHosting...\t=====");
    match core::hoster::host() {
      Ok(_) => println!("Ok"),
      Err(_) => println!("Err"),
    }
  }
}
```