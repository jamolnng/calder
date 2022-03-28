use crate::core::page::{paginator, Paginator};
use std::path::PathBuf;

use pulldown_cmark::{html, Options, Parser};
use tera::{to_value, Tera, Value};

pub fn tera_markdown(
  value: &Value, _: &std::collections::HashMap<String, Value>,
) -> tera::Result<Value> {
  let data = tera::try_get_value!("markdown", "value", String, value);

  let options = Options::all();
  let parser = Parser::new_ext(&data, options);
  let mut html = String::with_capacity(data.len());
  html::push_html(&mut html, parser);
  Ok(to_value(html).unwrap())
}

pub fn build(input: &PathBuf, output: &PathBuf) -> paginator::Result<()> {
  let mut tera =
    match Tera::new(format!("{}/**/*.html", input.display()).as_str()) {
      Ok(t) => t,
      Err(e) => {
        println!("Parsing error(s): {}", e);
        return Err(paginator::PaginatorError {});
      }
    };
  tera.autoescape_on(vec![]);
  tera.register_filter("markdown", tera_markdown);

  let mut pages = Paginator::from(input);
  pages.render(&tera)?;
  pages.write(output)
}
