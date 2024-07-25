# 🎨 Color Quantization in Rust

I've implemented the octree method for image color quantization using Rust.

Instead of using an octree, I use a `Map<String, [count, r, g, b]>` to simplify the logic of reducing colors.

A `depth` parameter is used to control the maximum number of colors: 
- `depth = 1` results in a maximum of 8 colors
- `depth = 2` results in a maximum of 64 colors
- `depth = 3` results in a maximum of 256 colors
- ...

> There is a lot of room for improvement and optimization. I created this project for fun and to learn Rust. I enjoy working with images!

## Usage

```sh
cargo run <image> <depth>
```

For better performance with large images, build in release mode:

```sh
cargo build --release
```

---

