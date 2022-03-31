use chrono::{NaiveDate, NaiveDateTime, Utc};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub enum PageErrorKind {
  Tera(tera::Error),
  Io(std::io::Error),
  ContentType(String),
}

#[derive(Debug)]
pub struct PageError {
  #[allow(dead_code)]
  kind: PageErrorKind,
}

impl From<tera::Error> for PageError {
  fn from(e: tera::Error) -> Self {
    Self { kind: PageErrorKind::Tera(e) }
  }
}

impl From<std::io::Error> for PageError {
  fn from(e: std::io::Error) -> Self {
    Self { kind: PageErrorKind::Io(e) }
  }
}

pub type Result<T> = std::result::Result<T, PageError>;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Tag(pub String);

impl Tag {
  pub fn from(tag: String) -> Self {
    Self { 0: Self::canonicalize(&tag) }
  }

  pub fn canonicalize(tag: &str) -> String {
    tag
      .trim()
      .to_lowercase()
      .split_whitespace()
      .collect::<Vec<&str>>()
      .join("-")
  }
}

impl std::fmt::Display for Tag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'de> Deserialize<'de> for Tag {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Deserialize::deserialize(deserializer).map(|s| Tag::from(s))
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PageInfo {
  pub title: String,
  pub date: NaiveDate,
  pub desc: String,
  #[serde(deserialize_with = "from_template")]
  pub template: String,
  pub img: Option<String>,
  pub tags: Vec<Tag>,
  #[serde(skip_deserializing)]
  pub build_date: NaiveDateTime,
  #[serde(skip_deserializing)]
  pub path: String,
}

impl Default for PageInfo {
  fn default() -> Self {
    Self {
      title: String::new(),
      date: Utc::now().naive_utc().date(),
      desc: String::new(),
      template: String::from("_templates/default.html"),
      build_date: Utc::now().naive_utc(),
      tags: Vec::new(),
      img: None,
      path: String::new(),
    }
  }
}

fn from_template<'de, D>(
  deserializer: D,
) -> std::result::Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let mut template: String = Deserialize::deserialize(deserializer)?;
  if !template.ends_with(".html") {
    template.push_str(".html");
  }
  Ok(template)
}

#[derive(Debug, Clone)]
pub enum ContentType {
  Markdown,
  Template,
  HTML,
}

#[derive(Debug)]
pub struct PageContent {
  pub kind: ContentType,
  pub content: String,
}

#[derive(Debug)]
pub struct Page {
  pub info: PageInfo,
  pub content: PageContent,
}

impl Page {
  pub fn has_tag(&self, tag: &Tag) -> bool {
    self.info.tags.contains(tag)
  }

  pub fn tags(&self) -> &Vec<Tag> {
    &self.info.tags
  }

  pub fn template(&self) -> &String {
    &self.info.template
  }

  pub fn render(
    &mut self, tera: &tera::Tera, mut context: &mut tera::Context,
  ) -> Result<()> {
    match self.content.kind {
      ContentType::Markdown => {
        self.content = Self::render_markdown(&self.content.content);
        self.content = self.render_template(&tera, &mut context)?;
        Ok(())
      }
      ContentType::Template => {
        self.content = self.render_template(&tera, &mut context)?;
        Ok(())
      }
      ContentType::HTML => Ok(()),
    }
  }

  pub fn render_markdown(str: &String) -> PageContent {
    let options = Options::all();
    let parser = Parser::new_ext(&str, options);
    let mut html = String::with_capacity(str.len() * 3 / 2);
    html::push_html(&mut html, parser);
    PageContent { kind: ContentType::Template, content: html }
  }

  fn render_template(
    &self, tera: &tera::Tera, context: &mut tera::Context,
  ) -> Result<PageContent> {
    context.insert("self", &self.info);
    context.insert("content", &self.content.content);
    Ok(
      tera
        .render(&self.info.template, &context)
        .map(|s| PageContent { kind: ContentType::HTML, content: s })?,
    )
  }

  pub fn write(&self, base: &PathBuf) -> Result<()> {
    let content = match self.content.kind {
      ContentType::HTML => Ok(&self.content.content),
      _ => Err(PageError {
        kind: PageErrorKind::ContentType(format!(
          "Content must be rendered to be written, not {:#?}",
          self.content.kind
        )),
      }),
    }?;
    let path = base.join(&self.info.path).with_extension("html");
    let parent = path.parent().unwrap();
    std::fs::create_dir_all(parent)?;
    std::fs::write(path, content)?;
    Ok(())
  }
}
