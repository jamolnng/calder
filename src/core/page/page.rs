use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Page {
  pub title: String,
  pub date: String,
  pub desc: String,
  #[serde(rename = "type")] 
  pub template: String,
  pub tags: Vec<String>,
  #[serde(skip_deserializing)]
  pub build_date: String,
  #[serde(skip_serializing, skip_deserializing)]
  pub path: PathBuf,
  #[serde(skip_serializing, skip_deserializing)]
  pub html: String,
}

#[derive(Debug, Clone)]
pub struct PageError;

pub type Result<T> = std::result::Result<T, PageError>;

impl Page {
  fn render(&self) -> Result<()> {
    Ok(())
  }

  fn write(&self, path: &PathBuf) -> Result<()> {
    Ok(())
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
      path: PathBuf::new(),
      html: String::new(),
    }
  }
}
