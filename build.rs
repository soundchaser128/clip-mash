use clip_mash_types::Api;
use std::fs::File;
use typescript_type_def::{write_definition_file, DefinitionFileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("./frontend/src/types.generated.ts")?;
    let mut options = DefinitionFileOptions::default();
    options.root_namespace = None;
    write_definition_file::<_, Api>(&mut file, options)?;

    Ok(())
}
