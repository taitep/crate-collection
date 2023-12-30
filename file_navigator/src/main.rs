use anyhow::{self, bail};
use file_navigator;
use std::path::PathBuf;

use pancurses::{
    endwin, has_colors, init_pair, initscr, start_color, Window, COLOR_BLACK, COLOR_WHITE,
};

fn main() -> anyhow::Result<()> {
    let window = initscr();
    window.nodelay(false);
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

    window.getch();

    Ok(())
}
