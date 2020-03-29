# Raytracing in Rust
![header](assets/header.png)  

An implementation of Peter Shirley's Ray Tracing in One Weekend. This project was created with the sole purpose of improving my knowledge of the Rust language.

## How to build
```rust
cargo build
```

## Running the code
```rust
cargo run -- -o [output_file_name] -x [image_x_size] -y [image_y_size]
```
e.g.
```rust
cargo run -- -o output.ppm -x 1000 -y 500
```

## References
[[1] Raytracing in one weekend, Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
