use std::fs;

const DEV: &str = "/sys/bus/iio/devices/iio:device1";

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
			} else {
			}
		} else {
			if acceleration.y > 0 {
			} else {
			}
		};
		std::thread::sleep_ms(2000);
	}
}
