#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub trait Sample {
	fn get_duration(&self) -> Duration;
	fn get(&self, sample_rate: u64) -> Vec<f64>;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SineOscillator {
	pub duration: Duration,
	pub amplitude: f64,
	pub frequency: f64,
}

impl Default for SineOscillator {
	fn default() -> Self {
		Self {
			duration: Duration::from_secs(1),
			amplitude: 0.5,
			frequency: 440.0,
		}
	}
}

impl Sample for SineOscillator {
	fn get_duration(&self) -> Duration {
		self.duration
	}

	fn get(&self, sample_rate: u64) -> Vec<f64> {
		(0..self.duration.as_secs() * sample_rate)
			.map(|t| {
				let w = 2.0 * std::f64::consts::PI * self.frequency * t as f64;
				let s = f64::sin(w / (sample_rate as f64));
				f64::floor(255.0 * (self.amplitude * s + self.amplitude))
			})
			.collect::<Vec<f64>>()
	}
}
