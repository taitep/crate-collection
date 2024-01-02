// Copyright (c) 2023-2024 taitep (https://github.com/taitep)
// Licensed under the MIT license.

use anyhow::Result;

use termion::{
    clear,
    color::{self, Bg, Fg},
    cursor, style, terminal_size,
};

use std::{
    fs::{self, DirEntry},
    io::Write,
    path::PathBuf,
};

pub fn draw_menu_bar<S: Write>(screen: &mut S, path: &str) -> Result<()> {
    write!(
        screen,
        "{}{}{}{}{}{}{}",
        cursor::Goto(1, 1),
        Fg(color::Black),
        Bg(color::White),
        " ".repeat(terminal_size()?.0 as usize),
        cursor::Goto(1, 1),
        path,
        style::Reset
    )?;
    Ok(())
}

pub fn draw_file_list<S: Write>(
    screen: &mut S,
    files: &Vec<DirEntry>,
    selection: usize,
    scroll: usize,
) -> anyhow::Result<()> {
    write!(
        screen,
        "{}{}\n\r{}",
        cursor::Goto(1, 2),
        if scroll > 0 { "..." } else { "   " },
        clear::AfterCursor
    )?;

    let max_files_shown = terminal_size()?.1 - 3;

    for file in files.iter().skip(scroll).take(max_files_shown as usize) {
        write!(
            screen,
            "{}{}\n\r",
            match file.file_name().to_str() {
                Some(name) => name,
                None => {
                    continue;
                }
            },
            if file.file_type()?.is_dir() { "/" } else { "" }
        )?;
    }

    write!(
        screen,
        "{}",
        if files.len() > scroll + max_files_shown as usize {
            "..."
        } else {
            "   "
        }
    )?;

    let screen_selection = selection - scroll + 3;
    write!(screen, "{}", cursor::Goto(1, screen_selection as u16))?;

    screen.flush()?;

    Ok(())
}

pub fn get_files(path: &PathBuf) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries: Vec<DirEntry> = vec![];
    for entry in fs::read_dir(path)? {
        match entry {
            Ok(entry) => entries.insert(entries.len(), entry),
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(entries)
}
