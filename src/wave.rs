#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::sample::Sample;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Wave {
	pub sample_rate: u64,
	data: Vec<u8>,
}

impl Wave {
	pub fn push(&mut self, sample: Box<dyn Sample + Send + Sync>) {
		self.data.extend(
			sample
				.get(self.sample_rate)
				.iter()
				.map(|s| *s as u8)
				.collect::<Vec<u8>>(),
		);
	}

	pub fn into_wav_file(&self) -> Vec<u8> {
		let mut buf = Vec::new();
		buf.extend(b"RIFF");
		buf.extend(make_u32(20 + self.data.len() as u32));
		buf.extend(b"WAVE");
		buf.extend(b"fmt ");
		// fmt chunk size
		buf.extend(make_u32(16));
		// format code (PCM)
		buf.extend(make_usize(1));
		// number of channels
		buf.extend(make_usize(1));
		// sample rate
		buf.extend(make_u32(self.sample_rate as u32));
		// data rate
		buf.extend(make_u32(self.sample_rate as u32));
		// data block size
		buf.extend(make_usize(1));
		// bits per sample
		buf.extend(make_usize(8));

		buf.extend(b"data");
		buf.extend(make_u32(self.data.len() as u32));
		buf.extend(self.data.clone());

		buf
	}
}

impl Default for Wave {
	fn default() -> Self {
		Self {
			sample_rate: 44100,
			data: Vec::new(),
		}
	}
}

pub fn make_u32(v: u32) -> Vec<u8> {
	[0u8; 4]
		.iter()
		.enumerate()
		.map(|(i, _)| ((v >> (8 * i)) & 0xff) as u8)
		.collect()
}

pub fn make_usize(v: usize) -> Vec<u8> {
	[0u8; 2]
		.iter()
		.enumerate()
		.map(|(i, _)| ((v >> (8 * i)) & 0xff) as u8)
		.collect()
}
