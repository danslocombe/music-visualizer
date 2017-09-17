# music-visualizer

### Flow:

Audio in (.wav/.mp3/...) ->

Audio extractor: generates raw data (amplitude, freq) ->

Audio processor: dispatches processing modules which generate high-level data (amp threshold, freq band etc.) ->

Mapper: redirect to video processors. repackage data. ->

Video processor: dispatches rendering modules which render effects to window (circles, ripples, bars etc.) ->

Video output: (post processing?) final output ->

### Script Ideas:

~~~~
// set base to render upon
// background: BackGround(Param: AudioEffect, ...)

background: Plain(Color: black)

// or...

background: Plain(Color: red)

// or even...

background: Plain(Color: HighFrequency) // this could be more complex...!

// more...

background: ChangingColor(Change: MidFrequency) // dynamic backgrounds...


// then effects, map audio -> video:
// VideoEffect(Param: AudioEffect, ...)

DansCircles(Radius: AmplitudeThreshold) // other parameters are set to default...

// or...

DansCircles(Radius: AmplitudeThreshold, Color: white) // alternative to "white": raw color? [1.0, 1.0, 1.0]

// or...

DansCircles(Radius: AmplitudeThreshold, Color: LowFrequency)

// or with expressions...

DansCircles(Radius: Amplitude * Amplitude)


// so final program idea:

background: Plain(HighFrequency)

DansCircles(Radius: AmplitudeThreshold, Color: white)

Squares(Length: Amplitude * Amplitude, Color: 1.0 - HighFrequency) // with 1.0 as max (could make things hideous)
~~~~
