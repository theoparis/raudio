#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait Sample {
	fn get(&mut self, time: f64, sample_rate: u64) -> f64;
}

pub struct SineOscillator {
	pub frequency: Box<dyn FnMut(f64) -> f64 + Send + Sync>,
	pub amplitude: Box<dyn FnMut(f64) -> f64 + Send + Sync>,
}

impl Default for SineOscillator {
	fn default() -> Self {
		Self {
			frequency: Box::new(|_t| 440.0),
			amplitude: Box::new(|_t| 0.5),
		}
	}
}

impl Sample for SineOscillator {
	fn get(&mut self, time: f64, sample_rate: u64) -> f64 {
		let freq = (self.frequency)(time);
		let w = 2.0 * std::f64::consts::PI * freq * time as f64;
		let s = f64::sin(w / (sample_rate as f64));
		let amp = (self.amplitude)(time);
		f64::floor(255.0 * (amp * s + amp))
	}
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AdsrEnvelope {
	pub attack_time: f64,
	pub decay_time: f64,
	pub release_time: f64,

	pub sustain_amplitude: f64,
	pub start_amplitude: f64,
	pub trigger_on_time: f64,
	pub trigger_off_time: f64,

	note_on: bool,
}

impl Default for AdsrEnvelope {
	fn default() -> Self {
		Self {
			attack_time: 0.01,
			decay_time: 0.01,
			start_amplitude: 1.0,
			sustain_amplitude: 0.8,
			release_time: 0.02,
			trigger_on_time: 0.0,
			trigger_off_time: 0.0,
			note_on: false,
		}
	}
}

impl AdsrEnvelope {
	pub fn note_on(&mut self, time: f64) {
		self.trigger_on_time = time;
		self.note_on = true;
	}

	pub fn note_off(&mut self, time: f64) {
		self.trigger_off_time = time;
		self.note_on = false;
	}
}

impl Sample for AdsrEnvelope {
	fn get(&mut self, time: f64, _sample_rate: u64) -> f64 {
		let lifetime = time - self.trigger_on_time;
		let mut amplitude = 0.0;

		if self.note_on {
			// ADS

			// Attack
			if lifetime <= self.attack_time {
				amplitude =
					(lifetime / self.attack_time) * self.start_amplitude;
			}

			// Decay
			if lifetime > self.decay_time
				&& lifetime <= (self.attack_time * self.decay_time)
			{
				amplitude = ((lifetime - self.attack_time) / self.decay_time)
					* (self.sustain_amplitude - self.start_amplitude)
					+ self.start_amplitude;
			}

			// Sustain
			if lifetime > (self.attack_time + self.decay_time) {
				amplitude = self.sustain_amplitude;
			}
		} else {
			// Release
			amplitude = ((time - self.trigger_off_time) / self.release_time)
				* (0.0 - self.sustain_amplitude)
				+ self.sustain_amplitude;
		}

		if amplitude <= 0.0001 {
			amplitude = 0.0;
		}

		amplitude
	}
}
