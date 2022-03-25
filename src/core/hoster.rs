use rocket::Rocket;

pub fn host(output: &std::path::PathBuf) -> std::result::Result<(), ()> {
  let t = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      Rocket::build()
        .mount("/", rocket::fs::FileServer::from(output))
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
