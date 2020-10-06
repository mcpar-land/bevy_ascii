A library for making terminal-based games in the style of Dwarf Fortress.

# Examples

- [`basic`](examples/basic.rs)
- [`depth_buffer`](examples/depth_buffer.rs)
- [`keyboard`](examples/keyboard.rs)

# To do

- [ ] **Performance!** Right now performance is really awful. Using the GPU with [emu](https://github.com/calebwin/emu) is the goal.
- [ ] Write documentation
- [ ] Add 'background inheritance,' where the nearest background color is applied if there are characters above it with no background set.
- [ ] add more examples to test complex functionality
  - [ ] multiple cameras on-screen at once
  - [ ] camera culling based on the camera's Z location
- [ ] Add some way to translate Bevy's UI system into terminal interface components
- [x] [Fix a bug with keyboard input](https://github.com/bevyengine/bevy/issues/636)
