#![allow(dead_code)]

use crate::core::page::Paginator;

use pulldown_cmark::{html, Options, Parser};
use std::path::PathBuf;
use tera::Tera;

pub fn build(base: &PathBuf) -> std::result::Result<(), ()> {
  println!("{:#?}", Paginator::from(base).get_type("post"));
  return Ok(());
  // let mut tera = match Tera::new(
  //   format!("{}/_templates/**/*.html", base.display()).as_str(),
  // ) {
  //   Ok(t) => t,
  //   Err(e) => {
  //     println!("Parsing error(s): {}", e);
  //     return Err(());
  //   }
  // };
  // tera.autoescape_on(vec![]);
  // let options = Options::all();
  // let files = glob(format!("{}/**/*.md", base.display()).as_str()).ok();
  // if let Some(files) = files {
  //   for file in files {
  //     match file {
  //       Ok(file) => {
  //         if file.is_file() {
  //           let processed = process_file(&tera, &file, &options);
  //           match processed {
  //             Ok(s) => write_processed(base, &file, &s),
  //             Err(_) => {}
  //           }
  //         }
  //       }
  //       Err(e) => println!("{e}"),
  //     }
  //   }
  // }
  // Ok(())
}

fn render_markdown(file: &PathBuf, options: &Options) -> Result<String, ()> {
  if let Ok(s) = std::fs::read_to_string(file) {
    let parser = Parser::new_ext(s.as_str(), *options);
    let mut html = String::with_capacity(s.len());
    html::push_html(&mut html, parser);
    return Ok(html);
  }
  Err(())
}

fn write_processed(base: &PathBuf, file: &PathBuf, processed: &String) {
  let mut outfile = PathBuf::from(file.strip_prefix(base).unwrap());
  outfile = PathBuf::from("output/").join(outfile).with_extension("html");
  print!("{} ", outfile.to_string_lossy());
  std::fs::create_dir_all(outfile.parent().unwrap()).unwrap();
  std::fs::write(outfile, processed).unwrap();
  println!("Ok");
}

fn process_file(
  tera: &Tera, file: &PathBuf, options: &Options,
) -> Result<String, ()> {
  print!("Building: {}... ", file.display());
  let dir = file.parent().unwrap().file_name().unwrap();

  let html = render_markdown(file, options).unwrap();
  let mut context = tera::Context::new();
  context.insert("rendered", &html.as_str());

  let mut template_name = format!("{}.html", dir.to_string_lossy())
    .trim_start_matches('_')
    .to_string();

  if tera.get_template_names().find(|s| *s == template_name).is_none() {
    template_name = "default.html".to_string();
  }

  let result = tera.render(&template_name, &context);
  match result {
    Ok(s) => Ok(s),
    Err(e) => {
      println!("{e}");
      Err(())
    }
  }
}
