use anyhow::{Context, Result};
use octocrab::models::repos::Release;
use urlencoding::encode;

/// `tag_prefix`: `zbcli` in `zbcli-1.1.0`
pub async fn list(tag_prefix: &str, zb_version: &Option<String>) -> Result<Vec<Release>> {
    let github_instance = octocrab::instance();
    let repo = github_instance.repos("zymbit-applications", "zb-bin");

    let releases = repo.releases();

    let release_list = if let Some(version) = zb_version {
        let release = if version.to_lowercase().eq("latest") {
            releases
                .get_latest()
                .await
                .context("Failed to get latest release")?
        } else {
            let version_tag = encode(format!("{tag_prefix}-{version}").as_str()).into_owned();
            releases
                .get_by_tag(version_tag.as_str())
                .await
                .context(format!(
                    "Failed to get release {version}.\n\
                    Note that 'zbcli-' should be ommitted from the version argument;\n\
                    e.g. to select 'zbcli-1.2.0-rc.23', specify '1.2.0-rc.23' as \
                    the parameter to '--zb-version'"
                ))?
        };
        std::iter::once(release).collect()
    } else {
        releases
            .list()
            .per_page(10)
            .send()
            .await
            .context("Failed to get latest releases")?
            .into_iter()
            .filter(|release| release.tag_name.starts_with(&format!("{tag_prefix}-")))
            .collect()
    };

    Ok(release_list)
}
