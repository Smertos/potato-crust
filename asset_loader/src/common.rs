use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(try_from = "&str")]
pub enum VersionType {
    Snapshot,
    Release,
    OldBeta,
    OldAlpha,
}

impl TryFrom<&str> for VersionType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value {
            "snapshot" => VersionType::Snapshot,
            "release" => VersionType::Release,
            "old_beta" => VersionType::OldBeta,
            "old_alpha" => VersionType::OldAlpha,
            unknown_type => return Err(anyhow::anyhow!("Unknown version type: {}", unknown_type)),
        };

        Ok(result)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(from = "&str")]
#[repr(transparent)]
pub struct VersionId(pub String);

impl From<&str> for VersionId {
    fn from(value: &str) -> Self {
        VersionId(value.into())
    }
}
