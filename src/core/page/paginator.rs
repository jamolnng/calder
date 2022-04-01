#![allow(dead_code)]

use glob::glob;
use std::collections::BTreeSet;
use std::path::PathBuf;

use tera::{to_value, Tera, Value};

use super::page::{
  ContentType, Page, PageContent, PageError, PageInfo, Tag, TagPage,
};

#[derive(Debug)]
pub enum PaginatorErrorKind {
  Tera(tera::Error),
  Page(PageError),
  Glob(glob::GlobError),
  GlobPattern(glob::PatternError),
  Io(std::io::Error),
  MissingHeader(String),
  StripPrefix(std::path::StripPrefixError),
  YAML(serde_yaml::Error),
}

#[derive(Debug)]
pub struct PaginatorError {
  kind: PaginatorErrorKind,
}

impl From<tera::Error> for PaginatorError {
  fn from(e: tera::Error) -> Self {
    Self { kind: PaginatorErrorKind::Tera(e) }
  }
}

impl From<PageError> for PaginatorError {
  fn from(e: PageError) -> Self {
    Self { kind: PaginatorErrorKind::Page(e) }
  }
}

impl From<glob::GlobError> for PaginatorError {
  fn from(e: glob::GlobError) -> Self {
    Self { kind: PaginatorErrorKind::Glob(e) }
  }
}

impl From<glob::PatternError> for PaginatorError {
  fn from(e: glob::PatternError) -> Self {
    Self { kind: PaginatorErrorKind::GlobPattern(e) }
  }
}

impl From<std::io::Error> for PaginatorError {
  fn from(e: std::io::Error) -> Self {
    Self { kind: PaginatorErrorKind::Io(e) }
  }
}

impl From<std::path::StripPrefixError> for PaginatorError {
  fn from(e: std::path::StripPrefixError) -> Self {
    Self { kind: PaginatorErrorKind::StripPrefix(e) }
  }
}

impl From<serde_yaml::Error> for PaginatorError {
  fn from(e: serde_yaml::Error) -> Self {
    Self { kind: PaginatorErrorKind::YAML(e) }
  }
}

pub type Result<T> = std::result::Result<T, PaginatorError>;

#[derive(serde::Serialize)]
pub struct Site<'a> {
  posts: &'a Vec<PageInfo>,
  tags: &'a BTreeSet<Tag>,
}

#[derive(Debug)]
pub struct Paginator {
  base: PathBuf,
  pages: Vec<Page>,
  tag_pages: Vec<TagPage>,
  tags: BTreeSet<Tag>,
  tera: tera::Tera,
}

impl Paginator {
  pub fn from(path: &PathBuf) -> Result<Self> {
    let mut tera =
      Tera::new(format!("{}/**/*.html", path.display()).as_str())?;
    tera.autoescape_on(vec![]);
    tera.register_filter("markdown", Self::tera_markdown);

    let pages = Self::get_pages(path)?;
    let tags = Self::get_tags(&pages);
    let tag_pages = Self::get_tag_pages(
      pages.iter().map(|p| p.info.clone()).collect(),
      &tags,
    );
    Ok(Self { base: path.clone(), pages, tag_pages, tags, tera })
  }

  pub fn render(&mut self) -> super::page::Result<Vec<()>> {
    let site = Site {
      posts: &self.with_template(&String::from("_templates/post.html")),
      tags: &self.tags,
    };
    let mut context = tera::Context::new();
    context.insert("site", &site);
    self
      .pages
      .iter_mut()
      .map(|page| {
        print!("Building: {}... ", page.path());
        let r = page.render(&self.tera, &mut context);
        match r {
          Ok(_) => println!("ok"),
          Err(_) => println!("err"),
        }
        r
      })
      .collect::<super::page::Result<Vec<()>>>()?;

    let mut context = tera::Context::new();
    let ebtree = BTreeSet::new();
    self
      .tag_pages
      .iter_mut()
      .map(|tp| {
        let site = Site { posts: &tp.tagged(), tags: &ebtree };
        context.insert("site", &site);
        print!("Building: {}... ", tp.path());
        let r = tp.render(&self.tera, &mut context);
        match r {
          Ok(_) => println!("ok"),
          Err(_) => println!("err"),
        }
        r
      })
      .collect::<super::page::Result<Vec<()>>>()
  }

  pub fn write(&self, base: &PathBuf) -> super::page::Result<Vec<()>> {
    self
      .pages
      .iter()
      .map(|page| {
        print!(
          "Writing: {}... ",
          base.join(&page.path()).with_extension("html").display()
        );
        let r = page.write(base);
        match r {
          Ok(_) => println!("ok"),
          Err(_) => println!("err"),
        }
        r
      })
      .collect::<super::page::Result<Vec<()>>>()?;

    self
      .tag_pages
      .iter()
      .map(|tp| {
        print!(
          "Writing: {}... ",
          base.join(&tp.path()).with_extension("html").display()
        );
        let r = tp.write(base);
        match r {
          Ok(_) => println!("ok"),
          Err(_) => println!("err"),
        }
        r
      })
      .collect()
  }

  pub fn pages(&self) -> &Vec<Page> {
    &self.pages
  }

  pub fn with_tag(&self, tag: &Tag) -> Vec<&PageInfo> {
    self.pages.iter().filter(|p| p.has_tag(tag)).map(|p| &p.info).collect()
  }

