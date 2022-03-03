#[macro_use] extern crate log;

use std::path::PathBuf;

use clap::Parser;
use remote_viewing::RemoteViewing;

use crate::vision::Vision;

mod vision;
mod remote_viewing;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Path to vision executable
	#[clap(short = 'v', long = "vision", parse(from_os_str))]
	pub vision_path: PathBuf,

	/// Extra agruments that are passed to the vision program
	/// Arguments should be seperated by space
	#[clap(short = 'a', long = "vision-args", default_value_t = String::from(""))]
	pub vision_args: String,

	/// Host to connect to for mqtt broker
	#[clap(short = 'm', long = "mqtt-host", default_value_t = String::from("localhost"))]
	pub mqtt_host: String,

	/// Port to use when connecting to mqtt broker
	#[clap(short = 'p', long = "mqtt-port", parse(try_from_str), default_value_t = 1883)]
	pub mqtt_port: u16,

	/// Mqtt topic to listen on for commands
	#[clap(short, long, default_value_t = String::from("pi/control"))]
	pub topic: String,

	/// Host to send remote viewing video to
	// for some reason localhost doesn't work here
	#[clap(short = 'r', long = "rtp-host", default_value_t = String::from("127.0.0.1"))]
	pub rtp_host: String,

	/// Port to send remote viewing video over
	#[clap(short = 'o', long = "rtp-port", default_value_t = 5000)]
	pub rtp_port: u16,
}

fn main() {
	env_logger::init();

	let args = Args::parse();

	let mut vision_args = Vec::new();

	for vision_arg in args.vision_args.split(' ') {
		vision_args.push(vision_arg);
	}

	gstreamer::init().expect("failed to initilize gstreamer");

	let mut remote_viewing = RemoteViewing::new(&args.rtp_host, args.rtp_port).unwrap();
	let mut vision = Vision::new(args.vision_path.to_str().unwrap(), &args.mqtt_host, args.mqtt_port, &vision_args);

	remote_viewing.start().unwrap();

	loop {
	}
}
