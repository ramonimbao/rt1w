# Ray Tracing in One Weekend

A Rust implementation of the book [*Ray Tracing in One Weekend* by Peter Shirley](https://in1weekend.blogspot.com/).

Some parts of the next book *Ray Tracing: The Next Week* also implemented.

<!--<p align="center">
	<img src="output.png" alt="">	
</p>-->

# Usage

Run the program with `cargo run --release`. You can also give it a JSON configuration file as an argument to change the render settings without having to recompile the program using `--config [config.json]`. Scene loading is also now supported using `--scene [scene.json]`.

Example:  
`$ rt1w --scene res/scenes/texture-test.json --config res/config/front-good.json`

Rendering the default scene with the `good.json` config (800×600 at 100 samples/pixel) takes almost 19 minutes.

# To Do

- Implement being able to input random materials via JSON.
- Implement `--verbose` option input to display all messages.
- Implement Triangle shape.
- Implement OBJ model loading.

# License

This project is licensed under the MIT license - see the [LICENSE.md](LICENSE.md) for details.

Image textures obtained from [OpenGameArt "Seamless Space Rocks Textures Pack (512px)" by mafon2](https://opengameart.org/content/seamless-space-rocks-textures-pack-512px).  
Licensed under: [CC BY 3.0](https://creativecommons.org/licenses/by/3.0/legalcode)

Crate textures obtained from [OpenGameArt "3 crate textures (w/ bump & normal)" by Luke.RUSTLTD](https://opengameart.org/content/3-crate-textures-w-bump-normal).  
Licensed under [CC 0 (Public Domain)](https://creativecommons.org/publicdomain/zero/1.0/legalcode).