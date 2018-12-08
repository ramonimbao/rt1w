# Ray Tracing in a Weekend

A Rust implementation of the book [*Ray Tracing in a Weekend* by Peter Shirley](https://in1weekend.blogspot.com/).

Some parts of the next book *Ray Tracing: The Next Week* also implemented.

![](output.png)

# Usage

Run the program with `cargo run --release`. You can also give it a JSON configuration file as an argument to change the render settings without having to recompile the program using `--config [config.json]`. Scene loading is also now supported using `--scene [scene.json]`.

Example:  
`$ rt1w --scene json/scenes/two-balls.json --config json/config/github_sample.json`

# To Do

- Implement moving sphere loading into scene via JSON
- Implement loading checkerboard pattern into scene via JSON

# License

This project is licensed under the MIT license - see the [LICENSE.md](LICENSE.md) for details.
