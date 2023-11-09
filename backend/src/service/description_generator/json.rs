use super::{DescriptionGenerator, TemplateContext};
use crate::Result;

pub struct JsonDescriptionGenerator;

impl DescriptionGenerator for JsonDescriptionGenerator {
    fn generate(&self, options: TemplateContext) -> Result<String> {
        serde_json::to_string(&options).map_err(From::from)
    }
}
