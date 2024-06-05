use clip_mash_client::apis::{configuration::Configuration, library_api::list_videos};
use color_eyre::Result;
use comfy_table::Table;

pub struct ListVideoOptions {
    pub url: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub query: Option<String>,
}

pub async fn list(options: ListVideoOptions) -> Result<()> {
    let mut configuration = Configuration::new();
    configuration.base_path = options
        .url
        .unwrap_or_else(|| "http://localhost:5174".to_string());

    let response = list_videos(
        &configuration,
        options.query.as_deref(),
        None,
        None,
        None,
        options.page,
        options.size,
        None,
        None,
    )
    .await?;

    let mut table = Table::new();
    table.set_header(vec!["ID", "Title", "Tags", "Source"]);
    for video in response.content {
        let video = video.video;
        table.add_row(vec![
            video.id.to_string(),
            video.title.clone(),
            video.tags.join(", "),
            video.source.to_string(),
        ]);
    }

    println!("{}", table);

    Ok(())
}

pub async fn upload() -> Result<()> {
    println!("Uploading video");
    Ok(())
}
