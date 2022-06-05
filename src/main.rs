use raudio::{
	sample::SineOscillator,
	sequencer::{Note, Sequencer},
	wave::Wave,
};
use std::{fs::File, io::Write, time::Duration};

fn main() {
	let mut file = File::create("sine.wav").unwrap();
	let mut wave = Wave::default();

	let mut sequencer = Sequencer::new(wave.sample_rate);

	sequencer.push(Note {
		start_time: 0.0,
		end_time: 10.0,
	});

	wave.push(
		Box::new(SineOscillator {
			amplitude: Box::new(move |time| sequencer.get_amplitude(time)),
			frequency: Box::new(move |time| {
				29.0 * (time / wave.sample_rate as f64)
			}),
		}),
		Duration::from_secs_f64(10.0),
	);

	file.write_all(&wave.into_wav_file()).unwrap();
}
