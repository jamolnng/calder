use glob::glob;

pub fn minify() -> std::result::Result<(), ()> {
  let files = glob("output/**/*").ok();
  if let Some(files) = files {
    for file in files {
      match file {
        Ok(file) => println!("{}", file.to_string_lossy()),
        Err(e) => println!("{e}")
      }
    }
  }
  Ok(())
}
