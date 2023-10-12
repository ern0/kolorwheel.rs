# KolorWheel.rs


## What is it?

This crate is designed 
to make it easy 
to create palettes for GUI applications.

> With a slightly different API, for a slightly different platform:
[KolorWheel.js](https://github.com/ern0/kolorwheel.js/)

The way of creating a palette is 
to specify a base colour 
and some parameters 
that modifies the H, S, L values
in the given *spin mode* and steps.


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

Note that HSL color space is not linear.

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
- Not all value triples are different colors,
  but different colors have different values.

Also note, 
converting a HSL value to RGB is trivial,
but converting RGB to HSL is not.
For example, `#000000` (black) color has
zero *lightness*, but
*hue* and *saturation* can be amything.


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

#### `KolorWheel::new<T>(color: T, count: usize) -> Self`

Creates a `KolorWheel` object
with base parameters:

- `color`: base color, should be `HslColor` or
  any type which implements `Into<HslColor>`.
- `count`: number of steps of color transformation,
  including initial state.

Returns `KolorWheel` object,
which must be declared as mutable.


### Spin the wheel

These methods apply specified `SpinMode`
on the `KolorWheel` object.

They return mutable reference for chaining.

- `with_hue(&mut self, spin_mode: SpinMode) -> &mut KolorWheel`
- `with_saturation(&mut self, spin_mode: SpinMode) -> &mut KolorWheel`
- `with_lightness(&mut self, spin_mode: SpinMode) -> &mut KolorWheel`

The `SpinMode` enum lists the possible spin
operations.

Valid ranges for *hue* is 0 to 360°,
but upon overflow or underflow,
it will be normalized,
e.g. *365° -> 5* or *-10° -> 350°*.

The *saturation* and *lightness* 
must be between 0 and 100%.
Upon overflow or underflow, 
they will be cut, 
e.g. *99% + 5% -> 100%* or *10% - 25% = 0*.

```
pub enum SpinMode<'m> {
    Still,
    Absolute(i32),
    RelativeIncl(i32),
    RelativeExcl(i32),
    Offset(&'m [i32]),
}
```

- `Still`: defult value for no change, 
   shouldn't be used.
- `Absolute`: set absolute target value.
- `RelativeIncl`: set relative target value.
   the target value will be included in the result.
- `RelativeIncl`: set relative target value,
   the target value will be excluded from the result.
   It's useful when the created palette is circular,
   the real target is the next round's first color,
   so the last color should be one step back.
- `Offset`: defines a list, which offsets the result.
   If the offset size is less than spin size,
   offsets will repeat.

A H/S/L component may have optionally one of
`Absolute`, `RelativeIncl` and `RelativeExcl`,
and optionally one `Offset` specified.


### Spin macros

This method applies specified `SpinMacro`
on the `KolorWheel` object.

It returns mutable reference for chaining.

`with_macro(&mut self, spin_macro: SpinMacro) -> &mut Self`

`SpinMacro` transformations are shourtcuts
to common transformations,
they are implemented with `SpinMode` under the hood.

```
pub enum SpinMacro {
    GradientColor(HslColor),
    FadeToGray(i32),
    FadeToBlack,
    FadeToWhite,
}
```

- `GradientColor`: simple gradient,
  shorthand for
  setting absolute transformations for
  H, S and L.
  Target color should be `HslColor`,
  or any type which implements `Into<HslColor>`,
  they will be converted to `HslColor`
  upon the method call.
- `FadeToGray`: shorthand for
  transforming *saturation* to zero,
  and *lightness* to the specified value,
  aka. gray level.
- `FadeToBlack`: special case for `FadeToGray`,
  with *lightness* value of 0.
- `FadeToWhite`: special case for `FadeToGray`,
  with *lightness* value of 100%.


### Forking

Creates a new *spinner* for each result,
with results as base colors.

`fork(&mut self, count: usize) -> &mut Self`

- `count`: the number of steps for 
  the inner spinner.
  
Returns mutable reference for chaining.

The overall steps will be the product value of
all `count` parameters, e.g. the initial
value in constructor, 
and ones specified in `fork()` calls.

For example, 
- create a `KolorWheel` with
  base color of red and 4 steps,
- add transformation
  `SpinMode::Absolute` to blue, then
- call `fork()` with count of 10
  and add a macro transformation
  `SpinMacro::FadeToWhite`.

What we get:
- the result's first color will be red, then
- it fades to white in 10 steps.
- The 11th color will be violet 
  (1/4 step closer to blue), and
- it will be faded to white in 10 steps,

...and so on.
The total number of results will be *4 * 10 = 40*.


### Result

There are three way of getting the result:
- iterator,
- vector,
- callback.

The result item is always `HslColor`,
which can be converted to `RgbColor`
using `From` and `Into` traits:
```
pub struct RgbColor {
    pub r: u8, 
    pub g: u8, 
    pub b: u8,
}
```

#### Iterator

The most convenient way to get through the result
is using `KolorWheel` as iterator:

```
let mut kw = KolorWheel::new( ... );
...
for hsl_color in kw {
  let rgb_color: RgbColor = hsl_color.into();
  ...
}
```

#### Callback

The `spin()` method is calling the callback 
with each result value:
```
KolorWheel::new( ... )
    ...
    .spin(&mut|hsl_color: HslColor| { 
        ...
    })
;
```

#### Vector

Use `spin_vec()` to get the results in a `vec`.
let result = KolorWheel::new( ... )
    ...
    .spin_vec::<HslColor>()
;

The result stored in the `vec` can be any type,
which implements `From<HslColor>` trait.


## Examples



### 1: Gradient

### 2: Lit/abs

### 3-4: Hue/reli, Hue/relx

### 5: HueOffsets

### 6: Palette1

### 7: Palette2


https://github.com/dyuri/pastel
