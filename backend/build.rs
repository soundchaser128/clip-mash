use std::fs::File;
use std::process::{Command, Output};

use clip_mash_types::Api;
use typescript_type_def::{write_definition_file, DefinitionFileOptions};

type Error = Box<dyn std::error::Error>;

const TYPE_DEFINITIONS: &str = "../frontend/src/types.generated.ts";

pub fn commandline_error<T>(command_name: &str, output: Output) -> Result<T, Error> {
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    Err(format!(
        "command {} failed with exit code {}, stdout:\n'{}'\nstderr:\n'{}'",
        command_name,
        output.status.code().unwrap_or(1),
        stdout,
        stderr
    )
    .into())
}

fn format_file() -> Result<(), Error> {
    let output = Command::new("prettier")
        .args(["--write", TYPE_DEFINITIONS])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        commandline_error("prettier", output)
    }
}

fn main() -> Result<(), Error> {
    let mut file = File::create(TYPE_DEFINITIONS)?;
    let mut options = DefinitionFileOptions::default();
    options.root_namespace = None;
    write_definition_file::<_, Api>(&mut file, options)?;
    let _ = format_file();

    println!("cargo:rerun-if-changed=clip-mash-types");
    Ok(())
}
