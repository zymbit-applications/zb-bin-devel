use std::{fs::File, path::Path};

use anyhow::{bail, Result};
use reqwest;

use crate::{
    system,
    terminal::{formatted_left_output, OutputColor},
};

use super::version;

pub async fn latest(target_asset: ZbcliAsset) -> Result<()> {
    let latest_release = version::latest().await?;

    let Some(asset) = latest_release
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

#[derive(derive_more::Display, PartialEq)]
pub enum ZbcliAsset {
    #[display(fmt = "zbcli-rpi4")]
    Rpi4,

    #[display(fmt = "zbcli-rpi4-hardware")]
    Rpi4Hardware,

    #[display(fmt = "zbcli-rpi5")]
    Rpi5,

    #[display(fmt = "zbcli-rpi5-hardware")]
    Rpi5Hardware,
}
