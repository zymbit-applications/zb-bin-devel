use anyhow::{Context, Result};
use octocrab::models::repos::Release;
use urlencoding::encode;

/// `tag_prefix`: `zbcli` in `zbcli-1.1.0`
pub async fn list(tag_prefix: &str, zb_version: &Option<String>) -> Result<Vec<Release>> {
    let github_instance = octocrab::instance();
    let repo = github_instance.repos("zymbit-applications", "zb-bin");

    let releases = repo.releases();

    // special case when a specific tag is requested
    if let Some(version) = zb_version {
        if version.to_lowercase().ne("latest") {
            let version_tag = encode(format!("{tag_prefix}-{version}").as_str()).into_owned();
            let release = releases
                .get_by_tag(version_tag.as_str())
                .await
                .context(format!(
                    "Failed to get release {version}.\n\
                    Note that 'zbcli-' should be omitted from the version argument;\n\
                    e.g. to select 'zbcli-1.2.0-rc.23', specify '1.2.0-rc.23' as \
                    the parameter to '--zb-version'"
                ))?;
            return Ok(std::iter::once(release).collect());
        }
    }

    let zbcli_filter = releases
        .list()
        .per_page(10)
        .send()
        .await
        .context("Failed to get latest releases")?
        .into_iter()
        .filter(|release| release.tag_name.starts_with(&format!("{tag_prefix}-")));

    Ok(zbcli_filter.collect())
}
