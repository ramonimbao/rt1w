# Making a Scene

The following is a list of possible key-value pairs you can enter to create scenes.  
Any time there is a `<number>`, you can input a number of a string formatted as `"a,b"` which will give you a random value in the range of [a, b).

Make sure not to have any trailing commas after the last values.

## Skybox 

Creates a lit skybox around the entire scene

```
skybox: {
	"r": "&lt;number&gt;",
	"g": "&lt;number&gt;",
	"b": "&lt;number&gt;"
}
```

## Materials and Textures

This is the possible key-value pairs you can input that's common to all objects in the scene.

### Lambertian and Metal

Use `"matte/<type>"` for a matte diffuse material and `"metal/<type>"` for a glossy material. For metals, a `fuzz` key that takes a number can be added inside `material` to adjust how shiny the metal is.

#### Constant Texture

```
	"material": {
		"type": "&lt;matte or metal&gt;/constant",
		"color": {
			"r": "&lt;number&gt;",
			"g": "&lt;number&gt;",
			"b": "&lt;number&gt;"
		}
	}
```

#### Checkered Texture

`scale` defaults to 1 if no value is set.

```
	"material": {
		"type": "&lt;matte or metal&gt;/checkered",
		"colors": [
			{
				"r": "&lt;number&gt;",
				"g": "&lt;number&gt;",
				"b": "&lt;number&gt;"
			},
			{
				"r": "&lt;number&gt;",
				"g": "&lt;number&gt;",
				"b": "&lt;number&gt;"
			}
		],
		"scale": "&lt;number&gt;" (OPTIONAL)
	}
```

#### Image Texture

(From the `image` crate) supported image formats are: PNG, JPEG, GIF, BMP, ICO, TIFF, Webp, and PNM. Alpha channels are not supported; they'll appear black! A wrong path to an image will give you a black and magenta texture and warn you instead of crashing the program.

```
	"material": {
		"type": "&lt;matte or metal&gt;/image",
		"filename": "&lt;path/to/image&gt;"
		"scale": "&lt;number&gt;" (OPTIONAL)
	}
```

#### Noise Texture

Uses Perlin noise.

```
	"material": {
		"type": "&lt;matte or metal&gt;/noise",
		"scale": "&lt;number&gt;" "&lt;number&gt;" (OPTIONAL)
	}
```

### Light

For some reason, values above 1 give an ugly result. I suggest adjusting the size of the object rather than the intensity of the light if you want it brighter.

```
	"material": {
		"type": "light",
		"color": {
			"r": "&lt;number&gt;",
			"g": "&lt;number&gt;",
			"b": "&lt;number&gt;"
		}
	}
```

### Dielectric (Glass)

See [https://en.wikipedia.org/wiki/List\_of\_refractive_indices](https://en.wikipedia.org/wiki/List_of_refractive_indices)

```
	"material": {
		"type": "dielectric",
		"refractive_index": "&lt;number&gt;"
	}
```

## Objects

Some objects support a `density` key that accepts a number that will allow you to convert it to a volume object.  
All objects also support a `copies` key which is useful for example generating a scene with 250 spheres with random positions. (See `panda-night.json` in the `scenes` folder for an example).

### Rotation

Rotation is available for some objects. Completely optional.

```
"&lt;object&gt;": [
	{
		...
		"rotation:" {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		}
	}
]
```

### Spheres
```
"spheres": [
	{
		"position": {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		},
		"radius": "&lt;number&gt;"
		"material": { ... },
		"density": "&lt;number&gt;", (OPTIONAL)
		"copies": "&lt;number&gt;" (OPTIONAL)
	},

	...

	{
		...
	}
]
```

### Moving Spheres

Creates a moving sphere that moves from (x0, y0, z0) at t0 to (x1, y1, z1) at t1.

```
"moving_spheres": [
	{
		"positions": [
			{
				"x": "&lt;number&gt;",
				"y": "&lt;number&gt;",
				"z": "&lt;number&gt;",
				"t": "&lt;number&gt;"
			},
			{
				"x": "&lt;number&gt;",
				"y": "&lt;number&gt;",
				"z": "&lt;number&gt;",
				"t": "&lt;number&gt;"
			}
		]
		"radius": "&lt;number&gt;"
		"material": { ... },
		"density": "&lt;number&gt;", (OPTIONAL)
		"copies": "&lt;number&gt;" (OPTIONAL)
	},

	...

	{
		...
	}
]
```

### Planes

```
"planes": [
	{
		"position": {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		},
		"normal": {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		},
		"material" { ... },
		"rotation:" { ... } (OPTIONAL)
		"density": "&lt;number&gt;", (OPTIONAL)
		"copies": "&lt;number&gt;" (OPTIONAL)
	},

	...

	{
		...
	}
]
```

### Cuboids

The `position` will be the center of the cuboid.

```
"cuboids": [
	{
		"position": {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		},
		"size": {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		},
		"material": { ... },
		"rotation:" { ... } (OPTIONAL)
		"density": "&lt;number&gt;", (OPTIONAL)
		"copies": "&lt;number&gt;" (OPTIONAL)
	},

	...

	{
		...
	}
]
```

### Meshes

Currently, the program supports loading meshes via STL files, and crashes when a bad path is given. Also don't bother loading high triangle count meshes since this isn't GPU accelerated. You'll probably wait a *really* long time for the render to complete!

```
"meshes": [
	{
		"filename": "&lt;path/to/stl&gt;",
		"position": {
			"x": "&lt;number&gt;",
			"y": "&lt;number&gt;",
			"z": "&lt;number&gt;"
		},
		"material": { ... },
		"rotation:" { ... } (OPTIONAL)
		"density": "&lt;number&gt;", (OPTIONAL)
		"copies": "&lt;number&gt;" (OPTIONAL)
	},

	...

	{
		
	}
]
```