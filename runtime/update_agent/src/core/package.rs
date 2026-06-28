use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use ad_buy_engine_domain::{ReleaseManifest, UpdateSlot};
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct ReleasePackage {
    pub root: PathBuf,
    pub manifest: ReleaseManifest,
    pub actual_sha256_by_path: BTreeMap<String, String>,
}

impl ReleasePackage {
    pub fn from_unpacked_root(root: PathBuf) -> Result<Self> {
        let manifest_path = root.join("manifest.json");
        let manifest: ReleaseManifest = serde_json::from_slice(
            &fs::read(&manifest_path)
                .with_context(|| format!("failed to read {}", manifest_path.display()))?,
        )
        .context("release manifest is not valid JSON")?;
        let actual_sha256_by_path = sha256_by_relative_path(&root)?;
        Ok(Self {
            root,
            manifest,
            actual_sha256_by_path,
        })
    }

    pub fn from_manifest_for_tests(
        root: PathBuf,
        manifest: ReleaseManifest,
        actual_sha256_by_path: BTreeMap<String, String>,
    ) -> Self {
        Self {
            root,
            manifest,
            actual_sha256_by_path,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstalledRelease {
    pub version: String,
    pub path: PathBuf,
    pub slot: UpdateSlot,
    pub manifest: ReleaseManifest,
}

pub fn sha256_by_relative_path(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut output = BTreeMap::new();
    collect_sha256(root, root, &mut output)?;
    Ok(output)
}

fn collect_sha256(
    root: &Path,
    current: &Path,
    output: &mut BTreeMap<String, String>,
) -> Result<()> {
    for entry in
        fs::read_dir(current).with_context(|| format!("failed to read {}", current.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_sha256(root, &path, output)?;
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("manifest.json") {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .context("release file was outside the package root")?
            .to_string_lossy()
            .replace('\\', "/");
        output.insert(relative, sha256_file(&path)?);
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file =
        fs::File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("failed to read {}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
