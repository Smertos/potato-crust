use serde::Deserialize;
use sha1_smol::Digest;

use crate::common::VersionId;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct VersionManifest {
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    assets: String, // number string
    id: VersionId,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct AssetIndex {
    id: String, // number string
    sha1: Digest,
    size: u64,
    #[serde(rename = "totalSize")]
    total_size: u64,
    url: String,
}

#[allow(dead_code)]
pub async fn get_version_manifest(url: impl AsRef<str>) -> anyhow::Result<VersionManifest> {
    let version_manifest: VersionManifest = reqwest::get(url.as_ref()).await?.json().await?;

    Ok(version_manifest)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_and_parse_version_manifest_test() {
        use super::{get_version_manifest, AssetIndex};
        use sha1_smol::Digest;
        use std::str::FromStr;

        const TEST_VERSION_MANIFEST_URL: &str = "https://piston-meta.mojang.com/v1/packages/ff7960c2739033d6439f660ed11999322e6e6e99/1.19.3.json";
        let test_digest = Digest::from_str("63b14365d7df8a206d2fae60e3400d84bab5a7a4")
            .expect("create test digest from predefined hex hash");

        let manifest = get_version_manifest(TEST_VERSION_MANIFEST_URL)
            .await
            .expect("get test version manifest from URL");

        assert_eq!(
            manifest.id,
            "1.19.3".into(),
            "loaded version manifest's version-id does not match test data"
        );

        assert_eq!(
            manifest.asset_index,
            AssetIndex {
                id: "2".into(),
                sha1: test_digest,
                size: 390746,
                total_size: 549080055,
                url: "https://piston-meta.mojang.com/v1/packages/63b14365d7df8a206d2fae60e3400d84bab5a7a4/2.json".into(),
            },
            "loaded asset index info does not match test data",
        );
    }
}
