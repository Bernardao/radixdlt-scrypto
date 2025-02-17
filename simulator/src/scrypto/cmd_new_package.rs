use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use std::fs;
use std::path::*;

use crate::scrypto::*;

const ARG_NAME: &str = "NAME";
const ARG_PATH: &str = "PATH";

/// Constructs a `new-package` subcommand.
pub fn make_new_package<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_NEW_PACKAGE)
        .about("Creates a package")
        .version(crate_version!())
        .arg(
            Arg::with_name(ARG_NAME)
                .help("Specifies the package name.")
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_PATH)
                .long("path")
                .takes_value(true)
                .help("Specifies the package dir.")
                .required(false),
        )
}

/// Handles a `new-package` request.
pub fn handle_new_package(matches: &ArgMatches) -> Result<(), Error> {
    let pkg_name = matches
        .value_of(ARG_NAME)
        .ok_or_else(|| Error::MissingArgument(ARG_NAME.to_owned()))?;
    let pkg_dir = matches.value_of(ARG_PATH).unwrap_or(pkg_name);

    let simulator_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scrypto_dir = simulator_dir.parent().unwrap().to_string_lossy();

    if PathBuf::from(pkg_dir).exists() {
        Err(Error::PackageAlreadyExists)
    } else {
        fs::create_dir_all(format!("{}/src", pkg_dir)).map_err(Error::IOError)?;
        fs::create_dir_all(format!("{}/tests", pkg_dir)).map_err(Error::IOError)?;

        fs::write(
            PathBuf::from(format!("{}/Cargo.toml", pkg_dir)),
            include_str!("../../../assets/template/Cargo.toml")
                .replace("${package_name}", pkg_name)
                .replace("${scrypto_home}", &scrypto_dir.replace("\\", "/")),
        )
        .map_err(Error::IOError)?;

        fs::write(
            PathBuf::from(format!("{}/src/lib.rs", pkg_dir)),
            include_str!("../../../assets/template/src/lib.rs"),
        )
        .map_err(Error::IOError)?;

        fs::write(
            PathBuf::from(format!("{}/tests/lib.rs", pkg_dir)),
            include_str!("../../../assets/template/tests/lib.rs"),
        )
        .map_err(Error::IOError)?;

        Ok(())
    }
}
