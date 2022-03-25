use glob::glob;
use tera::Tera;

use pulldown_cmark::{html, Options, Parser};

use std::path::PathBuf;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let mut tera = match Tera::new("site/_templates/**/*") {
      Ok(t) => t,
      Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
      }
    };
    tera.autoescape_on(vec![]);
    tera
  };
}

pub fn build() -> std::result::Result<(), ()> {
  let options = Options::all();
  let files = glob("site/**/*.md").ok();
  if let Some(files) = files {
    for file in files {
      match file {
        Ok(file) => {
          if file.is_file() {
            let processed = process_file(&file, &options);
            match processed {
              Ok(s) => write_processed(&file, &s),
              Err(_) => {}
            }
          }
        }
        Err(e) => println!("{e}"),
      }
    }
  }
  Ok(())
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

fn write_processed(file: &PathBuf, processed: &String) {
  let mut outfile = PathBuf::from(file.strip_prefix("site/").unwrap());
  let outfile_string = outfile.to_string_lossy();
  if outfile_string.starts_with("_") {
    let start = outfile.iter().next().unwrap().to_string_lossy();
    let t = start.trim_start_matches('_');
    let pb = PathBuf::from(outfile.strip_prefix(start.to_string()).unwrap());
    outfile = PathBuf::from(t).join(pb);
  }
  let outfile = PathBuf::from("output/").join(outfile).with_extension("html");
  print!("{} ", outfile.to_string_lossy());
  std::fs::create_dir_all(outfile.parent().unwrap()).unwrap();
  std::fs::write(outfile, processed).unwrap();
  println!("Ok");
}

fn process_file(file: &PathBuf, options: &Options) -> Result<String, ()> {
  print!("Building: {}... ", file.to_string_lossy());
  let dir = file.parent().unwrap().file_name().unwrap();

  let html = render_markdown(file, options).unwrap();
  let mut context = tera::Context::new();
  context.insert("rendered", &html.as_str());

  let mut template_name = format!("{}.html", dir.to_string_lossy()).trim_start_matches('_').to_string();

  if TEMPLATES.get_template_names().find(|s| *s == template_name).is_none() {
    template_name = "default.html".to_string();
  }

  let result = TEMPLATES.render(&template_name, &context);
  match result {
    Ok(s) => Ok(s),
    Err(_) => Err(()),
  }
}
