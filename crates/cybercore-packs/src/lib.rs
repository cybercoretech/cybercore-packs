use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::{Path, PathBuf}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid pack: {0}")]
    Invalid(String),
    #[error("checksum mismatch for {path}: expected {expected}, got {actual}")]
    ChecksumMismatch { path: String, expected: String, actual: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackManifest {
    pub id: String,
    pub version: String,
    pub kind: String,
    pub channel: Channel,
    pub license: String,
    #[serde(default)]
    pub compatibility: Compatibility,
    #[serde(default)]
    pub dependencies: Vec<PackDependency>,
    #[serde(default)]
    pub resources: Vec<PackResource>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Compatibility {
    #[serde(default)]
    pub control_plane: Option<String>,
    #[serde(default)]
    pub web_kit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackDependency {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackResource {
    pub id: String,
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub preview: Option<String>,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub source: String,
    pub author: String,
    pub reviewed: bool,
}

impl PackManifest {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, PackError> {
        let manifest: Self = serde_json::from_slice(&fs::read(path)?)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), PackError> {
        if self.id.split('.').count() < 2 {
            return Err(PackError::Invalid(format!("pack ID must be namespaced: {}", self.id)));
        }
        if self.version.is_empty() || self.kind.is_empty() || self.license.is_empty() {
            return Err(PackError::Invalid(format!("{} has incomplete identity fields", self.id)));
        }
        if !self.provenance.source.starts_with("https://") {
            return Err(PackError::Invalid(format!("{} provenance source must use https", self.id)));
        }
        for resource in &self.resources {
            if resource.id.is_empty() || resource.path.starts_with('/') || resource.sha256.len() != 64 {
                return Err(PackError::Invalid(format!("{} has an invalid resource entry", self.id)));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub schema_version: u32,
    pub packs: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub id: String,
    pub version: String,
    pub kind: String,
    pub channel: Channel,
    pub manifest: String,
    pub archive: Option<String>,
}

impl Registry {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, PackError> {
        Ok(serde_json::from_slice(&fs::read(path)?)?)
    }

    pub fn search<'a>(&'a self, query: &str) -> impl Iterator<Item = &'a RegistryEntry> {
        let query = query.to_ascii_lowercase();
        self.packs.iter().filter(move |pack| {
            pack.id.to_ascii_lowercase().contains(&query)
                || pack.kind.to_ascii_lowercase().contains(&query)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackLock {
    pub lockfile_version: u32,
    pub packs: Vec<LockedPack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPack {
    pub id: String,
    pub version: String,
    pub manifest: String,
    pub archive: Option<String>,
}

impl PackLock {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, PackError> {
        Ok(serde_json::from_slice(&fs::read(path)?)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), PackError> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn add(&mut self, entry: &RegistryEntry) {
        self.packs.retain(|pack| pack.id != entry.id);
        self.packs.push(LockedPack {
            id: entry.id.clone(),
            version: entry.version.clone(),
            manifest: entry.manifest.clone(),
            archive: entry.archive.clone(),
        });
        self.packs.sort_by(|a, b| a.id.cmp(&b.id));
    }

    pub fn verify_files(&self, root: impl AsRef<Path>) -> Result<(), PackError> {
        for pack in &self.packs {
            let manifest_path = root.as_ref().join(&pack.manifest);
            PackManifest::from_path(manifest_path)?;
        }
        Ok(())
    }
}

pub fn sha256_file(path: impl AsRef<Path>) -> Result<String, PackError> {
    let bytes = fs::read(path)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn resolve_manifest_path(root: impl AsRef<Path>, entry: &RegistryEntry) -> PathBuf {
    root.as_ref().join(&entry.manifest)
}
