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

## Windows beginner quick start

These commands assume you are using PowerShell and want the repository folder to be named `Symmetrical Spoon Advanced Hardlink Manager` on your PC.

1. Install Rust from <https://www.rust-lang.org/tools/install>. Use the default installer option when prompted, then close and reopen PowerShell.
2. Check that Rust is available:

   ```powershell
   cargo --version
   rustc --version
   ```

3. Clone this repository into your Documents folder. Because the local folder name contains spaces, keep the quotation marks:

   ```powershell
   cd $HOME\Documents
   git clone https://github.com/isaac7rodrigo-pixel/Symmetrical-Spoon-Advanced-Hardlink-Manager "Symmetrical Spoon Advanced Hardlink Manager"
   cd "Symmetrical Spoon Advanced Hardlink Manager"
   ```

4. Confirm you are in the right folder:

   ```powershell
   dir
   ```

   You should see `Cargo.toml`, `README.md`, `src`, and `tests`.

5. Build and run `hm-probe` against the current folder:

   ```powershell
   cargo run --bin hm-probe -- .
   ```

If you choose a different local folder name, replace only the quoted folder name in the `git clone` and `cd` commands. The `hm-probe` command stays the same.

### Windows troubleshooting: "The system cannot find the path specified"

This error means PowerShell or Command Prompt is trying to open a folder path that does not exist exactly as typed. Check the folder location before running Cargo commands:

```powershell
cd $HOME\Documents
dir
```

If you see `Symmetrical Spoon Advanced Hardlink Manager` in the list, enter it with quotation marks because the name contains spaces:

```powershell
cd "Symmetrical Spoon Advanced Hardlink Manager"
```

If you do not see that folder, the repository may not have been cloned into Documents yet. Clone it there with:

```powershell
git clone https://github.com/isaac7rodrigo-pixel/Symmetrical-Spoon-Advanced-Hardlink-Manager "Symmetrical Spoon Advanced Hardlink Manager"
cd "Symmetrical Spoon Advanced Hardlink Manager"
```

If you cloned the repository somewhere else, such as Desktop or Downloads, go to that location first:

```powershell
cd $HOME\Desktop
dir
```

Then use `cd` with the exact folder name shown by `dir`. You can also type the first few letters of the folder name and press `Tab` to let PowerShell complete the correct path.
