/*
* Definitions that prob don't belong in main.
*/

use std::{io::{Cursor, BufReader}, thread};

use rand::seq::SliceRandom;
use rand::thread_rng;
use rodio::{OutputStream, Decoder, Source};
use windows_volume_control::AudioController;

use crate::{SUSSY_AUDIO, DELAY_PER_MEME_MUSIC};

pub type Result<T, E = Box<dyn std::error::Error>> = core::result::Result<T, E>;
pub type SyncResult<T, E = Box<dyn std::error::Error + Send + Sync>> = Result<T, E>;

pub fn choose_rand<T: Copy, V: AsRef<[T]>>(v: V) -> Option<T> {
	v.as_ref().choose(&mut thread_rng()).map(|value| *value)
}

pub fn play_meme_audio() -> SyncResult<()> {
	// RIP HEADPHONE USERS
	let _ = set_volume_to(100.0);

	// Get a output stream handle to the default physical sound device
	let (_stream, stream_handle) = OutputStream::try_default()?;

	let meme_audio = choose_rand(SUSSY_AUDIO.to_vec()).ok_or_else(
		|| "Could not load meme music... Literally 1984."
	)?;

	let meme_audio = BufReader::new(Cursor::new(meme_audio));

	// Decode that sound file into a source
	let source = Decoder::new(meme_audio)?;

	// Play the sound directly on the device
	stream_handle.play_raw(source.convert_samples())?;
	thread::sleep(std::time::Duration::from_secs(DELAY_PER_MEME_MUSIC));

	Ok(())
}

pub fn set_volume_to(vol: f32) -> Result<()> {
	unsafe { 
        let mut controller = AudioController::init();
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();

        let master = controller.get_session_by_name("master".to_string());
		let master = master.ok_or_else(
			|| "Could not get master channel"
		)?;

		master.setVolume(vol / 100.0);
    };

	Ok(())
}
