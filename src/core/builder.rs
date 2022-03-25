use crate::core::page::{paginator, Paginator};
use std::path::PathBuf;
use tera::Tera;

pub fn build(base: &PathBuf) -> paginator::Result<()> {
  let mut tera = match Tera::new(
    format!("{}/_templates/**/*.html", base.display()).as_str(),
  ) {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e);
      return Err(paginator::PaginatorError {});
    }
  };
  tera.autoescape_on(vec![]);

  let mut pages = Paginator::from(base);
  pages.render(&tera)?;
  pages.write(&PathBuf::from("output/"))
}
