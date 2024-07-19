use std::{fs::File, path::Path};

use anyhow::{bail, Context, Result};
use dialoguer::theme::ColorfulTheme;
use reqwest;

use crate::{
    system,
    terminal::{formatted_left_output, OutputColor},
};

use super::version;

/// Prompts user for version and installs tool
///
/// `tag_prefix`: `zbcli` in `zbcli-1.1.0`
pub async fn prompt(tag_prefix: &str, target_asset: &str) -> Result<()> {
    let releases = version::list(tag_prefix).await?;

    let releases_list = releases
        .iter()
        .filter(|release| {
            release
                .assets
                .iter()
                .find(|asset| target_asset == asset.name)
                .is_some()
        })
        .collect::<Vec<_>>();

    let releases_strings = releases_list
        .iter()
        .map(|release| &release.tag_name)
        .collect::<Vec<_>>();

    let selection = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select version")
        .items(&releases_strings)
        .interact()
        .context("Failed to get version selection")?;

    let target_release = releases_list[selection];

    let Some(asset) = target_release
        .assets
        .iter()
        .find(|asset| target_asset.to_string() == asset.name)
    else {
        bail!(
            "{} failed to find '{target_asset}' from latest release",
            formatted_left_output("Error", &OutputColor::Red)
        )
    };

    let asset_res = reqwest::get(asset.browser_download_url.clone()).await?;
    let mut content = std::io::Cursor::new(asset_res.bytes().await?);

    let zb_path = Path::new("/usr").join("bin").join("zbcli");

    let mut zb_file = File::create(&zb_path)?;

    std::io::copy(&mut content, &mut zb_file)?;
    system::add_executable_permission(&zb_path)?;

    Ok(())
}
