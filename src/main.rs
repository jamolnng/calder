#[macro_use]
extern crate lazy_static;

mod core;

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
      Ok(_) => println!("=====\tOk...\t\t====="),
      Err(_) => println!("=====\tErr...\t\t====="),
    }
  }
  println!("=====\tCopying...\t=====");
  match core::copy::copy() {
    Ok(_) => println!("=====\tOk...\t\t====="),
    Err(_) => println!("=====\tErr...\t\t====="),
  }
  if minify {
    println!("=====\tMinifying...\t=====");
    match core::minifyer::minify() {
      Ok(_) => println!("=====\tOk...\t\t====="),
      Err(_) => println!("=====\tErr...\t\t====="),
    }
  }
  if host {
    println!("=====\tHosting...\t=====");
    match core::hoster::host() {
      Ok(_) => println!("=====\tOk...\t\t====="),
      Err(_) => println!("=====\tErr...\t\t====="),
    }
  }
}
