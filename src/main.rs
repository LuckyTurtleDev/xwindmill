use std::{fs, process::Command, thread::sleep, time::Duration};
use strum_macros::Display;

const DEV: &str = "/sys/bus/iio/devices/iio:device0";

struct Acceleration {
	x: i32,
	y: i32,
}

#[derive(Debug, Clone, Copy, Display, PartialEq)]
#[strum(serialize_all = "lowercase")]
enum Rotation {
	Left,
	Right,
	Inverted,
	Normal,
}

fn get_acceleration() -> Acceleration {
	let mut string = fs::read_to_string(format!("{DEV}/in_accel_x_raw")).unwrap();
	string.pop();
	let x = string.parse().unwrap();
	let mut string = fs::read_to_string(format!("{DEV}/in_accel_y_raw")).unwrap();
	string.pop();
	let y = string.parse().unwrap();
	Acceleration { x, y }
}

fn main() {
	let mut last_rotation: Option<Rotation> = None;
	loop {
		let acceleration = get_acceleration();
		println!("x:{} y:{}", acceleration.x, acceleration.y);
		if acceleration.x.abs() < 400 && acceleration.y.abs() < 400 {
			sleep(Duration::from_secs(2));
			continue;
		}
		let rotation = if acceleration.x.abs() > acceleration.y.abs() {
			if acceleration.x > 0 {
				Rotation::Left
			} else {
				Rotation::Right
			}
		} else {
			if acceleration.y > 0 {
				Rotation::Inverted
			} else {
				Rotation::Normal
			}
		};
		if Some(rotation) == last_rotation {
			sleep(Duration::from_secs(2));
			continue;
		}
		println!("rotate display to {}", rotation);
		Command::new("xrandr")
			.args(["--output", "eDP", "--rotate", rotation.to_string().as_str()])
			.spawn()
			.unwrap();
		last_rotation = Some(rotation);
		sleep(Duration::from_secs(2));
		for input in vec!["Wacom HID 52B0 Pen Pen (0x8061bce5)", "Wacom HID 52B0 Finger"] {
			Command::new("xinput").args(["map-to-output", input, "eDP"]).spawn().unwrap();
		}
	}
}
