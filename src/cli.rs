use clap::{crate_authors, crate_name, crate_version, crate_description, Arg, Command};
use std::path::PathBuf;
use std::error::Error;
use std::fs::canonicalize;

pub fn get_cli_options() -> Result<PathBuf, Box<dyn Error>> {
    let opt = Command::new(crate_name!())
                .author(crate_authors!("\n"))
                .version(crate_version!())
                .about(crate_description!())
                .arg(
                    Arg::new("bin_file")
                        .short('f')
                        .long("bin-file")
                        .help("Binary file to read instructions from")
                        .required(true)
                )
                .get_matches();

    let bin_file: String = match opt.get_one::<String>("bin_file") {
        Some(v) => v.to_owned().into(),
        None => {
            return Err("No binary instruction file was supplied".into());
        }
    };
    
    let _bin_file: PathBuf = PathBuf::from(bin_file);
    let f_path: PathBuf = match canonicalize(&_bin_file){
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    };
    Ok(f_path)
}
