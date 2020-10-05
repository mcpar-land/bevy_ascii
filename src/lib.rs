mod camera;
mod plugin;
mod pos;
mod primitives;
mod systems;
mod term_char;

use systems::*;

pub use camera::*;
pub use plugin::*;
pub use pos::*;
pub use primitives::*;
pub use term_char::*;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
