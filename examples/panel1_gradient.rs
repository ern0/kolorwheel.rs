use crate::hsl::Hsl;

pub struct P1Gradient {
    color1: Hsl,
    color2: Hsl,
}

impl P1Gradient {
	
	pub fn new() -> Self {
		Self {
			color1: Hsl { h: 0, s: 100, l: 50 },
			color2: Hsl { h: 270, s: 70, l: 30 },
		}
	}

	pub fn hello(&mut self) {
		println!("hello");
	}

}
