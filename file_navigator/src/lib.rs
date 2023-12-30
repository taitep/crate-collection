use pancurses::{Window, COLOR_PAIR};

use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn draw_menu_bars(window: &Window, color_pair: u32, path: &str) {
    window.chgat(-1, COLOR_PAIR(color_pair), color_pair.try_into().unwrap());
    window.attrset(COLOR_PAIR(color_pair));
    window.mv(0, 0);
    window.addstr(path);
}

pub fn draw_file_list(window: &Window, files: Vec<&str>) {
    window.attrset(COLOR_PAIR(0));
    window.mv(1, 0);

    for file in files {
        window.addstr(file);
        window.addch('\n');
    }
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
