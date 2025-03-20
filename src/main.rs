use crossterm::{
	cursor::{Hide, MoveTo},
	event::{Event, KeyCode, KeyEventKind},
	execute,
	style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
	terminal::{Clear, ClearType},
};
use std::{array, fs, io::StdoutLock};
use std::collections::HashSet;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct Board {
	cells: [char; 25],
}

impl Board {
	fn new() -> Self {
		Self {
			cells: array::from_fn(|index| (index as u8 + 65) as char),
		}
	}
	
	fn update(&mut self, ptr_x: usize, ptr_y: usize, mut r_char: char) {
		let index: usize = ptr_y * 5 + ptr_x;
		r_char = r_char.to_ascii_uppercase();
		self.cells[index] = r_char;
	}

	fn display(&self, out: &mut StdoutLock, ptr_x: usize, ptr_y: usize) -> anyhow::Result<()> {
		execute!(out, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;

		for (index, char) in self.cells.iter().enumerate() {
			let index_x = index % 5;
			let index_y = index / 5;

			if index_x == ptr_x && index_y == ptr_y {
				execute!(
					out,
					SetBackgroundColor(Color::White),
					SetForegroundColor(Color::Black),
					Print(char),
					ResetColor
				)?;
			} else {
				execute!(out, Print(char))?;
			}

			if (index + 1) % 5 == 0 {
				execute!(out, Print('\n'))?;
			} else {
				execute!(out, Print(' '))?;
			}
		}
		
		Ok(())
	}
}

fn load_dictionary(path: PathBuf) -> anyhow::Result<HashSet<String>> {
	let dictionary = BufReader::new(fs::OpenOptions::new().read(true).open(path)?);
	let mut output: HashSet<String> = HashSet::new();
	
	for line in dictionary.lines() {
		output.insert(line?);
	}
	
	Ok(output)
}

fn main() -> anyhow::Result<()> {
	let mut board = Board::new();
	
	let mut changed = true;
	let mut stdout = std::io::stdout().lock();
	
	let dict = load_dictionary(std::env::current_dir()?.join("dictionary.txt"));

	let mut pointer_x: usize = 0;
	let mut pointer_y: usize = 0;

	execute!(stdout, Hide)?;

	loop {
		if changed {
			changed = false;

			board.display(&mut stdout, pointer_x, pointer_y)?;
		}

		match crossterm::event::read()? {
			Event::Key(key) => {
				let code = key.code;
				let kind = key.kind;

				match kind {
					KeyEventKind::Press => match code {
						KeyCode::Enter => {}
						KeyCode::Left => {
							pointer_x = pointer_x.saturating_sub(1).max(0);
							changed = true;
						}
						KeyCode::Right => {
							pointer_x = (pointer_x + 1).min(4);
							changed = true;
						}
						KeyCode::Up => {
							pointer_y = pointer_y.saturating_sub(1).max(0);
							changed = true;
						}
						KeyCode::Down => {
							pointer_y = (pointer_y + 1).min(4);
							changed = true;
						}
						KeyCode::Char(c) => {
							board.update(pointer_x, pointer_y, c);
							changed = true;
						}
						_ => {}
					},
					_ => {}
				}
			}
			_ => {}
		}
	}
}
