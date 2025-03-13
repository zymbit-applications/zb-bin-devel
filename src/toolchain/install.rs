use std::{fs::File, path::Path};

use anyhow::{bail, Context, Result};
use dialoguer::theme::ColorfulTheme;
use reqwest;

use crate::{
    system,
    terminal::{formatted_left_output, OutputColor},
};

use super::version;

/// Prompts user for version and installs tool to `/usr/bin/{tag_prefix}`
///
/// `tag_prefix`: `zbcli` in `zbcli-1.1.0`
pub async fn prompt(
    tag_prefix: &str,
    target_asset: &str,
    zb_version: &Option<String>,
) -> Result<()> {
    let releases = version::list(tag_prefix, zb_version, 10_u8).await?;

    let releases_list = releases
        .iter()
        .filter(|release| {
            release
                .assets
                .iter()
                .any(|asset| target_asset == asset.name)
        })
        .collect::<Vec<_>>();
    if releases_list.is_empty() && zb_version.is_some() {
        bail!(
            "asset '{target_asset}' does not exist within the '{}' release",
            zb_version.as_ref().unwrap()
        )
    }

    let releases_strings = releases_list
        .iter()
        .map(|release| &release.tag_name)
        .collect::<Vec<_>>();

    let target_release = match zb_version {
        Some(_) => releases_list[0],
        None => {
            let selection = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select version")
                .items(&releases_strings)
                .interact()
                .context("Failed to get version selection")?;

            releases_list[selection]
        }
    };

    let Some(asset) = target_release
        .assets
        .iter()
        .find(|asset| *target_asset == asset.name)
    else {
        bail!(
            "{} failed to find '{target_asset}' from latest release",
            formatted_left_output("Error", &OutputColor::Red)
        )
    };

    let asset_res = reqwest::get(asset.browser_download_url.clone()).await?;
    let mut content = std::io::Cursor::new(asset_res.bytes().await?);

    println!("Installing {tag_prefix}");

    let zb_path = Path::new("/usr").join("bin").join(tag_prefix);

    let mut zb_file = File::create(&zb_path)?;
    std::io::copy(&mut content, &mut zb_file)?;
    system::add_executable_permission(&zb_path)?;

    Ok(())
}
