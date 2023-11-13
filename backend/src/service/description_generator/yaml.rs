use tracing::info;

use super::{DescriptionGenerator, TemplateContext};
use crate::Result;

pub struct YamlDescriptionGenerator;

impl DescriptionGenerator for YamlDescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String> {
        info!("Generating YAML description");
        serde_yaml::to_string(&options).map_err(From::from)
    }
}
