use clap::{App, Arg};
use std::path::{PathBuf};
use litcontainers::*;
use litaudio::*;
use litdsp::*;

fn main() {
	let matches = App::new("LitDSP phase vocoder")
		.version("1.0")
		.author("Egor Dmitriev <egordmitriev2@gmail.com>")
		.about("Speeds up or slows down an audio file")
		.arg(Arg::with_name("input")
			.help("Input audio file")
			.required(true))
		.arg(Arg::with_name("output")
			.short("o")
			.help("Output directory")
			.takes_value(true))
		.arg(Arg::with_name("alpha")
			.short("a")
			.help("Speed scaling constant which goes inverse with the speedup")
			.takes_value(true))
		.get_matches();

	let path = PathBuf::from(matches.value_of("input").unwrap());
	assert!(path.exists(), "Input file doesnt exist!");

	let filename = (&path).file_stem().unwrap().to_str().unwrap().to_string();
	let out_path = matches.value_of("output").map(|s| PathBuf::from(s))
		.unwrap_or(std::env::current_dir().unwrap())
		.join(filename + "_pv.mp3");

	let alpha: f32 = matches.value_of("alpha")
		.map(|v| v.parse().unwrap_or(0.)).unwrap_or(0.);

	let audio: AudioDeinterleaved<f64, U1, Dynamic> = litaudioio::read_audio(&path).unwrap();
	let sr = audio.sample_rate() as f64;
	let result = litdsp::algorithms::calculate_pv(
		&audio,
		sr,
		alpha,
		typenum::consts::U2048::name()
	).into_audio(sr as i32, Deinterleaved);

	litaudioio::write_audio(&out_path, &result).unwrap();
}