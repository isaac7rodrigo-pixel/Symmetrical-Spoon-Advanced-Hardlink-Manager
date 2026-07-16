# Hardlink Manager for Double Commander

Hardlink Manager is a planned Double Commander plugin that makes hardlinks easier to see and manage. It is designed for general files and folders, with ModDB-style mod staging workflows as the motivating use case.

## Version 1 scope

Version 1 is intentionally **read-only**:

- Detect whether the selected file or folder is hardlinked.
- Return whether the selected item should display the `↘` marker.
- Analyze folder contents for color-state decisions:
  - `All` → red folder/text (`🔴` in examples).
  - `Some` → neon orange folder/text (`🟠` in examples).
  - `None` or `Empty` → normal display.
- Build the requested `Hardlink Tools` menu model with exact destructive-action wording, but do not execute destructive actions.

The first implemented milestone is the hardlink detection engine in Rust. Double Commander integration can call this engine from a native plugin layer and map its states to panel coloring/context-menu UI.

## Safety model

The engine only reads filesystem metadata and directory entries. It does not remove, disable, enable, rewrite, or relink files. Future versions must keep these guarantees:

- Removing a hardlink must remove only one directory entry and must not delete the remaining original data.
- Disabling hardlinks must persist enough state to restore what was disabled.
- Mass actions and nuke actions must require an explicit confirmation layer outside the read-only detection engine.

## Menu model

The root context menu is named `Hardlink Tools`.

For an item that is itself hardlinked, the engine returns:

```text
↘ Show Link Information
Edit Hardlink
Remove Hardlink
Disable Hardlink
```

For an item that is not itself hardlinked, the engine returns:

```text
↘ Show Link Information
Hardlink
```

For folders, the engine appends folder actions after a separator. A folder with no hardlinked contents gets only the mass-create option:

```text
Mass Hardlink Folder Contents
```

Folders with hardlinked contents omit that mass-create action and get the mass-edit/destructive placeholders:

```text
Edit Mass Hardlink
Disable Mass Hardlink
Nuke all Hardlinks inside Folder
```

When multiple folders are selected, the exact nuke label becomes:

```text
Nuke all Hardlinks inside Folders
```

## Development

Run the test suite:

```bash
cargo test
```

Probe a path from the command line:

```bash
cargo run --bin hm-probe -- <path>
```
