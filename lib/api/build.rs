use std::{
    env::{self},
    path::PathBuf,
};

use bzd_lib::error::Error;

fn main() -> Result<(), Error> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("tokens_descriptor.bin"))
        .compile_protos(&["src/tokens.proto"], &["src"])?;

    tonic_prost_build::configure().compile_protos(&["src/notify.proto"], &["src"])?;

    Ok(())
}
