# Ascii Graphics
A Rust library for drawing ascii graphics.

## Usage
Add `ascii_graphics` as a dependency in `Cargo.toml`:

```
[dependencies]
ascii_graphics = "0.1.0"
```

## Quick Example
```
use ascii_graphics::*;
let mut window = screen::create(10, 10);
window
	.background(' ')
	.border(border::settings('+', '-', '|'))
	.stroke('0')
	.line(2, 6, 6, 2)
	.text("hello", 2, 8)
	.print();
// +--------+
// |        |
// |     0  |
// |    0   |
// |   0    |
// |  0     |
// | 0      |
// |        |
// | hello  |
// +--------+
```

## Future Features
* [ ] Support for non-stdout streams
* [ ] Triangles and n-sided polygons
* [ ] Color output for output streams that support it
* 