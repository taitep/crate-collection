use pancurses::{Window, COLOR_PAIR};

use anyhow;

use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn draw_menu_bars(window: &Window, color_pair: u32, path: &str) {
    window.chgat(-1, COLOR_PAIR(color_pair), color_pair.try_into().unwrap());
    window.attrset(COLOR_PAIR(color_pair));
    window.mv(0, 0);
    window.addstr(path);
}

pub fn draw_file_list(
    window: &Window,
    files: &Vec<DirEntry>,
    selection: usize,
    scroll: usize,
) -> anyhow::Result<()> {
    window.attrset(COLOR_PAIR(0));
    window.mv(1, 0);
    if scroll > 0 {
        window.addstr("...\n");
    } else {
        window.addstr("   \n");
    }

    window.clrtobot();

    let max_files_shown = window.get_max_y() - 3;

    for file in files.iter().skip(scroll).take(max_files_shown as usize) {
        window.addstr(format!(
            "{}{}",
            match file.file_name().to_str() {
                Some(name) => name,
                None => {
                    continue;
                }
            },
            match file.file_type()?.is_dir() {
                true => "/",
                false => "",
            }
        ));
        window.addch('\n');
    }

    if files.len() > scroll + max_files_shown as usize {
        window.addstr("...\n");
    } else {
        window.addstr("   \n");
    }

    let screen_selection = selection - scroll + 2;
    window.mv(screen_selection as i32, 0);

    window.refresh();

    Ok(())
}

pub fn get_files(path: PathBuf) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries: Vec<DirEntry> = vec![];
    for entry in fs::read_dir(path).unwrap() {
        match entry {
            Ok(entry) => entries.insert(entries.len(), entry),
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(entries)
}
