
use std::error::Error;
use rustbox::{Color, RustBox};
use rustbox::Key;
use rustbox::Event::*;
use rustbox;

pub fn read_into(start_x: usize, start_y: usize, rustbox: &RustBox, result: &mut String){
	let mut position: usize=0;
	loop{
		match rustbox.poll_event(false) {
			Ok(KeyEvent(key)) => {
				match key {
					Some(Key::Enter) => {
						break;
					}
					Some(Key::Backspace) => {
						if result.len()>0 {
							result.pop();
							position-=1;
							rustbox.print_char(start_x+position, start_y, rustbox::RB_BOLD, Color::Default, Color::Default, ' ');
							rustbox.present();
						}
					}
					Some(Key::Char(v)) => {
						result.push(v);
						rustbox.print_char(start_x+position, start_y, rustbox::RB_BOLD, Color::White, Color::Black, v);
						rustbox.present();
						position+=1;
					}
					_ => { }
				}
			},
			Err(e) => panic!("{}", e.description()),
			_ => { }
		}
	}
}
