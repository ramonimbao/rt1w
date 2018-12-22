# Configuring a Render

The following is a list of possible key-value pairs you can enter to configure the camera and the output render. 

## Config

The `output` key specifies the name of the output file you want to render. From the `image` crate, supported output formats are: PNG, JPEG, GIF, BMP, ICO, and PNM. Though I've only ever tested creating PNGs.

```
"config": {
	"width:" &lt;number&gt;,
	"height:" &lt;number&gt;,
	"samples:" &lt;number&gt;,
	"output": &lt;output/file/name&gt;
}
```

## Camera

All keys are optional. A default will be set if no value for the key is found. Values for `from` and `to` must *both* be specified or either won't be read.  
`t0` and `t1` specify the time the camera shutter is kept open for creating the motion blur in the moving spheres.

```
"camera:" {
	"from:" {
		"x:" &lt;number&gt;,
		"y:" &lt;number&gt;,
		"z:" &lt;number&gt;
	},
	"to:" {
		"x:" &lt;number&gt;,
		"y:" &lt;number&gt;,
		"z:" &lt;number&gt;
	},
	"vertical:" {
		"x:" &lt;number&gt;,
		"y:" &lt;number&gt;,
		"z:" &lt;number&gt;
	},
	"fov": &lt;number&gt;,
	"aspect_ratio": &lt;number&gt;,
	"aperture": &lt;number&gt;,
	"focus_distance": &lt;number&gt;,
	"t0:" &lt;number&gt;,
	"t1": &lt;number&gt;
}
```