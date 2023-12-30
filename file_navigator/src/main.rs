use anyhow::{self, bail};
use file_navigator;
use scopeguard;
use std::env;
use std::path::PathBuf;

use pancurses::{
    endwin, has_colors, init_pair, initscr, noecho, start_color, Input, COLOR_BLACK, COLOR_WHITE,
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        bail!("Too many arguments.")
    }

    let mut path: PathBuf = PathBuf::new().join(".");
    if args.len() > 1 {
        path = PathBuf::from(args[1].clone());
    }

    let window = initscr();

    let _guard = scopeguard::guard((), |()| {
        endwin();
    });

    window.nodelay(false);
    window.keypad(true);
    noecho();
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

    let mut files = file_navigator::get_files(&path)?;

    let mut selection = 0;
    let mut scroll = 0;

    file_navigator::draw_file_list(&window, &files, selection, scroll)?;

    loop {
        match window.getch() {
            Some(Input::Character('q')) => {
                break;
            }
            Some(Input::KeyUp) => {
                if selection > 0 && files.len() > 0 {
                    selection -= 1;

                    if selection - scroll < 5 && selection >= 5 {
                        scroll -= 1;
                    }

                    file_navigator::draw_file_list(&window, &files, selection, scroll)?;
                }
            }
            Some(Input::KeyDown) => {
                if files.len() > 0 {
                    if selection < files.len() - 1 {
                        selection += 1;

                        if window.get_max_y() - (selection as i32 - scroll as i32 + 3) < 5
                            && files.len() - selection >= 5
                        {
                            scroll += 1;
                        }

                        file_navigator::draw_file_list(&window, &files, selection, scroll)?;
                    }
                }
            }
            Some(Input::Character('\n')) => {
                if files.len() > 0 {
                    if files[selection].file_type()?.is_dir() {
                        path = path.join(files[selection].file_name());
                        files = file_navigator::get_files(&path)?;

                        selection = 0;
                        scroll = 0;

                        file_navigator::draw_file_list(&window, &files, selection, scroll)?;
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}
