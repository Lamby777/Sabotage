/*
* Constant values, mostly config stuff
*/

use lazy_static::lazy_static;

pub const DELAY_PER_INSTANCE_SPAWN:	u64 = 120;
pub const DELAY_PER_MEME_MUSIC:		u64 = 240;

// Title bar text options (picked randomly)
pub const WINDOW_TITLES: &[&str] = &[
	"[OK] [Cancel]",
	"We have detected a trojan spyware on this PC!",
	"And now, a word from our sponsors...",
	"DDoS traffic detected from `127.0.0.1`",
	"Sh33p Al3rt",
	"Annoying Adware",
	"Memester Trojan",
	"Free V-Bucks.exe",
	"Kid Named Malware:",
	"Definitely Not The Flag",
	"Literally 1984",
	"uwu",
	"This CTF was brought to you by:",
	"free robux no scam working 2023 legit no virus no human verification",
	"We'd like to reach out about your car's extended warranty",
	"Most Epic Executable",
	"Ampersand Tick A Mute :)",
	"Libre Malbolge Interpreter v1.3.37",
	"Sussy PowerShell script that you copied off of GitHub Gists",
	"cascade@delta5: bash - node19",
];

#[derive(Clone, Copy)]
pub enum Shitpost {
	Small(&'static [u8]),
	Large(&'static [u8])
}

impl Shitpost {
	pub fn get_bytes(&self) -> &[u8] {
		match &self {
			Shitpost::Large(v)	=> v,
			Shitpost::Small(v)	=> v,
		}
	}
}

lazy_static! {
	pub static ref SUSSY_AUDIO:	Vec<&'static [u8]> = vec![
		include_bytes!("../music/amogusdrip.mp3"),
		include_bytes!("../music/spooder.mp3"),
		include_bytes!("../music/badpiggiesdrip.mp3"),
		include_bytes!("../music/bone-trooper.mp3"),
		include_bytes!("../music/otamatone-saul.mp3"),
		include_bytes!("../music/rickroll64.mp3"),
	];

	pub static ref SHITPOSTS:	Vec<Shitpost> = vec![
		// i should probably write a macro to condense all this stuff...
		// PRAISE MIDDLE CLICK!
		Shitpost::Large(include_bytes!("../memes/faze-yuri.png")),
		Shitpost::Large(include_bytes!("../memes/kit-hoodie.jpg")),
		Shitpost::Large(include_bytes!("../memes/kit-hoodie-2.jpg")),
		Shitpost::Large(include_bytes!("../memes/mickeyrobux.png")),
		Shitpost::Large(include_bytes!("../memes/removed.png")),
		Shitpost::Large(include_bytes!("../memes/sam-4090.png")),
		Shitpost::Large(include_bytes!("../memes/s3rtxsthumb.png")),
		Shitpost::Large(include_bytes!("../memes/robot.png")),
		Shitpost::Large(include_bytes!("../memes/ruhan.png")),
		Shitpost::Large(include_bytes!("../memes/asusamogus.png")),
		Shitpost::Large(include_bytes!("../memes/tom404.png")),
		Shitpost::Large(include_bytes!("../memes/sussyfiles.png")),
		Shitpost::Large(include_bytes!("../memes/sussyfiles2.png")),
		Shitpost::Large(include_bytes!("../memes/quacc-bread.png")),
		Shitpost::Large(include_bytes!("../memes/duccflash.png")),
		Shitpost::Small(include_bytes!("../memes/noflags.png")),
		Shitpost::Small(include_bytes!("../memes/kitkatz.jpg")),
		Shitpost::Small(include_bytes!("../memes/yippee.jpg")),
		Shitpost::Small(include_bytes!("../memes/yippee2.jpg")),
		Shitpost::Small(include_bytes!("../memes/bunnee.jpg")),
		Shitpost::Small(include_bytes!("../memes/amogusdoc.png")),
		Shitpost::Small(include_bytes!("../memes/tomargus.jpg")),
		Shitpost::Small(include_bytes!("../memes/yerr.png")),
		Shitpost::Small(include_bytes!("../memes/bsd-drip.png")),
		Shitpost::Small(include_bytes!("../memes/tux-drip.webp")),
		Shitpost::Small(include_bytes!("../memes/nukes.png")),
		Shitpost::Small(include_bytes!("../memes/localhost.png")),
		Shitpost::Small(include_bytes!("../memes/catlag.png")),
		Shitpost::Small(include_bytes!("../memes/ronaldosip.png")),
		Shitpost::Small(include_bytes!("../memes/jermaburger.png")),
		Shitpost::Small(include_bytes!("../memes/slowpoke.png")),
		Shitpost::Small(include_bytes!("../memes/lusa-cs.png")),
		Shitpost::Small(include_bytes!("../memes/ye.png")),
		Shitpost::Small(include_bytes!("../memes/doofus.png")),
		Shitpost::Small(include_bytes!("../memes/serena-sigint.png")),
		Shitpost::Small(include_bytes!("../memes/kat.webp")),
		Shitpost::Small(include_bytes!("../memes/ipee.gif")),
		Shitpost::Small(include_bytes!("../memes/bandito.png")),
		Shitpost::Small(include_bytes!("../memes/jerma.png")),
		Shitpost::Small(include_bytes!("../memes/pigmask-sheep.png")),
		Shitpost::Small(include_bytes!("../memes/rickey.png")),
		Shitpost::Small(include_bytes!("../memes/sans.png")),
		Shitpost::Small(include_bytes!("../memes/weebux.png")),
		Shitpost::Small(include_bytes!("../memes/xenia-mc.png")),
		Shitpost::Small(include_bytes!("../memes/ferris-drip.png")),
		Shitpost::Small(include_bytes!("../memes/spongefn.jpg")),
		Shitpost::Small(include_bytes!("../memes/space-ex.png")),
		Shitpost::Small(include_bytes!("../memes/bebe.png")),
		Shitpost::Small(include_bytes!("../memes/clownery.png")),
		Shitpost::Small(include_bytes!("../memes/n3rd.jpg")),
	];
}
