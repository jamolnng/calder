mod core;

use clap::{arg, command};

fn main() {
  let matches = command!()
    .arg(arg!([path] "path to generate site from").required(true))
    .arg(arg!(-b --build "flag to generate site").required(false))
    .arg(arg!(-m --minify "flag to minify the sites html, css, and js code").required(false))
    .arg(arg!(-r --host "flag to host via a webserver when done").required(false))
    .get_matches();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    println!(
      "Need to specify the path ex: cargo run -- example_dir --build --minify --host"
    );
  }
  let path = std::path::PathBuf::from(matches.value_of("path").unwrap());
  let build = matches.is_present("build");
  let minify = matches.is_present("minify");
  let host = matches.is_present("host");
  let do_something = build || minify || host;
  if !do_something {
    println!(
      "Please provide command line arguments to build, minify, or host"
    );
    return;
  }
  if build {
    println!("=====\tBuilding...\t=====");
    match core::builder::build(&path) {
      Ok(_) => println!("=====\tOk...\t\t====="),
      Err(_) => println!("=====\tErr...\t\t====="),
    }
  }
  // println!("=====\tCopying...\t=====");
  // match core::copy::copy(&path) {
  //   Ok(_) => println!("=====\tOk...\t\t====="),
  //   Err(_) => println!("=====\tErr...\t\t====="),
  // }
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
