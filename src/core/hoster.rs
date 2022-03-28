use rocket::{catch, catchers, fs::NamedFile, Request, Rocket};

struct BasePath(std::path::PathBuf);

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> Option<NamedFile> {
  let path = req.rocket().state::<BasePath>();
  match path {
    Some(path) => NamedFile::open(path.0.join("404.html")).await.ok(),
    None => None,
  }
}

pub fn host(output: &std::path::PathBuf) -> std::result::Result<(), ()> {
  let t = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      Rocket::build()
        .mount("/", rocket::fs::FileServer::from(output))
        .manage(BasePath { 0: output.clone() })
        .register("/", catchers![not_found])
        .ignite()
        .await?
        .launch()
        .await
    });
  match t {
    Ok(_) => Ok(()),
    Err(_) => Err(()),
  }
}
