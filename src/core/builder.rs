use tera::Tera;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let tera = match Tera::new("site/templates/**/*") {
      Ok(t) => t,
      Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
      }
    };
    tera
  };
}

pub fn build() -> std::result::Result<(), ()> {
  Ok(())
}
