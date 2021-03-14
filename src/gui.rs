use sdl2::render::Canvas;
use sdl2::video::Window;
pub use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
pub use sdl2::render::BlendMode;
use sdl2::mouse::RelativeMouseState;

pub struct Draw {
	event_pump: sdl2::EventPump,
	canvas: sdl2::render::Canvas<Window>,
	mosue_state: RelativeMouseState,
}

impl Draw {

	pub fn new(width: u32, height: u32) -> Draw{

		let sdl_context = sdl2::init().unwrap();
		
		// Hide the cursor
		sdl_context.mouse().show_cursor(false);

		let event_pump = sdl_context.event_pump().unwrap();
		let video_subsystem = sdl_context.video();
		
		let mosue_state = RelativeMouseState::new(&event_pump);

		// create a window 
		let window = video_subsystem.unwrap().window("Example", width, height).build().unwrap();
		
		// create a Canvas will use to draw in our Window
		let mut canvas : Canvas<Window> = window.into_canvas()
		    .present_vsync() //< this means the screen cannot
		    // render faster than your display rate (usually 60Hz or 144Hz)
		    .build().unwrap();

		canvas.clear();

		Draw {canvas, event_pump, mosue_state}

	}

	pub fn rect(&mut self, x: i32, y: i32, width: u32, height:u32) {
		self.canvas.fill_rect(Rect::new(x, y, width, height)).unwrap();
		// self.canvas.present();
	}

	pub fn present(&mut self){
		self.canvas.present();	
	}

	pub fn stroke(&mut self, col: Color) {
		self.canvas.set_draw_color(col); //Color::RGB(255, 210, 0));
	}

	pub fn line(&mut self,x1: i32, y1: i32, x2: i32, y2: i32) {
		let point1 = Point::new(x1, y1);
		let point2 = Point::new(x2, y2);

		self.canvas.draw_line(point1, point2).unwrap();
		self.canvas.present();
	}

	pub fn handel_window_events(&mut self) {
		for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => panic!("done"),
                _ => {},
            }
		}
	}

	pub fn clear(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		// fills the canvas with the color we set in `set_draw_color`.
		self.canvas.clear();
	}

	pub fn blend_mode(&mut self, mode: BlendMode) {
		self.canvas.set_blend_mode(mode);
	}

	pub fn is_mouse_pressed(&self) -> bool {
		self.event_pump.mouse_state().left()
	}

	pub fn mouse_x(&self) -> i32 {
		self.event_pump.mouse_state().x()
	}

	pub fn mouse_y(&self) -> i32 {
		self.event_pump.mouse_state().y()
	}
}

// canvas.set_draw_color(Color::RGB(0, 0, 0));
// canvas.clear();

// canvas.set_draw_color(Color::RGB(255, 210, 0));
// canvas.fill_rect(Rect::new(10, 10, 780, 580)).unwrap();

// canvas.set_draw_color(Color::RGB(0, 0, 0));

// for x in 0..800 {
// canvas.draw_point(Point::new(x, 300)).unwrap();		
// }

// canvas.present();
