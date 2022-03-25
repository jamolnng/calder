use glob::glob;
use html_minifier::HTMLMinifier;

pub fn minify(output: &std::path::PathBuf) -> std::result::Result<(), ()> {
  let files = glob(&format!("{}/**/*.css", output.display())).ok();
  if let Some(files) = files {
    for file in files {
      match file {
        Ok(file) => {
          let r = minify_css(&file);
          print!("Minifying: {}... ", file.to_string_lossy());
          match r {
            Ok(s) => {
              std::fs::write(file, s).unwrap();
              println!("ok");
            }
            Err(_) => {
              println!("error");
            }
          }
        }
        Err(e) => {
          println!("{e}");
        }
      }
    }
  }
  let files = glob(&format!("{}/**/*.js", output.display())).ok();
  if let Some(files) = files {
    for file in files {
      match file {
        Ok(file) => {
          let r = minify_js(&file);
          print!("Minifying: {}... ", file.to_string_lossy());
          match r {
            Ok(s) => {
              std::fs::write(file, s).unwrap();
              println!("ok");
            }
            Err(_) => {
              println!("error");
            }
          }
        }
        Err(e) => {
          println!("{e}");
        }
      }
    }
  }
  let files = glob(&format!("{}/**/*.html", output.display())).ok();
  if let Some(files) = files {
    for file in files {
      match file {
        Ok(file) => {
          let r = minify_html(&file);
          print!("Minifying: {}... ", file.to_string_lossy());
          match r {
            Ok(s) => {
              std::fs::write(file, s).unwrap();
              println!("ok");
            }
            Err(_) => {
              println!("error");
            }
          }
        }
        Err(e) => {
          println!("{e}");
        }
      }
    }
  }
  Ok(())
}

fn minify_css(file: &std::path::PathBuf) -> std::result::Result<String, ()> {
  if let Ok(s) = std::fs::read_to_string(file) {
    if let Ok(s) = minifier::css::minify(s.as_str()) {
      return Ok(s);
    }
  }
  Err(())
}

fn minify_js(file: &std::path::PathBuf) -> std::result::Result<String, ()> {
  if let Ok(s) = std::fs::read_to_string(file) {
    return Ok(minifier::js::minify(s.as_str()));
  }
  Err(())
}

fn minify_html(file: &std::path::PathBuf) -> std::result::Result<String, ()> {
  if let Ok(s) = std::fs::read_to_string(file) {
    let mut html_minifier = HTMLMinifier::new();
    html_minifier.set_remove_comments(true);
    if html_minifier.digest(s.as_str()).is_ok() {
      if let Ok(s) = std::str::from_utf8(html_minifier.get_html()) {
        return Ok(String::from(s));
      }
    }
  }
  Err(())
}
