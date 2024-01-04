// Copyright (c) 2023-2024 taitep <taitep@taitep.se>
// Licensed under the MIT license.

use anyhow::{bail, Result};

use std::{
    env,
    io::{stdin, stdout},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
    terminal_size,
};

use file_navigator;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        bail!("Too many arguments.")
    }

    let mut path: PathBuf = PathBuf::new().join(".");
    if args.len() > 1 {
        path = PathBuf::from(args[1].clone());
    }

    path = path.canonicalize()?;

    let mut screen = stdout().into_raw_mode()?.into_alternate_screen()?;
    let stdin = stdin();

    let (mut w, mut h) = terminal_size()?;

    file_navigator::draw_menu_bar(
        &mut screen,
        std::str::from_utf8(path.as_os_str().as_bytes())?,
    )?;

    let mut files = file_navigator::get_files(&path)?;

    let mut selection = 0;
    let mut scroll = 0;

    file_navigator::draw_file_list(&mut screen, &files, selection, scroll)?;

    let mut events = stdin.events();
    loop {
        match events.next().transpose()? {
            Some(Event::Key(Key::Char('q') | Key::Esc)) => {
                break;
            }
            Some(Event::Key(Key::Up)) => {
                if selection > 0 && files.len() > 0 {
                    selection -= 1;

                    if selection - scroll < 5 && selection >= 5 {
                        scroll -= 1;
                    }

                    file_navigator::draw_file_list(&mut screen, &files, selection, scroll)?;
                }
            }
            Some(Event::Key(Key::Down)) => {
                if files.len() > 0 {
                    if selection < files.len() - 1 {
                        selection += 1;

                        if terminal_size()?.1 - (selection as u16 - scroll as u16 + 3) < 5
                            && files.len() - selection >= 5
                        {
                            scroll += 1;
                        }

                        file_navigator::draw_file_list(&mut screen, &files, selection, scroll)?;
                    }
                }
            }
            Some(Event::Key(Key::Char('\n')) | Event::Key(Key::Right)) => {
                if files.len() > 0 {
                    if files[selection].file_type()?.is_dir() {
                        path = path.join(files[selection].file_name()).canonicalize()?;
                        files = file_navigator::get_files(&path)?;

                        selection = 0;
                        scroll = 0;

                        file_navigator::draw_menu_bar(
                            &mut screen,
                            std::str::from_utf8(path.as_os_str().as_bytes())?,
                        )?;
                        file_navigator::draw_file_list(&mut screen, &files, selection, scroll)?;
                    }
                }
            }
            Some(Event::Key(Key::Backspace) | Event::Key(Key::Left)) => {
                let old_path = path.clone();
                path = path.join("..").canonicalize()?;
                files = file_navigator::get_files(&path)?;

                selection = files
                    .iter()
                    .position(|entry| entry.path() == old_path)
                    .unwrap_or(0);
                scroll = 0;

                file_navigator::draw_menu_bar(
                    &mut screen,
                    std::str::from_utf8(path.as_os_str().as_bytes())?,
                )?;
                file_navigator::draw_file_list(&mut screen, &files, selection, scroll)?;
            }
            _ => {
                if terminal_size()? != (w, h) {
                    (w, h) = terminal_size()?;
                    file_navigator::draw_menu_bar(
                        &mut screen,
                        std::str::from_utf8(path.as_os_str().as_bytes())?,
                    )?;
                    file_navigator::draw_file_list(&mut screen, &files, selection, scroll)?;
                }
            }
        }
    }

    Ok(())
}
