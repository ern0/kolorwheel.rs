# KolorWheel.rs


## What is it?

This crate is designed 
to make it easy 
to create palettes for GUI applications.

> With a slightly different API, for a slightly different platform:
[KolorWheel.js](https://github.com/ern0/kolorwheel.js/)

The way to create a palette is 
to specify a base colour 
and some parameters 
that tweak the H, S, L values.


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


## The flaws of the HSL color model

The HSL model does not represent
how human vision actually works.

Linear changes in H, S, L values 
may not perceived linearly 
by the human eye.


## Quick API overview

**Step 1**: create the `KolorWheel` object 
with base color and number of steps:
```
let base_color = HslColor::new(0, 100, 40);
let mut kw = KolorWheel::new(base_color, 10);

```

**Step 2**: define H/S/L absolute or relative changes:
```
kw.with_hue(SpinMode::Absolute(120));
kw.with_saturation(SpinMode::RelativeIncl(-15));

```

**Step 3**: optionally,
on top of absolute/relative changes,
offset array slices can be specified:
```
kw.with_lightness(SpinMode::Offset(&[0, 15, 30]));
```

**Step 4**: optionally, 
the result can be forked,
so each result item produces a separate
series of colors using item color as base color,
only the size of sub-series should be specified:
```
kw.fork(5);
kw.with_hue(SpinMode::RelativeIncl(45));
```
The number of overall items will be
the product of original and forked counts
(in this example: 10 * 5 = 50);

**Step 5**: the result can be get via
iterator, lambda or `vec`. The item type is
`HslColor`, which can be transformed
to `RgbColor`:
```
for hsl_color in kw {
  let rgb_color: RgbColor = hsl_color.into();
  //...
}
```

## API details

### Create `KolorWheel` object



## Examples
