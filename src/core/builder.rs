use crate::core::page::{paginator, Paginator};
use std::path::PathBuf;

pub fn build(input: &PathBuf, output: &PathBuf) -> paginator::Result<()> {
  let mut pages = Paginator::from(input)?;
  pages.render()?;
  pages.write(output)?;
  Ok(())
}
