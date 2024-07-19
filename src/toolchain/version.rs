use anyhow::{Context, Result};
use octocrab::models::repos::Release;

/// `tag_prefix`: `zbcli` in `zbcli-1.1.0`
pub async fn list(tag_prefix: &str) -> Result<Vec<Release>> {
    let github_instance = octocrab::instance();
    let repo = github_instance.repos("zymbit-applications", "zb-bin");

    let releases = repo
        .releases()
        .list()
        .per_page(100)
        .send()
        .await
        .context("Failed to get latest release")?;

    Ok(releases
        .into_iter()
        .filter(|release| release.tag_name.starts_with(&format!("{tag_prefix}-")))
        .collect::<Vec<_>>())
}
