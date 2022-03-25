use glob::glob;
use std::path::PathBuf;

pub fn copy(input: &PathBuf, output: &PathBuf) -> Result<(), glob::PatternError> {
  let files = glob(format!("{}/[!_]*/**/*[!.md]", input.display()).as_str())?
    .chain(glob(format!("{}/*[!.md]", input.display()).as_str())?);
  for file in files {
    match file {
      Ok(file) => {
        if file.is_file() {
          print!("Copying: {}... ", file.to_string_lossy());
          let outfile = file.strip_prefix(input).unwrap();
          let outfile = output.join(outfile);
          std::fs::create_dir_all(outfile.parent().unwrap()).unwrap();
          match std::fs::copy(file, outfile) {
            Ok(_) => {
              println!("ok");
            }
            Err(_) => {
              println!("error");
            }
          }
        }
      }
      Err(e) => {
        println!("{e}");
      }
    }
  }
  Ok(())
}
