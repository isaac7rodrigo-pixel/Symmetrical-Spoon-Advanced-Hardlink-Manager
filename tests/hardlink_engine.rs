use hardlink_manager::{analyze_path, build_menu, FolderContentState, LinkState, MenuEntry};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let mut path = std::env::temp_dir();
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("hm-test-{suffix}"));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &PathBuf {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn detects_file_hardlink_marker() {
    let temp = TempDir::new();
    let original = temp.path().join("original.txt");
    let linked = temp.path().join("linked.txt");
    fs::write(&original, "data").unwrap();
    fs::hard_link(&original, &linked).unwrap();

    let analysis = analyze_path(&original).unwrap();

    assert_eq!(analysis.link_state, LinkState::Hardlinked);
    assert!(analysis.shows_marker());
}

#[test]
fn classifies_folder_with_some_hardlinked_contents() {
    let temp = TempDir::new();
    let original = temp.path().join("original.txt");
    let linked = temp.path().join("linked.txt");
    let normal = temp.path().join("normal.txt");
    fs::write(&original, "data").unwrap();
    fs::hard_link(&original, &linked).unwrap();
    fs::write(normal, "normal").unwrap();

    let analysis = analyze_path(temp.path()).unwrap();

    assert_eq!(
        analysis.folder_content_state,
        Some(FolderContentState::Some)
    );
    assert_eq!(analysis.hardlinked_entries, 2);
}

#[test]
fn uses_exact_singular_and_plural_nuke_labels() {
    let temp = TempDir::new();
    let original = temp.path().join("original.txt");
    let linked = temp.path().join("linked.txt");
    fs::write(&original, "data").unwrap();
    fs::hard_link(&original, &linked).unwrap();
    let analysis = analyze_path(temp.path()).unwrap();

    let singular = build_menu(&analysis, 1);
    let plural = build_menu(&analysis, 2);

    assert!(singular
        .entries
        .contains(&MenuEntry::Action("Nuke all Hardlinks inside Folder")));
    assert!(plural
        .entries
        .contains(&MenuEntry::Action("Nuke all Hardlinks inside Folders")));
}

#[test]
fn omits_mass_hardlink_action_when_folder_already_has_hardlinked_contents() {
    let temp = TempDir::new();
    let original = temp.path().join("original.txt");
    let linked = temp.path().join("linked.txt");
    fs::write(&original, "data").unwrap();
    fs::hard_link(&original, &linked).unwrap();

    let analysis = analyze_path(temp.path()).unwrap();
    let menu = build_menu(&analysis, 1);

    assert!(!menu
        .entries
        .contains(&MenuEntry::Action("Mass Hardlink Folder Contents")));
}

#[test]
fn base_folder_only_offers_mass_hardlink_contents() {
    let temp = TempDir::new();
    fs::write(temp.path().join("normal.txt"), "normal").unwrap();

    let analysis = analyze_path(temp.path()).unwrap();
    let menu = build_menu(&analysis, 1);

    assert!(menu
        .entries
        .contains(&MenuEntry::Action("Mass Hardlink Folder Contents")));
    assert!(!menu
        .entries
        .contains(&MenuEntry::Action("Edit Mass Hardlink")));
    assert!(!menu
        .entries
        .contains(&MenuEntry::Action("Nuke all Hardlinks inside Folder")));
}
