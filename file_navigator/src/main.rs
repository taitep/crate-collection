use anyhow::{self, bail};
use file_navigator;
use std::path::PathBuf;

use pancurses::{
    endwin, has_colors, init_pair, initscr, noecho, start_color, Input, Window, COLOR_BLACK,
    COLOR_WHITE,
};

fn main() -> anyhow::Result<()> {
    let window = initscr();
    window.nodelay(false);
    window.keypad(true);
    noecho();
    let res = wrapped_main(window, PathBuf::new().join("."));
    endwin();
    res
}

fn wrapped_main(window: Window, path: PathBuf) -> anyhow::Result<()> {
    if !has_colors() {
        bail!("Terminal does not support color.")
    }
    start_color();
    init_pair(1, COLOR_BLACK, COLOR_WHITE);

    file_navigator::draw_menu_bars(
        &window,
        1,
        match path.to_str() {
            Some(str) => str,
            None => bail!("Invalid path"),
        },
    );

    let files = file_navigator::get_files(path)?;

    let mut selection = 0;
    let mut scroll = 0;

    file_navigator::draw_file_list(&window, &files, selection, scroll)?;

    loop {
        match window.getch() {
            Some(Input::Character('q')) => {
                break;
            }
            Some(Input::KeyUp) => {
                if selection > 0 {
                    selection -= 1;

                    if selection - scroll < 5 && !selection < 5 {
                        scroll -= 1;
                    }

                    file_navigator::draw_file_list(&window, &files, selection, scroll)?;
                }
            }
            Some(Input::KeyDown) => {
                if selection < files.len() - 1 {
                    selection += 1;

                    if window.get_max_y() - (selection as i32 - scroll as i32 + 2) < 5
                        && files.len() - selection >= 5
                    {
                        scroll += 1;
                    }

                    file_navigator::draw_file_list(&window, &files, selection, scroll)?;
                }
            }
            _ => (),
        }
    }

    Ok(())
}
