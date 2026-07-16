//! Read-only hardlink detection engine for the Hardlink Manager Double Commander plugin.
//!
//! Version 1 intentionally does not mutate the filesystem. The exported engine only reads
//! metadata and directory entries so destructive menu items can be shown as future actions
//! without being executed by this crate.

use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A filesystem item's hardlink state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    /// The item has a single directory entry, or its link count cannot prove hardlinking.
    NotHardlinked,
    /// The item has more than one directory entry pointing to the same inode/file index.
    Hardlinked,
}

/// Folder aggregate state used by the file panel coloring layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderContentState {
    /// No hardlinked descendants were found.
    None,
    /// At least one, but not all, descendants are hardlinked. Suggested UI: neon orange.
    Some,
    /// Every detected descendant is hardlinked. Suggested UI: red.
    All,
    /// The folder has no descendants to classify.
    Empty,
}

/// Full read-only analysis for a selected path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemAnalysis {
    pub path: PathBuf,
    pub is_dir: bool,
    pub link_state: LinkState,
    pub folder_content_state: Option<FolderContentState>,
    pub scanned_entries: usize,
    pub hardlinked_entries: usize,
}

impl ItemAnalysis {
    /// Whether the selected item itself should show the ↘ marker.
    pub fn shows_marker(&self) -> bool {
        self.link_state == LinkState::Hardlinked
    }
}

/// Analyze a path without modifying it.
pub fn analyze_path(path: impl AsRef<Path>) -> io::Result<ItemAnalysis> {
    let path = path.as_ref();
    let metadata = fs::symlink_metadata(path)?;
    let is_dir = metadata.is_dir();
    let link_state = link_state_from_metadata(&metadata);

    let (folder_content_state, scanned_entries, hardlinked_entries) = if is_dir {
        let summary = analyze_folder_contents(path)?;
        (
            Some(summary.state),
            summary.scanned_entries,
            summary.hardlinked_entries,
        )
    } else {
        (None, 0, 0)
    };

    Ok(ItemAnalysis {
        path: path.to_path_buf(),
        is_dir,
        link_state,
        folder_content_state,
        scanned_entries,
        hardlinked_entries,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FolderSummary {
    state: FolderContentState,
    scanned_entries: usize,
    hardlinked_entries: usize,
}

fn analyze_folder_contents(root: &Path) -> io::Result<FolderSummary> {
    let mut queue = VecDeque::from([root.to_path_buf()]);
    let mut scanned_entries = 0usize;
    let mut hardlinked_entries = 0usize;

    while let Some(dir) = queue.pop_front() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)?;

            scanned_entries += 1;
            if link_state_from_metadata(&metadata) == LinkState::Hardlinked {
                hardlinked_entries += 1;
            }
            if metadata.is_dir() {
                queue.push_back(path);
            }
        }
    }

    let state = match (scanned_entries, hardlinked_entries) {
        (0, _) => FolderContentState::Empty,
        (_, 0) => FolderContentState::None,
        (total, linked) if total == linked => FolderContentState::All,
        _ => FolderContentState::Some,
    };

    Ok(FolderSummary {
        state,
        scanned_entries,
        hardlinked_entries,
    })
}

#[cfg(unix)]
fn link_state_from_metadata(metadata: &fs::Metadata) -> LinkState {
    use std::os::unix::fs::MetadataExt;

    if metadata.nlink() > 1 {
        LinkState::Hardlinked
    } else {
        LinkState::NotHardlinked
    }
}

#[cfg(windows)]
fn link_state_from_metadata(metadata: &fs::Metadata) -> LinkState {
    use std::os::windows::fs::MetadataExt;

    if metadata.number_of_links() > 1 {
        LinkState::Hardlinked
    } else {
        LinkState::NotHardlinked
    }
}

/// Menu entries for Double Commander's future context menu integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuModel {
    pub entries: Vec<MenuEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuEntry {
    Action(&'static str),
    Separator,
}

/// Build the exact Hardlink Tools menu requested for the current read-only analysis.
pub fn build_menu(analysis: &ItemAnalysis, selected_folder_count: usize) -> MenuModel {
    let mut entries = vec![MenuEntry::Action("↘ Show Link Information")];

    if analysis.shows_marker() {
        entries.extend([
            MenuEntry::Action("Edit Hardlink"),
            MenuEntry::Action("Remove Hardlink"),
            MenuEntry::Action("Disable Hardlink"),
        ]);
    } else {
        entries.push(MenuEntry::Action("Hardlink"));
    }

    if analysis.is_dir {
        entries.push(MenuEntry::Separator);
        if matches!(
            analysis.folder_content_state,
            Some(FolderContentState::None | FolderContentState::Empty)
        ) {
            entries.push(MenuEntry::Action("Mass Hardlink Folder Contents"));
        } else {
            entries.extend([
                MenuEntry::Action("Edit Mass Hardlink"),
                MenuEntry::Action("Disable Mass Hardlink"),
                MenuEntry::Action(if selected_folder_count > 1 {
                    "Nuke all Hardlinks inside Folders"
                } else {
                    "Nuke all Hardlinks inside Folder"
                }),
            ]);
        }
    }

    MenuModel { entries }
}
