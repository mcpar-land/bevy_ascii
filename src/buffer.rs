use crate::TermRect;

pub struct Buffer<T = char>
where
	T: Clone,
{
	size: TermRect,
	vals: Vec<Vec<T>>,
}

impl<T> Buffer<T>
where
	T: Clone,
{
	pub fn new(size: TermRect, fill: T) -> Self {
		Self {
			size,
			vals: vec![vec![fill; size.w as usize]; size.h as usize],
		}
	}
	pub fn get(&self, pos: &TermRect) -> Option<&T> {
		self
			.vals
			.get(pos.h as usize)
			.map(|v| v.get(pos.w as usize))
			.flatten()
	}
	pub fn get_pos(&self, pos: crate::Position) -> Option<&T> {
		self.get(&TermRect {
			w: pos.0.x() as u16,
			h: pos.0.y() as u16,
		})
	}
	pub fn get_mut(&mut self, pos: &TermRect) -> Option<&mut T> {
		self
			.vals
			.get_mut(pos.h as usize)
			.map(|v| v.get_mut(pos.w as usize))
			.flatten()
	}
	pub fn get_mut_pos(&mut self, pos: crate::Position) -> Option<&mut T> {
		self.get_mut(&TermRect {
			w: pos.0.x() as u16,
			h: pos.0.y() as u16,
		})
	}
	pub fn set(&mut self, pos: &TermRect, val: T) {
		if let Some(v) = self
			.vals
			.get_mut(pos.h as usize)
			.map(|v| v.get_mut(pos.w as usize))
			.flatten()
		{
			*v = val;
		}
	}
	pub fn size(&self) -> &TermRect {
		&self.size
	}
}

impl Buffer<char> {
	pub fn to_string(&self) -> String {
		let w = self.size().w as usize;
		let h = self.size().h as usize;

		let mut s: String = "".to_string();

		for x in 0..w {
			for y in 0..h {
				s = format!(
					"{}{}",
					s,
					self
						.get(&TermRect {
							w: x as u16,
							h: y as u16
						})
						.unwrap()
				);
			}
			s = format!("{}\n", s);
		}

		s
	}
}
