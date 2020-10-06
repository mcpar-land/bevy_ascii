/// Simple `w` x `h` primitive for screen-space coordinates
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TermRect {
	/// Width
	pub w: u16,
	/// Height
	pub h: u16,
}

impl TermRect {
	pub fn is_inside(&self, container: &TermRect) -> bool {
		self.w <= container.w && self.h <= container.h
	}
}

impl std::ops::Add for TermRect {
	type Output = TermRect;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			w: self.w + rhs.w,
			h: self.h + rhs.h,
		}
	}
}

impl std::ops::Sub for TermRect {
	type Output = TermRect;

	fn sub(self, rhs: Self) -> Self::Output {
		TermRect {
			w: self.w - rhs.w,
			h: self.h - rhs.h,
		}
	}
}

impl std::ops::Add<u16> for TermRect {
	type Output = TermRect;

	fn add(self, rhs: u16) -> Self::Output {
		TermRect {
			w: self.w + rhs,
			h: self.h + rhs,
		}
	}
}

impl std::ops::Sub<u16> for TermRect {
	type Output = TermRect;

	fn sub(self, rhs: u16) -> Self::Output {
		TermRect {
			w: self.w - rhs,
			h: self.h - rhs,
		}
	}
}

impl std::ops::Div<u16> for TermRect {
	type Output = TermRect;

	fn div(self, rhs: u16) -> Self::Output {
		TermRect {
			w: self.w / rhs,
			h: self.h / rhs,
		}
	}
}

impl std::ops::Mul<u16> for TermRect {
	type Output = TermRect;

	fn mul(self, rhs: u16) -> Self::Output {
		TermRect {
			w: self.w * rhs,
			h: self.h * rhs,
		}
	}
}
