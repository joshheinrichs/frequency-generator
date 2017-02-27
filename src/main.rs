extern crate vorbis;
extern crate clap;

use std::io::Write;
use std::f64;
use clap::{Arg, App};

fn main() {
    let matches = App::new("frequency-generator")
                          .version("0.1.0")
                          .author("Josh Heinrichs <joshiheinrichs@gmail.com>")
                          .about("Generatres audio files with specific frequencies")
                          .arg(Arg::with_name("FREQUENCY")
                          	   .help("The frequency of the audio file to generate in Hz")
                          	   .required(true)
                          	   .index(1))
                          .arg(Arg::with_name("length")
                          	   .short("l")
                          	   .long("length")
                          	   .value_name("LENGTH")
                          	   .help("The length of the audio file in seconds")
                          	   .default_value("10.0"))
                          .arg(Arg::with_name("amplitude")
                          	   .short("a")
                          	   .long("amplitude")
                          	   .value_name("AMPLITUDE")
                          	   .help("Loudness of the file between 0 and 1")
                          	   .default_value("0.75"))
                          .arg(Arg::with_name("sample rate")
                          	   .short("r")
                          	   .long("sample-rate")
                          	   .value_name("RATE")
                          	   .help("Number of samples per second")
                          	   .default_value("48000"))
                          .arg(Arg::with_name("output")
                          	   .short("o")
                          	   .long("output")
                          	   .value_name("OUTPUT")
                          	   .help("Name of the output file")
                          	   .default_value("output.ogg"))
                          .get_matches();

    let frequency = matches.value_of("FREQUENCY").unwrap().parse::<f64>().unwrap();
    println!("frequency: {}Hz", frequency);
    let length = matches.value_of("length").unwrap().parse::<f64>().unwrap();
    println!("length: {}s", length);
    let amplitude = matches.value_of("amplitude").unwrap().parse::<f64>().unwrap();
    println!("amplitude: {}", amplitude);
    let sample_rate = matches.value_of("sample rate").unwrap().parse::<u64>().unwrap();
    println!("sample rate: {}", sample_rate);
    let output = matches.value_of("output").unwrap();
    println!("output: {}", output);

    let mut out_file = std::fs::File::create(output).unwrap();
    let mut data = Vec::new();    
    for i in 0..((sample_rate as f64 * length) as i64) {
        let signal = (2.0f64 * f64::consts::PI * i as f64 * frequency / sample_rate as f64).sin();
        let sample = signal * 2.0f64.powf(15.0) * amplitude;
        data.push(sample as i16);
    }

    let mut encoder = vorbis::Encoder::new(1, sample_rate, vorbis::VorbisQuality::Midium).expect("Error in creating encoder");
    out_file.write(encoder.encode(&data).expect("Error in encoding.").as_slice()).expect("Error in writing");
    out_file.write(encoder.flush().expect("Error in flushing.").as_slice()).expect("Error in writing");
}
