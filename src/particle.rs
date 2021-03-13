use crate::gui::*;
use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Particle {
	position: Matrix,
	velocity: Matrix,
	size: u32,
	pub lifetime: i32
}

impl Particle {

	pub fn new(position: Matrix, velocity: Matrix, size: u32) -> Particle {
		Particle {
			position,
			velocity,
			size,
			lifetime: 255
		}
	}

	pub fn update(&mut self) {
		self.position = self.position.add(&self.velocity);
		self.velocity = self.velocity.add(&Matrix {
			arr: vec![vec![0.0], vec![0.5]],
		});

		self.lifetime = self.lifetime-7;

		if self.lifetime < 0 {
			self.lifetime = 0 ;
		}

		// if self.position.arr[1][0] > 200.0 {
		// 	// self.position.arr[0][0]
		// 	self.velocity = self.velocity.dot(&Matrix { arr: vec![vec![0.0],vec![-0.7]]});
		// }
	}

	pub fn show(&self, canvas: &mut Draw) {
		canvas.stroke(Color { r: 200, g: 200, b: 20 , a:self.lifetime as u8});
		canvas.rect(self.position.arr[0][0] as i32, self.position.arr[1][0]as i32, self.size, self.size);
	}
}