use super::*;

#[test]
fn config_dir_uses_tuicli_path() {
    let path = config_dir().expect("config directory should exist");

    assert!(
        path.ends_with("tuicli"),
        "Unexpected config directory: {}",
        path.display()
    );
}
