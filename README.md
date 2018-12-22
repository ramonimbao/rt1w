# Ray Tracing in One Weekend

A Rust implementation of the book [*Ray Tracing in One Weekend* by Peter Shirley](https://in1weekend.blogspot.com/).

Some parts of the next book *Ray Tracing: The Next Week* also implemented.

<p align="center">
	<img src="output.png" alt="">	
</p>

# Usage

Run the program with `cargo run --release`. You can also give it a JSON configuration file as an argument to change the render settings without having to recompile the program using `--config [config.json]`. Scene loading is also now supported using `--scene [scene.json]`. You can also run the program in single-threaded mode by passing `--single-threaded` for whatever reason. ¯\\\_(ツ)\_/¯

Example:  
`$ rt1w --scene res/scenes/texture-test.json --config res/config/front-good.json`

# Performance

The following are a few stats on renders done on a PC with an i3-2100 CPU. You can see the performance improvement of the multithreading compared to a single thread. I'll add more to this as I do more renders.

| | Multithreaded | Single threaded |  
| --- | --- | --- |
| Cornell box at 800×600 with 10 SPP | 1m 50s | 4m 19s |
| Cornell box at 800×600 with 100 SPP | 21m 05s | 41m 14s |

Here are some stats done on my laptop equipped with an i7-7500U CPU. Seems it's 25% than my old desktop. 

| | Multithreaded | Single threaded |  
| --- | --- | --- |
| Panda Night at 2524×2524 with 25 SPP | 9h 31m | TBD |

# To Do

- Implement being able to input random materials via JSON.
- Implement `--verbose` option input to display all messages.

# License

This project is licensed under the MIT license - see the [LICENSE.md](LICENSE.md) for details.

Image textures obtained from [OpenGameArt: "Seamless Space Rocks Textures Pack (512px)" by mafon2](https://opengameart.org/content/seamless-space-rocks-textures-pack-512px).  
Licensed under: [CC BY 3.0](https://creativecommons.org/licenses/by/3.0/legalcode)

Crate textures obtained from [OpenGameArt: "3 crate textures (w/ bump & normal)" by Luke.RUSTLTD](https://opengameart.org/content/3-crate-textures-w-bump-normal).  
Licensed under [CC 0 (Public Domain)](https://creativecommons.org/publicdomain/zero/1.0/legalcode).

Utah Teapot STL file obtained from [Thingiverse: "Utah teapot" by phooky](https://www.thingiverse.com/thing:821).  
Licensed under [Public Domain](https://creativecommons.org/licenses/publicdomain/).

Stanford Bunny STL file obtained from [Thingiverse: "Low Poly Stanford Bunny" by johnny6](https://www.thingiverse.com/thing:151081).  
Licensed under [CC BY-NC 3.0](https://creativecommons.org/licenses/by-nc/3.0/legalcode).

Panda STL file obtained from [Thingiverse: "Sleeping Panda - dual color" by jkavalik](https://www.thingiverse.com/thing:2968129).  
Licensed under [CC BY-NC-SA 3.0](https://creativecommons.org/licenses/by-nc-sa/3.0/legalcode).