  pub fn with_template(&self, template: &String) -> Vec<PageInfo> {
    self
      .pages
      .iter()
      .filter(|p| p.template() == template)
      .map(|p| &p.info)
      .cloned()
      .collect()
  }

  fn get_tags(pages: &Vec<Page>) -> BTreeSet<Tag> {
    pages.iter().flat_map(|p| p.tags().clone()).collect()
  }

  fn get_tag_pages<'b>(
    pages: Vec<PageInfo>, tags: &BTreeSet<Tag>,
  ) -> Vec<TagPage> {
    tags
      .iter()
      .map(|tag| {
        let info = super::page::PageInfo {
          title: tag.0.clone(),
          desc: tag.0.clone(),
          template: String::from("_templates/tag.html"),
          path: format!("tags/{tag}.html"),
          ..Default::default()
        };
        let content = super::page::PageContent {
          kind: super::page::ContentType::Template,
          content: String::new(),
        };
        let ps =
          pages.iter().filter(|p| p.tags.contains(tag)).cloned().collect();
        TagPage { tag: tag.clone(), page: Page { info, content }, tagged: ps }
      })
      .collect()
  }

  fn get_pages(base: &PathBuf) -> Result<Vec<Page>> {
    let mut r = Vec::new();
    for file in Self::get_markdown_files(base)? {
      let file = file?;
      if file.is_file() {
        r.push(Self::process_md_file(&base, &file)?);
      }
    }
    let files = Self::get_html_files(base)?;
    r.reserve(files.len());
    for file in files {
      let file = file?;
      if file.is_file() {
        r.push(Self::process_html_file(&base, &file)?);
      }
    }
    Ok(r)
  }

  fn process_md_file(base: &PathBuf, file: &PathBuf) -> Result<Page> {
    let infile = std::fs::File::open(file)?;
    let reader = std::io::BufReader::new(&infile);
    let lines = Self::full_lines(reader);
    let mut first = false;
    let mut second = false;
    let mut header = String::new();
    let mut content = String::with_capacity(infile.metadata()?.len() as usize);
    for line in lines {
      let line = line?;
      if first && second {
        content.push_str(&line);
      } else {
        let lt = line.trim();
        if lt.starts_with("---") {
          if first {
            second = true;
          } else {
            first = true;
          }
        } else if !lt.is_empty() && !first {
          return Err(PaginatorError {
            kind: PaginatorErrorKind::MissingHeader(format!(
              "Missing YAML header from file: {}",
              file.display()
            )),
          });
        } else {
          header.push_str(&line);
        }
      }
    }
    let path = file.strip_prefix(base)?.with_extension("html");
    if first && second {
      let mut info = serde_yaml::from_str::<PageInfo>(&header)?;
      info.path = path.to_string_lossy().replace("\\", "/");
      let content = PageContent { kind: ContentType::Markdown, content };
      return Ok(Page { info, content });
    } else if !first && !second {
      let title = Self::pretty_unknown_title(&path);
      let info = PageInfo {
        title,
        path: path.to_string_lossy().replace("\\", "/"),
        ..Default::default()
      };
      let content = PageContent { kind: ContentType::Markdown, content };
      return Ok(Page { info, content });
    } else {
      return Err(PaginatorError {
        kind: PaginatorErrorKind::MissingHeader(format!(
          "Missing YAML header from file: {}",
          file.display()
        )),
      });
    }
  }

  fn process_html_file(base: &PathBuf, file: &PathBuf) -> Result<Page> {
    let s = std::fs::read_to_string(&file)?;
    let file = file.strip_prefix(base)?.to_string_lossy().replace("\\", "/");
    Ok(Page {
      info: PageInfo {
        title: Self::pretty_unknown_title(&file),
        template: file.clone(),
        path: file,
        ..Default::default()
      },
      content: PageContent { kind: ContentType::Template, content: s },
    })
  }

  fn pretty_unknown_title<P: AsRef<std::path::Path>>(file: P) -> String {
    let key = file
      .as_ref()
      .file_name()
      .unwrap()
      .to_string_lossy()
      .to_string()
      .replace(".md", "")
      .replace(".html", "");
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

  fn get_html_files(path: &PathBuf) -> Result<Vec<glob::GlobResult>> {
    let g1 = glob(format!("{}/[!_]*/**/*.html", path.display()).as_str())?;
    let g2 = glob(format!("{}/*.html", path.display()).as_str())?;
    Ok(g1.chain(g2).collect())
  }

  fn get_markdown_files(
    path: &PathBuf,
  ) -> std::result::Result<glob::Paths, glob::PatternError> {
    glob(format!("{}/**/*.md", path.display()).as_str())
  }

  fn full_lines(
    mut input: impl std::io::BufRead,
  ) -> impl Iterator<Item = std::io::Result<String>> {
    std::iter::from_fn(move || {
      let mut vec = String::new();
      match input.read_line(&mut vec) {
        Ok(0) => None,
        Ok(_) => Some(Ok(vec)),
        Err(e) => Some(Err(e)),
      }
    })
  }

  fn read_lines(
    filename: &PathBuf,
  ) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
  }

  fn tera_markdown(
    value: &Value, _: &std::collections::HashMap<String, Value>,
  ) -> tera::Result<Value> {
    let data = tera::try_get_value!("markdown", "value", String, value);
    Ok(to_value(Page::render_markdown(&data).content).unwrap())
  }
}
