use crate::common::{VersionId, VersionType};

use serde::Deserialize;
use url::Url;

const VERSION_LIST_MANIFEST_API_URL: &str =
    "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[allow(dead_code)]
pub async fn get_version_list_manifest() -> anyhow::Result<VersionListManifest> {
    let response = reqwest::get(VERSION_LIST_MANIFEST_API_URL).await?;
    let version_list_manifest: VersionListManifest = response.json().await?;

    Ok(version_list_manifest)
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct VersionListManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionListItem>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct LatestVersions {
    pub release: VersionId,
    pub snapshot: VersionId,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct VersionListItem {
    pub id: VersionId,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub url: Url,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn parse_version_list_manifest_test() {
        use super::*;

        const VERSION_MANIFEST_SAMPLE: &str = r#"
            {
                "latest": {
                    "release": "1.19.3",
                    "snapshot": "23w04a"
                },
                "versions": [
                    {"id": "1.19.3", "type": "release", "url": "https://piston-meta.mojang.com/v1/packages/ff7960c2739033d6439f660ed11999322e6e6e99/1.19.3.json", "time": "2023-01-24T14:49:13+00:00", "releaseTime": "2022-12-07T08:17:18+00:00"}
                ]
            }
        "#;

        let manifest: VersionListManifest = serde_json::from_str(VERSION_MANIFEST_SAMPLE)
            .expect("successfully parse manifest sample");

        assert!(
            !manifest.latest.snapshot.0.is_empty(),
            "latest snapshot is empty!"
        );
        assert!(
            !manifest.latest.release.0.is_empty(),
            "latest release is empty!"
        );

        assert!(
            manifest.versions.len() > 0,
            "versions list is empty!"
        );

        let version_manifest = manifest.versions.get(0).unwrap();

        assert_eq!(version_manifest.id, VersionId("1.19.3".into()));
        assert_eq!(version_manifest.version_type, VersionType::Release);
    }

    #[tokio::test]
    async fn get_version_list_manifest_test() {
        use super::*;

        let mut manifest = get_version_list_manifest().await.expect("get manifest");

        assert!(
            !manifest.latest.snapshot.0.is_empty(),
            "latest snapshot is empty!"
        );
        assert!(
            !manifest.latest.release.0.is_empty(),
            "latest release is empty!"
        );

        manifest
            .versions
            .retain(|version_info| version_info.id == "1.19.3".into());

        assert!(
            manifest.versions.len() > 0,
            "versions list is empty!"
        );

        let version_manifest = manifest.versions.get(0).unwrap();

        assert_eq!(version_manifest.id, VersionId("1.19.3".into()));
        assert_eq!(version_manifest.version_type, VersionType::Release);
    }
}
