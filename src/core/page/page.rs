use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Page {
  pub title: String,
  pub date: String,
  pub desc: String,
  pub template: String,
  pub tags: Vec<String>,
  #[serde(skip_deserializing)]
  pub build_date: String,
  #[serde(skip_deserializing)]
  pub path: String,
  #[serde(skip_deserializing)]
  pub data: String,
}

#[derive(Debug, Clone)]
pub struct PageError;

pub type Result<T> = std::result::Result<T, PageError>;

impl Page {
  pub fn has_tag(&self, tag: &str) -> bool {
    self.tags.contains(&tag.to_string())
  }

  pub fn template(&self) -> &String {
    &self.template
  }

  pub fn set_path(&mut self, path: &PathBuf) {
    self.path = path.to_string_lossy().replace("\\", "/");
  }

  pub fn set_data(&mut self, data: String) {
    self.data = data;
  }

  pub fn render(
    &mut self, pages: &Vec<Self>, tera: &tera::Tera,
  ) -> Result<()> {
    self.render_tera(pages, tera)?;
    Ok(())
  }

  pub fn write(&self, path: &PathBuf) -> Result<()> {
    let path = path.join(&self.path).with_extension("html");
    print!("{}", path.display());
    let parent = match path.parent() {
      Some(p) => Ok(p),
      None => Err(PageError {}),
    }?;
    match std::fs::create_dir_all(parent) {
      Ok(_) => Ok(()),
      Err(_) => Err(PageError {}),
    }?;
    match std::fs::write(path, &self.data) {
      Ok(_) => {
        println!(" ok");
        Ok(())
      }
      Err(e) => {
        println!("{e}");
        Err(PageError {})
      }
    }
  }

  fn render_tera(
    &mut self, pages: &Vec<Self>, tera: &tera::Tera,
  ) -> Result<()> {
    let mut context = tera::Context::new();
    context.insert("pages", &pages);
    context.insert(
      "posts",
      &crate::core::page::Paginator::get_type_from(pages, "_templates/post"),
    );
    context.insert("url", &"http://127.0.0.1:8000");
    context.insert("self", &self);

    let mut template = self.template.clone();
    if !self.template.ends_with(".html") {
      template = format!("{template}.html");
    }
    if tera.get_template_names().find(|s| *s == template).is_none() {
      template = "_templates/default.html".to_string();
    }

    let result = tera.render(&template, &context);
    match result {
      Ok(s) => {
        self.data = s;
        Ok(())
      }
      Err(e) => {
        println!("{e}");
        Err(PageError {})},
    }
  }
}

impl Default for Page {
  fn default() -> Self {
    Self {
      title: String::new(),
      date: String::new(),
      desc: String::new(),
      template: String::new(),
      build_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
      tags: Vec::new(),
      path: String::new(),
      data: String::new(),
    }
  }
}
