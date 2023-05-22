use serde::Deserialize;
use sha1_smol::Digest;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct AssetManifest {
    objects: HashMap<String, AssetInfo>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct AssetInfo {
    hash: Digest,
    size: u64,
}

#[allow(dead_code)]
pub async fn get_asset_manifest(url: impl AsRef<str>) -> anyhow::Result<AssetManifest> {
    let asset_manifest: AssetManifest = reqwest::get(url.as_ref()).await?.json().await?;

    Ok(asset_manifest)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_asset_manifest_test() {
        use super::get_asset_manifest;
        use sha1_smol::Digest;
        use std::str::FromStr;

        const TEST_ASSET_MANIFEST_URL: &str = "https://piston-meta.mojang.com/v1/packages/63b14365d7df8a206d2fae60e3400d84bab5a7a4/2.json";
        const TEST_ASSET_KEY: &str = "icons/icon_32x32.png";
        let test_digest = Digest::from_str("92750c5f93c312ba9ab413d546f32190c56d6f1f")
            .expect("create test digest from predefined hex hash");

        let manifest = get_asset_manifest(TEST_ASSET_MANIFEST_URL)
            .await
            .expect("successfully get asset manifest");

        assert!(
            manifest.objects.contains_key(TEST_ASSET_KEY),
            "asset manifest does not contain expected asset"
        );

        let asset_info = manifest
            .objects
            .get(TEST_ASSET_KEY)
            .expect("checked asset should be in place");

        assert_eq!(
            asset_info.hash, test_digest,
            "asset info's hash does not match test data"
        );

        assert_eq!(
            asset_info.size, 5362,
            "asset info's size does not match test data"
        );
    }
}
