use rocket::{catch, catchers, fs::NamedFile, routes, Request, Rocket};

pub struct BasePath(pub std::path::PathBuf);

#[catch(default)]
pub async fn catch_default(status: rocket::http::Status, req: &Request<'_>) -> Option<NamedFile> {
  let path = req.rocket().state::<BasePath>();
  match path {
    Some(path) => NamedFile::open(path.0.join(format!("{}.html", status.code))).await.ok(),
    None => None,
  }
}

#[rocket::get("/<path..>", rank = 0)]
pub async fn regular_file(
  path: std::path::PathBuf, base: &rocket::State<BasePath>,
) -> Option<NamedFile> {
  let total_path = base.0.join(&path);
  match total_path.extension() {
    Some(_) => NamedFile::open(total_path).await.ok(),
    None => NamedFile::open(total_path.with_extension("html")).await.ok(),
  }
}

pub fn host(output: &std::path::PathBuf) -> std::result::Result<(), ()> {
  let t = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      Rocket::build()
        .mount("/", rocket::fs::FileServer::from(output).rank(-15))
        .mount("/", routes![regular_file])
        .manage(BasePath { 0: output.clone() })
        .register("/", catchers![catch_default])
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
