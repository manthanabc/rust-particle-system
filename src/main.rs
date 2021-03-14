mod gui;
mod matrix;
mod particle;

extern crate rand;
use crate::particle::Particle;
use crate::matrix::Matrix;
use gui::*;

fn main() {
	let mut draw = Draw::new(400, 400);
	draw.stroke(Color::RGB(255, 255, 255));

	let mut particles: Vec<Particle> = vec![];

	// for _i in 0..1000 {
	// 	let position = Matrix {  arr: vec![vec![100.0], vec![100.0]]  };
	// 	let velocity = Matrix {  arr: vec![vec![rand::random::<f32>() * 100.0], vec![rand::random::<f32>() * 100.0]]  };
	// 	let p = Particle::new(position, velocity, 20);
	// 	particles.push(p);
	// }

	'draw: loop {
		draw.blend_mode(BlendMode::Add);
		draw.clear();
		draw.stroke(Color { r: 20, g: 20, b: 20 , a:20});

		for _i in 0..2 {
			let position = Matrix {  arr: vec![vec![draw.mouse_x() as f32], vec![draw.mouse_y() as f32]]  };
			let velocity = Matrix {  arr: vec![vec![(rand::random::<f32>() * 10.0) - 5.0], vec![(rand::random::<f32>() * 10.0) - 5.0]]  };
			let p = Particle::new(position, velocity, 10);
			particles.push(p);
		}

		for p in particles.iter_mut() {
			p.update();
			p.show(&mut draw);
		}

		// println!("{:?}", draw.mouse_y());

		// remove dead particles 
		particles = particles.into_iter().filter(|x| x.lifetime > 0 ).collect();

		draw.handel_window_events();
		draw.present();
	}
}