# music-visualizer

## About
This is a programmable music visualizer written in Rust. You can write a script to map audio input to visual output for your viewing pleasure.

It uses a number of external crates, notably Piston for graphical output. 


## Usage
You'll need cargo. To get going quickly: `cargo run [audio file] [script]`.


## Scripts
Writing a script is easy. You can find an example script in the /examples folder.

Graphical **effects** are written as follows:

```
effect{Argument = Expr, ...}
```

You can then map the inputs to the effect to Expressions. Expressions support simple arithmetic using Audio Components and constant values. You can have as many effects as you like. You can also write the arguments for the effects in any order, but they need to be named - they also have default values, so you don't have to specify them all.


## Current Feature List
#### October 4, 2017

### File Types
MP3 & WAV

### Effects & Arguments
* circles(Size, R, G, B, X, Y)
* dots(Size, R, G, B, Count, X, Y)
* bar(Size, R, G, B, X, Y)

### Audio Components
* Impulse (Level above a threshold)
* Level (Average level)

### Scripting support
* Effects.
* Audio outputs.
* Constants.
* Expressions: +,-,*,/ and ()


## Planned
* Actually playing music - with controls.
* Frequency components from audio.
* Normalise audio data, and sync more with graphics.
* Add backgrounds/post processing effects.
* Add better ways of dealing with colour (HSV).
* Add decay/delay controls to primitives.
* Adding bindings for variables and functions in the script.
* Adding in-built functions.
* More graphic effects!
