use hardlink_manager::{analyze_path, build_menu, MenuEntry};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = env::args_os().nth(1) else {
        eprintln!("usage: hm-probe <path>");
        return ExitCode::from(2);
    };

    match analyze_path(path) {
        Ok(analysis) => {
            println!("path: {}", analysis.path.display());
            println!("is_dir: {}", analysis.is_dir);
            println!("item_state: {:?}", analysis.link_state);
            println!("shows_marker: {}", analysis.shows_marker());
            if let Some(state) = analysis.folder_content_state {
                println!("folder_content_state: {:?}", state);
                println!("scanned_entries: {}", analysis.scanned_entries);
                println!("hardlinked_entries: {}", analysis.hardlinked_entries);
            }
            println!("Hardlink Tools:");
            for entry in build_menu(&analysis, usize::from(analysis.is_dir)).entries {
                match entry {
                    MenuEntry::Action(label) => println!("  {label}"),
                    MenuEntry::Separator => println!("  ─────────"),
                }
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("hm-probe: {error}");
            ExitCode::FAILURE
        }
    }
}
