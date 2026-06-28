# Mirl Rendering (0.0.0-alpha)

### Miring - Simple functions for drawing on a Buffer

<details>
<summary>Flags</summary>

### Default:

**Core**

- `std` (Default)
- `c_compatible`
- `all`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

**Custom**

- `font_support`

</details>

### Entry Points

- `mirl_rendering::draw_*`

### Purpose

Drawing on a buffer one pixel at a time is not very intuitive, this lib provides some algorithms for basic shapes

### Disclaimer

The code in this lib is very very old so it needs a huge rewrite. It should also support more than simple 2d shapes. A way to enable rendering using different backends would be nice to enable gpu based rendering

### Origin

When splitting mirl/mirl_core it was clear that the functions of this crate didn't fit anywhere else, though the code is so old that I don't remember most of how it's been written.