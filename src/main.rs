use raudio::{sample::SineOscillator, wave::Wave};
use std::{fs::File, io::Write};

fn main() {
	let mut file = File::create("sine.wav").unwrap();
	let mut wave = Wave::default();

	wave.push(Box::new(SineOscillator {
		..Default::default()
	}));

	file.write_all(&wave.into_wav_file()).unwrap();
}
