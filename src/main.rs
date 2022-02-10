use std::{fs, process::Command, time::Duration};

const DEV: &str = "/sys/bus/iio/devices/iio:device0";

struct Acceleration {
	x: i32,
	y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rotation {
	Left,
	Right,
	Inverted,
	Normal,
}

impl std::fmt::Display for Rotation {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Rotation::Left => write!(f, "left"),
			Rotation::Right => write!(f, "right"),
			Rotation::Inverted => write!(f, "inverted"),
			Rotation::Normal => write!(f, "normal"),
		}
	}
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
		std::thread::sleep(Duration::from_secs(2));
		let acceleration = get_acceleration();
		println!("x:{} y:{}", acceleration.x, acceleration.y);
		if acceleration.x.abs() < 400 && acceleration.y.abs() < 400 {
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
			continue;
		}
		println!("rotate display to {}", rotation);
		Command::new("xrandr")
			.args(["--output", "eDP", "--rotate", rotation.to_string().as_str()])
			.spawn()
			.unwrap();
		last_rotation = Some(rotation);
	}
}
