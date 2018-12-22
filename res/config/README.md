# Configuring a Render

The following is a list of possible key-value pairs you can enter to configure the camera and the output render. 

## Config

The `output` key specifies the name of the output file you want to render. From the `image` crate, supported output formats are: PNG, JPEG, GIF, BMP, ICO, and PNM. Though I've only ever tested creating PNGs.

```
"config": {
	"width:" <number>,
	"height:" <number>,
	"samples:" <number>,
	"output": <output/file/name>
}
```

## Camera

All keys are optional. A default will be set if no value for the key is found. Values for `from` and `to` must *both* be specified or either won't be read.  
`t0` and `t1` specify the time the camera shutter is kept open for creating the motion blur in the moving spheres.

```
"camera:" {
	"from:" {
		"x:" <number>,
		"y:" <number>,
		"z:" <number>
	},
	"to:" {
		"x:" <number>,
		"y:" <number>,
		"z:" <number>
	},
	"vertical:" {
		"x:" <number>,
		"y:" <number>,
		"z:" <number>
	},
	"fov": <number>,
	"aspect_ratio": <number>,
	"aperture": <number>,
	"focus_distance": <number>,
	"t0:" <number>,
	"t1": <number>
}
```