use crate::sample::{AdsrEnvelope, Sample};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Note {
	pub start_time: f64,
	pub end_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Sequencer {
	notes: Vec<Note>,
	pub envelope: AdsrEnvelope,
	sample_rate: u64,
}

impl Sequencer {
	pub fn new(sample_rate: u64) -> Self {
		Self {
			sample_rate,
			..Default::default()
		}
	}

	pub fn push(&mut self, note: Note) {
		self.notes.push(note);
	}

	pub fn get(&mut self, time: f64) -> Option<&mut Note> {
		self.notes.iter_mut().find(|n| {
			let current_time = time / self.sample_rate as f64;
			current_time >= n.start_time && current_time <= n.end_time
		})
	}

	pub fn get_amplitude(&mut self, time: f64) -> f64 {
		if let Some(_note) = self.get(time) {
			self.envelope.note_on(time / self.sample_rate as f64);
		} else {
			self.envelope.note_off(time / self.sample_rate as f64);
		}

		self.envelope.get(time, self.sample_rate)
	}
}
