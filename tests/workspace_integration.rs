use tuicli::desktop::workspace::{Workspace, WorkspaceId};

#[test]
fn workspace_constructor_preserves_id_and_name() {
    let workspace = Workspace::new(3, "CODE");

    assert_eq!(workspace.id, WorkspaceId(3));
    assert_eq!(workspace.name, "CODE");
}

#[test]
fn workspace_serialization_round_trip() {
    let original = Workspace::new(3, "CODE");

    let serialized = toml::to_string(&original).expect("workspace should serialize to TOML");

    let restored: Workspace =
        toml::from_str(&serialized).expect("workspace should deserialize from TOML");

    assert_eq!(restored, original);
}
