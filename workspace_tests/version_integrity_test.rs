use crate::toml_utils::{DependencyValue, LocalCrate, PackageEntryValue, MEMBER_TOMLS, ROOT_TOML};

const PARENT_BRANCH: &str = include_str!("../scripts/parent_branch.txt");
const MAIN_PARENT_BRANCH: &str = "main";
const EXPECTED_MAIN_VERSION: &str = "0.0.0";

#[test]
fn test_path_dependencies_are_members() {
    let non_member_path_crates: Vec<_> = ROOT_TOML
        .path_dependencies()
        .filter(|LocalCrate { path, .. }| !ROOT_TOML.members().contains(path))
        .collect();
    assert!(
        non_member_path_crates.is_empty(),
        "The following crates are path dependencies but not members of the workspace: \
         {non_member_path_crates:?}."
    );
}

#[test]
fn test_version_alignment() {
    let workspace_version = ROOT_TOML.workspace_version();
    let crates_with_incorrect_version: Vec<_> = ROOT_TOML
        .path_dependencies()
        .filter(|LocalCrate { version, .. }| version != workspace_version)
        .collect();
    assert!(
        crates_with_incorrect_version.is_empty(),
        "The following crates have versions different from the workspace version \
         '{workspace_version}': {crates_with_incorrect_version:?}."
    );
}

#[test]
fn validate_crate_version_is_workspace() {
    let crates_without_workspace_version: Vec<String> = MEMBER_TOMLS
        .iter()
        .flat_map(|(member, toml)| match toml.package.get("version") {
            // No `version` field.
            None => Some(member.clone()),
            Some(version) => match version {
                // version = "x.y.z".
                PackageEntryValue::String(_) => Some(member.clone()),
                // version.workspace = (true | false).
                PackageEntryValue::Object { workspace } => {
                    if *workspace {
                        None
                    } else {
                        Some(member.clone())
                    }
                }
                // Unknown version object.
                PackageEntryValue::Other(_) => Some(member.clone()),
            },
        })
        .collect();

    assert!(
        crates_without_workspace_version.is_empty(),
        "The following crates don't have `version.workspace = true` in the [package] section: \
         {crates_without_workspace_version:?}."
    );
}

#[test]
fn validate_no_path_dependencies() {
    let all_path_deps_in_crate_tomls: Vec<String> =
        MEMBER_TOMLS.values().flat_map(|toml| toml.path_dependencies()).collect();

    assert!(
        all_path_deps_in_crate_tomls.is_empty(),
        "The following crates have path dependency {all_path_deps_in_crate_tomls:?}."
    );
}

#[test]
fn test_no_features_in_workspace() {
    let dependencies_with_features: Vec<_> = ROOT_TOML
        .dependencies()
        .filter_map(|(name, dependency)| match dependency {
            DependencyValue::Object { features: Some(features), .. } => Some((name, features)),
            _ => None,
        })
        .collect();
    assert!(
        dependencies_with_features.is_empty(),
        "The following dependencies have features enabled in the workspace Cargo.toml: \
         {dependencies_with_features:#?}. Features should only be activated in the crate that \
         needs them."
    );
}

#[test]
fn test_main_branch_is_versionless() {
    if PARENT_BRANCH.trim() == MAIN_PARENT_BRANCH {
        let workspace_version = ROOT_TOML.workspace_version();
        assert_eq!(
            workspace_version, EXPECTED_MAIN_VERSION,
            "The workspace version should be '{EXPECTED_MAIN_VERSION}' when the parent branch is \
             '{MAIN_PARENT_BRANCH}'; found {workspace_version}.",
        );
    }
}
