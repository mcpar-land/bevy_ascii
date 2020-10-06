mod camera;
mod buffer;
mod plugin;
mod pos;
mod primitives;
mod render;
mod systems;

use systems::*;

pub use camera::*;
pub use buffer::*;
pub use plugin::*;
pub use pos::*;
pub use primitives::*;
pub use render::*;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
