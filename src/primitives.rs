#[derive(Debug, Clone, Copy)]
pub struct TermRect {
	/// Width
	pub w: i32,
	/// Height
	pub h: i32,
}

impl TermRect {
	pub fn add(&self, val: i32) -> TermRect {
		TermRect {
			w: self.w + val,
			h: self.h + val,
		}
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
		self + (rhs * -1)
	}
}

impl std::ops::Add<i32> for TermRect {
	type Output = TermRect;

	fn add(self, rhs: i32) -> Self::Output {
		TermRect {
			w: self.w + rhs,
			h: self.h + rhs,
		}
	}
}

impl std::ops::Sub<i32> for TermRect {
	type Output = TermRect;

	fn sub(self, rhs: i32) -> Self::Output {
		TermRect {
			w: self.w - rhs,
			h: self.h - rhs,
		}
	}
}

impl std::ops::Div<i32> for TermRect {
	type Output = TermRect;

	fn div(self, rhs: i32) -> Self::Output {
		TermRect {
			w: self.w / rhs,
			h: self.h / rhs,
		}
	}
}

impl std::ops::Mul<i32> for TermRect {
	type Output = TermRect;

	fn mul(self, rhs: i32) -> Self::Output {
		TermRect {
			w: self.w * rhs,
			h: self.h * rhs,
		}
	}
}
