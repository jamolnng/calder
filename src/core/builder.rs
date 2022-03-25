use crate::core::page::{paginator, Paginator};
use std::path::PathBuf;
use tera::Tera;

pub fn build(input: &PathBuf, output: &PathBuf) -> paginator::Result<()> {
  let mut tera = match Tera::new(
    format!("{}/_templates/**/*.html", input.display()).as_str(),
  ) {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e);
      return Err(paginator::PaginatorError {});
    }
  };
  tera.autoescape_on(vec![]);

  let mut pages = Paginator::from(input);
  pages.render(&tera)?;
  pages.write(output)
}
