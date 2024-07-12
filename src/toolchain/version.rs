use anyhow::{Context, Result};
use octocrab::models::repos::Release;

pub async fn latest() -> Result<Release> {
    let github_instance = octocrab::instance();
    let repo = github_instance.repos("zymbit-applications", "zbcli");
    let releases = repo.releases();

    releases
        .get_latest()
        .await
        .context("Failed to get latest release")
}
