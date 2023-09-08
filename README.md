# kolorwheel.rs


## What is it?

This crate is designed 
to make it easy 
to create palettes for GUI applications.

> With a slightly different API, for a slightly different platform:
[KolorWheel.js](https://github.com/ern0/kolorwheel.js/)


## The HSL color model

In HSL color model, colors are defined by
*Hue*, *Saturation* and *Lightness*.

**Hue** is a circular dimension,
0° is red, 
180° is cyan, 
and 360° is also red.

**Saturation** goes from 0 to 100%,
0 is gray, 100% is the full color.

**Lightness** also goes from 0 to 100%,
0 is black, 100% is white.

Note that 
not all value triples are different colors.

- The closer the *saturation* is to zero,
  the less effect the *hue* has.
- If the *saturation* is 0,
  the *hue* is meaningless.
- The farther the *lightness* is from 50%,
  the less effect 
	*hue* and *saturation* have.
- If the *lightness* is 0 or 100%,
  *hue* and *saturation* 
	have no effect at all.

Also note, 
converting a HSL value to RGB is trivial,
but converting RGB to HSL is not,
e.g. the *hue* value of
`#000000` (black) can be anything.

### The API
