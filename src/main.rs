use std::{fs, process::Command};

const DEV: &str = "/sys/bus/iio/devices/iio:device0";

struct Acceleration {
	x: i32,
	y: i32,
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
	loop {
		let acceleration = get_acceleration();
		println!("x:{} y:{}", acceleration.x, acceleration.y);
		if acceleration.x.abs() > acceleration.y.abs() {
			if acceleration.x > 0 {
				Command::new("xrandr")
					.args(["--output", "eDP", "--rotate", "left"])
					.spawn()
					.unwrap();
			} else {
				Command::new("xrandr")
					.args(["--output", "eDP", "--rotate", "right"])
					.spawn()
					.unwrap();
			}
		} else {
			if acceleration.y > 0 {
				Command::new("xrandr")
					.args(["--output", "eDP", "--rotate", "inverted"])
					.spawn()
					.unwrap();
			} else {
				Command::new("xrandr")
					.args(["--output", "eDP", "--rotate", "normal"])
					.spawn()
					.unwrap();
			}
		};
		std::thread::sleep_ms(2000);
	}
}
