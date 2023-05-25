use std::fs::File;
use typescript_type_def::{write_definition_file, DefinitionFileOptions};
use clip_mash_types::Api;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("./frontend/src/types.generated.ts")?;
    let options = DefinitionFileOptions::default();
    write_definition_file::<_, Api>(&mut file, options)?;

    Ok(())
}
