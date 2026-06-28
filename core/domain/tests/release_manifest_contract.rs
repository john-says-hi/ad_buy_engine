use std::collections::BTreeMap;

use ad_buy_engine_domain::{
    CURRENT_RELEASE_SCHEMA, ReleaseArtifact, ReleaseManifest, ReleaseRollbackPolicy,
    ReleaseSchemaCompatibility,
};

const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";

#[test]
fn manifest_validation_accepts_valid_release() -> Result<(), String> {
    let manifest = valid_manifest(TARGET_TRIPLE);
    let checksums = valid_checksums();

    manifest
        .validate(TARGET_TRIPLE, &checksums)
        .map_err(|errors| format!("unexpected validation errors: {errors:?}"))
}

#[test]
fn manifest_validation_rejects_wrong_target_triple() {
    let manifest = valid_manifest("aarch64-unknown-linux-gnu");
    let checksums = valid_checksums();

    let errors = manifest
        .validate(TARGET_TRIPLE, &checksums)
        .expect_err("wrong target triple should fail");

    assert!(errors.iter().any(|error| error.field == "target_triple"));
}

#[test]
fn manifest_validation_rejects_missing_files() {
    let manifest = valid_manifest(TARGET_TRIPLE);
    let mut checksums = valid_checksums();
    checksums.remove("campaign_server");

    let errors = manifest
        .validate(TARGET_TRIPLE, &checksums)
        .expect_err("missing binary should fail");

    assert!(errors.iter().any(|error| {
        error
            .message
            .contains("Missing release file campaign_server")
    }));
}

#[test]
fn manifest_validation_rejects_digest_mismatch() {
    let manifest = valid_manifest(TARGET_TRIPLE);
    let mut checksums = valid_checksums();
    checksums.insert("dist/index.html".to_string(), "bad-digest".to_string());

    let errors = manifest
        .validate(TARGET_TRIPLE, &checksums)
        .expect_err("digest mismatch should fail");

    assert!(errors.iter().any(|error| {
        error
            .message
            .contains("Digest mismatch for dist/index.html")
    }));
}

fn valid_manifest(target_triple: &str) -> ReleaseManifest {
    ReleaseManifest {
        version: "v0.1.0".to_string(),
        git_sha: "0123456789abcdef".to_string(),
        target_triple: target_triple.to_string(),
        schema: ReleaseSchemaCompatibility {
            manifest_version: CURRENT_RELEASE_SCHEMA,
            minimum_supported_schema: 1,
            maximum_supported_schema: 3,
            migrations_backward_compatible: true,
        },
        rollback: ReleaseRollbackPolicy {
            schema_rollback_safe: true,
            requires_database_restore: false,
            notes: "expand-only migration".to_string(),
        },
        binary_path: "campaign_server".to_string(),
        dashboard_path: "dist/index.html".to_string(),
        artifacts: vec![
            ReleaseArtifact {
                path: "campaign_server".to_string(),
                sha256: "abc123".to_string(),
            },
            ReleaseArtifact {
                path: "dist/index.html".to_string(),
                sha256: "def456".to_string(),
            },
        ],
    }
}

fn valid_checksums() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("campaign_server".to_string(), "abc123".to_string()),
        ("dist/index.html".to_string(), "def456".to_string()),
    ])
}
