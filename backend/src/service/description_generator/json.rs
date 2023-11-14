use tracing::info;

use super::{DescriptionGenerator, TemplateContext};
use crate::Result;
pub struct JsonDescriptionGenerator;

impl DescriptionGenerator for JsonDescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String> {
        info!("Generating JSON description");
        serde_json::to_string_pretty(&options).map_err(From::from)
    }
}
