#![allow(dead_code)]

use glob::glob;
use std::path::PathBuf;

use crate::core::page::Page;

#[derive(Debug, Clone)]
pub struct PaginatorError;

pub type Result<T> = std::result::Result<T, PaginatorError>;

#[derive(Debug)]
pub struct Paginator {
  base: PathBuf,
  pages: Vec<Page>,
}

impl Paginator {
  pub fn from(path: &PathBuf) -> Self {
    Self {
      base: path.clone(),
      pages: match Self::get_pages(path) {
        Ok(p) => p,
        Err(_) => Vec::new(),
      },
    }
  }

  pub fn pages(&self) -> &Vec<Page> {
    &self.pages
  }

  pub fn with_tag(&self, tag: &str) -> Vec<&Page> {
    Self::with_tag_from(&self.pages, tag)
  }

  pub fn with_tag_from<'a>(pages: &'a Vec<Page>, tag: &str) -> Vec<&'a Page> {
    let mut r = Vec::new();
    for page in pages {
      if page.tags.contains(&tag.to_string()) {
        r.push(page);
      }
    }
    r
  }

  pub fn get_type(&self, t: &str) -> Vec<&Page> {
    Self::get_type_from(&self.pages, t)
  }

  pub fn get_type_from<'a>(
    pages: &'a Vec<Page>, template: &str,
  ) -> Vec<&'a Page> {
    let mut r = Vec::new();
    for page in pages {
      if page.template == template {
        r.push(page);
      }
    }
    r
  }

  pub fn render(&mut self, tera: &tera::Tera) -> Result<()> {
    let pages = self.pages.clone();
    for page in &mut self.pages {
      match page.render(&pages, tera) {
        Ok(_) => {}
        Err(_) => {}
      }
    }
    Ok(())
  }

  pub fn write(&self, path: &PathBuf) -> Result<()> {
    for page in &self.pages {
      match page.write(path) {
        Ok(_) => {}
        Err(_) => {}
      }
    }
    Ok(())
  }

  fn get_pages(path: &PathBuf) -> Result<Vec<Page>> {
    let mut r = Vec::new();
    if let Some(files) = Self::get_markdown_files(path) {
      for file in files {
        if let Ok(file) = file {
          if file.is_file() {
            if let Ok(lines) = Self::read_lines(&file) {
              let mut first = false;
              let mut second = false;
              let mut page_str = String::new();
              let mut data = String::new();
              for line in lines {
                if let Ok(line) = line {
                  if first && second {
                    data.push_str(&format!("{line}\n"));
                  } else if line == "---" {
                    if !first {
                      first = true;
                    } else {
                      second = true;
                    }
                  } else if line.is_empty() && !first {
                    break; // TODO: error
                  } else {
                    page_str.push_str(&format!("{line}\n"));
                  }
                }
              }
              let file = PathBuf::from(file.strip_prefix(path).unwrap());
              if first && second {
                let page = serde_yaml::from_str::<Page>(&page_str);
                if let Ok(mut page) = page {
                  page.path = file;
                  page.data = data;
                  r.push(page);
                } else {
                  println!("{}, {:#?}", file.display(), page_str);
                  // TODO: error
                }
              } else if !first && !second {
                let title = Self::pretty_unknown_title(&file);
                r.push(Page {
                  title: title,
                  path: file,
                  data: data,
                  ..Page::default()
                })
              }
            } else {
              // TODO: error
            }
          }
        } else {
          // TODO: error
        }
      }
    } else {
      // TODO: error
    }
    Ok(r)
  }

  fn pretty_unknown_title(file: &PathBuf) -> String {
    let key = file
      .file_name()
      .unwrap()
      .to_string_lossy()
      .to_string()
      .replace(".md", "");
    let mut v = key
      .replace('_', " ")
      .split(' ')
      .map(|s| {
        let mut c = s.chars();
        match c.next() {
          Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
          None => String::new(),
        }
      })
      .collect::<Vec<String>>();
    v.retain(|s| !s.is_empty());
    v.join(" ")
  }

  fn get_markdown_files(path: &PathBuf) -> Option<glob::Paths> {
    glob(format!("{}/**/*.md", path.display()).as_str()).ok()
  }

  fn read_lines(
    filename: &PathBuf,
  ) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
  }
}
