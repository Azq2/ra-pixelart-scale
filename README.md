# Scale PixelArt images using RetroArch shaders.
[RetroArch](https://github.com/libretro/RetroArch) contains a lot of cool shaders for enlarging images (for example, scalefx) which can be useful when adopting pixel-art images for HIDPI screens.

This tool allows to use RetroArch shaders for static images.

# Usage
```
Pixel Art scaling algorithms from RetroArch

Usage: ra-pixelart-scale [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -m, --method <METHOD>                Scale method [default: scalefx-9x]
  -i, --input <INPUT>                  Input image
  -s, --scale <SCALE>                  Output scale [default: 0]
  -o, --output <OUTPUT>                Output filename
      --resize <RESIZE>                Output resize (WxH)
      --alpha <ALPHA>                  Alpha saving mode: none, split [default: split]
      --custom-preset <CUSTOM_PRESET>  Custom .slangp file
  -h, --help                           Print help
  -V, --version                        Print version
```

# Examples
```bash
# Scale will automatically set to best default for given method
ra-pixelart-scale -m scalefx-9x -i mario.png -o /tmp/mario-9x.png

# Or set custom scale
ra-pixelart-scale -m xbrz-freescale-multipass -i mario.png -s 3 -o /tmp/mario-3x.png

# Or use custom slangp preset
ra-pixelart-scale --custom-preset slangs-shaders/edge-smoothing/scalefx/scalefx.slangp -i mario.png -s 3 -o /tmp/mario-3x.png
```

# Alpha channel

Most shaders don't support the alpha channel and produce a white background. But this tool has a special mode for preserving the alpha channel.

This is done by splitting the RGBA image into an RGB + Alpha channel, then each part is scaled separately.

This mode is enabled by default and controlled via `--alpha` argument.

# Scale value

Most shaders are designed for one specific scale value at which they give the best results. And any other values can get worse results.

For e.g. for hq2x, the best scale is 2, for hq4x it is 4.

You don't need to pass `--scale` in most cases, because the best scale is chosen by default for each method.

Pass custom `--scale` only for specific scale methods which can work with any scale value. See table below for more info.

# Custom shaders

Although this program is designed for image enlargement, you can also use it to process images using any RetroArch shaders.

```bash
ra-pixelart-scale --custom-preset shaders/presets/crt-royale-pal-r57shell.slangp -i /tmp/lenna.png -o /tmp/lenna_crt.png
```

# Supported scaling methods

**scalefx**
| Name    | Scale | Shader |
|---------|-------|--------|
|scalefx-hybrid|3x|[edge-smoothing/scalefx/scalefx-hybrid.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx-hybrid.slangp)|
|scalefx+rAA|3x|[edge-smoothing/scalefx/scalefx+rAA.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx+rAA.slangp)|
|scalefx|3x|[edge-smoothing/scalefx/scalefx.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx.slangp)|
|scalefx-9x|9x|[edge-smoothing/scalefx/scalefx-9x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx-9x.slangp)|
|scalefx-old|3x|[edge-smoothing/scalefx/shaders/old/scalefx.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/shaders/old/scalefx.slangp)|
|scalefx-9x-old|9x|[edge-smoothing/scalefx/shaders/old/scalefx-9x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/shaders/old/scalefx-9x.slangp)|

**xbrz**
| Name    | Scale | Shader |
|---------|-------|--------|
|2xbrz-linear|2x|[edge-smoothing/xbrz/2xbrz-linear.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/2xbrz-linear.slangp)|
|4xbrz-linear|4x|[edge-smoothing/xbrz/4xbrz-linear.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/4xbrz-linear.slangp)|
|5xbrz-linear|5x|[edge-smoothing/xbrz/5xbrz-linear.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/5xbrz-linear.slangp)|
|6xbrz-linear|6x|[edge-smoothing/xbrz/6xbrz-linear.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/6xbrz-linear.slangp)|
|xbrz-freescale|6x|[edge-smoothing/xbrz/xbrz-freescale.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/xbrz-freescale.slangp)|
|xbrz-freescale-multipass|6x|[edge-smoothing/xbrz/xbrz-freescale-multipass.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/xbrz-freescale-multipass.slangp)|

**xbr**
| Name    | Scale | Shader |
|---------|-------|--------|
|xbr-lv2|2x|[edge-smoothing/xbr/xbr-lv2.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv2.slangp)|
|xbr-lv2-sharp|2x|[edge-smoothing/xbr/xbr-lv2-sharp.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv2-sharp.slangp)|
|xbr-lv3|2x|[edge-smoothing/xbr/xbr-lv3.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv3.slangp)|
|xbr-lv3-sharp|2x|[edge-smoothing/xbr/xbr-lv3-sharp.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv3-sharp.slangp)|
|super-xbr|2x|[edge-smoothing/xbr/super-xbr.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/super-xbr.slangp)|
|super-xbr-fast|2x|[edge-smoothing/xbr/super-xbr-fast.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/super-xbr-fast.slangp)|

**hqx**
| Name    | Scale | Shader |
|---------|-------|--------|
|hq3x|3x|[edge-smoothing/hqx/hq3x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq3x.slangp)|
|hq2x|2x|[edge-smoothing/hqx/hq2x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq2x.slangp)|
|hq2x-halphon|2x|[edge-smoothing/hqx/hq2x-halphon.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq2x-halphon.slangp)|
|hq4x|4x|[edge-smoothing/hqx/hq4x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq4x.slangp)|

**eagle**
| Name    | Scale | Shader |
|---------|-------|--------|
|super-2xsai-fix-pixel-shift|2x|[edge-smoothing/eagle/super-2xsai-fix-pixel-shift.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/super-2xsai-fix-pixel-shift.slangp)|
|2xsai|2x|[edge-smoothing/eagle/2xsai.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/2xsai.slangp)|
|super-2xsai|2x|[edge-smoothing/eagle/super-2xsai.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/super-2xsai.slangp)|
|2xsai-fix-pixel-shift|2x|[edge-smoothing/eagle/2xsai-fix-pixel-shift.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/2xsai-fix-pixel-shift.slangp)|
|supereagle|2x|[edge-smoothing/eagle/supereagle.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/supereagle.slangp)|

**omniscale**
| Name    | Scale | Shader |
|---------|-------|--------|
|omniscale|2x|[edge-smoothing/omniscale/omniscale.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/omniscale/omniscale.slangp)|
|omniscale-legacy|2x|[edge-smoothing/omniscale/omniscale-legacy.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/omniscale/omniscale-legacy.slangp)|

**scalenx**
| Name    | Scale | Shader |
|---------|-------|--------|
|scale2xSFX|2x|[edge-smoothing/scalenx/scale2xSFX.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale2xSFX.slangp)|
|scale2xplus|2x|[edge-smoothing/scalenx/scale2xplus.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale2xplus.slangp)|
|mmpx|2x|[edge-smoothing/scalenx/mmpx.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/mmpx.slangp)|
|scale3x|3x|[edge-smoothing/scalenx/scale3x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale3x.slangp)|
|epx|2x|[edge-smoothing/scalenx/epx.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/epx.slangp)|
|scale2x|2x|[edge-smoothing/scalenx/scale2x.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale2x.slangp)|

**sabr**
| Name    | Scale | Shader |
|---------|-------|--------|
|sabr-hybrid-deposterize|2x|[edge-smoothing/sabr/sabr-hybrid-deposterize.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/sabr/sabr-hybrid-deposterize.slangp)|
|sabr|2x|[edge-smoothing/sabr/sabr.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/sabr/sabr.slangp)|

**xsoft**
| Name    | Scale | Shader |
|---------|-------|--------|
|4xsoft|4x|[edge-smoothing/xsoft/4xsoft.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsoft/4xsoft.slangp)|
|4xsoftSdB|4x|[edge-smoothing/xsoft/4xsoftSdB.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsoft/4xsoftSdB.slangp)|

**scalehq**
| Name    | Scale | Shader |
|---------|-------|--------|
|4xScaleHQ|4x|[edge-smoothing/scalehq/4xScaleHQ.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalehq/4xScaleHQ.slangp)|
|2xScaleHQ|2x|[edge-smoothing/scalehq/2xScaleHQ.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalehq/2xScaleHQ.slangp)|

**xsal**
| Name    | Scale | Shader |
|---------|-------|--------|
|2xsal|2x|[edge-smoothing/xsal/2xsal.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/2xsal.slangp)|
|4xsal-level2|4x|[edge-smoothing/xsal/4xsal-level2.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/4xsal-level2.slangp)|
|4xsal-level2-crt|4x|[edge-smoothing/xsal/4xsal-level2-crt.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/4xsal-level2-crt.slangp)|
|2xsal-level2-crt|2x|[edge-smoothing/xsal/2xsal-level2-crt.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/2xsal-level2-crt.slangp)|
|4xsal-level2-hq|4x|[edge-smoothing/xsal/4xsal-level2-hq.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/4xsal-level2-hq.slangp)|

**fsr**
| Name    | Scale | Shader |
|---------|-------|--------|
|smaa+fsr|2x|[edge-smoothing/fsr/smaa+fsr.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/fsr/smaa+fsr.slangp)|
|fsr|2x|[edge-smoothing/fsr/fsr.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/fsr/fsr.slangp)|
|fsr-easu|2x|[edge-smoothing/fsr/fsr-easu.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/fsr/fsr-easu.slangp)|

**cleanEdge**
| Name    | Scale | Shader |
|---------|-------|--------|
|cleanEdge-scale|2x|[edge-smoothing/cleanEdge/cleanEdge-scale.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/cleanEdge/cleanEdge-scale.slangp)|

**ddt**
| Name    | Scale | Shader |
|---------|-------|--------|
|ddt-jinc-linear|2x|[edge-smoothing/ddt/ddt-jinc-linear.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-jinc-linear.slangp)|
|cut|2x|[edge-smoothing/ddt/cut.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/cut.slangp)|
|3-point|2x|[edge-smoothing/ddt/3-point.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/3-point.slangp)|
|ddt-jinc|2x|[edge-smoothing/ddt/ddt-jinc.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-jinc.slangp)|
|ddt|2x|[edge-smoothing/ddt/ddt.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt.slangp)|
|ddt-extended|2x|[edge-smoothing/ddt/ddt-extended.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-extended.slangp)|
|ddt-xbr-lv1|2x|[edge-smoothing/ddt/ddt-xbr-lv1.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-xbr-lv1.slangp)|

**nnedi3**
| Name    | Scale | Shader |
|---------|-------|--------|
|nnedi3-nns32-2x-rgb-nns32-4x-luma|4x|[edge-smoothing/nnedi3/nnedi3-nns32-2x-rgb-nns32-4x-luma.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns32-2x-rgb-nns32-4x-luma.slangp)|
|nnedi3-nns64-2x-nns32-4x-nns16-8x-rgb|8x|[edge-smoothing/nnedi3/nnedi3-nns64-2x-nns32-4x-nns16-8x-rgb.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns64-2x-nns32-4x-nns16-8x-rgb.slangp)|
|nnedi3-nns16-2x-luma|2x|[edge-smoothing/nnedi3/nnedi3-nns16-2x-luma.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns16-2x-luma.slangp)|
|nnedi3-nns16-4x-luma|4x|[edge-smoothing/nnedi3/nnedi3-nns16-4x-luma.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns16-4x-luma.slangp)|
|nnedi3-nns32-4x-rgb|4x|[edge-smoothing/nnedi3/nnedi3-nns32-4x-rgb.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns32-4x-rgb.slangp)|
|nnedi3-nns16-2x-rgb|2x|[edge-smoothing/nnedi3/nnedi3-nns16-2x-rgb.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns16-2x-rgb.slangp)|
|nnedi3-nns64-2x-nns32-4x-rgb|4x|[edge-smoothing/nnedi3/nnedi3-nns64-2x-nns32-4x-rgb.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns64-2x-nns32-4x-rgb.slangp)|

**nedi**
| Name    | Scale | Shader |
|---------|-------|--------|
|nedi-hybrid-sharper|2x|[edge-smoothing/nedi/nedi-hybrid-sharper.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi-hybrid-sharper.slangp)|
|nedi-hybrid|2x|[edge-smoothing/nedi/nedi-hybrid.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi-hybrid.slangp)|
|nedi|2x|[edge-smoothing/nedi/nedi.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi.slangp)|
|nedi-sharper|2x|[edge-smoothing/nedi/nedi-sharper.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi-sharper.slangp)|
|bilateral-variant6|2x|[edge-smoothing/nedi/presets/bilateral-variant6.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant6.slangp)|
|bilateral-variant2|2x|[edge-smoothing/nedi/presets/bilateral-variant2.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant2.slangp)|
|bilateral-variant3|2x|[edge-smoothing/nedi/presets/bilateral-variant3.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant3.slangp)|
|bilateral-variant4|2x|[edge-smoothing/nedi/presets/bilateral-variant4.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant4.slangp)|
|bilateral-variant7|2x|[edge-smoothing/nedi/presets/bilateral-variant7.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant7.slangp)|
|bilateral-variant|2x|[edge-smoothing/nedi/presets/bilateral-variant.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant.slangp)|
|bilateral-variant5|2x|[edge-smoothing/nedi/presets/bilateral-variant5.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/presets/bilateral-variant5.slangp)|
|fast-bilateral-nedi|2x|[edge-smoothing/nedi/fast-bilateral-nedi.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/fast-bilateral-nedi.slangp)|

**xbr-old**
| Name    | Scale | Shader |
|---------|-------|--------|
|xbr-lv1-standalone|2x|[edge-smoothing/xbr/other presets/xbr-lv1-standalone.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv1-standalone.slangp)|
|4xbr-hybrid-crt|4x|[edge-smoothing/xbr/other presets/4xbr-hybrid-crt.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/4xbr-hybrid-crt.slangp)|
|xbr-lv3-9x-standalone|9x|[edge-smoothing/xbr/other presets/xbr-lv3-9x-standalone.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv3-9x-standalone.slangp)|
|xbr-lv3-standalone|3x|[edge-smoothing/xbr/other presets/xbr-lv3-standalone.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv3-standalone.slangp)|
|xbr-lv3-multipass|3x|[edge-smoothing/xbr/other presets/xbr-lv3-multipass.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv3-multipass.slangp)|
|super-4xbr-3d-4p|4x|[edge-smoothing/xbr/other presets/super-4xbr-3d-4p.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/super-4xbr-3d-4p.slangp)|
|super-4xbr-3d-6p-smoother|4x|[edge-smoothing/xbr/other presets/super-4xbr-3d-6p-smoother.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/super-4xbr-3d-6p-smoother.slangp)|
|2xBR-lv1-multipass|2x|[edge-smoothing/xbr/other presets/2xBR-lv1-multipass.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/2xBR-lv1-multipass.slangp)|
|xbr-hybrid|2x|[edge-smoothing/xbr/other presets/xbr-hybrid.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-hybrid.slangp)|
|super-2xbr-3d-3p-smoother|2x|[edge-smoothing/xbr/other presets/super-2xbr-3d-3p-smoother.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/super-2xbr-3d-3p-smoother.slangp)|
|super-8xbr-3d-6p|8x|[edge-smoothing/xbr/other presets/super-8xbr-3d-6p.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/super-8xbr-3d-6p.slangp)|
|xbr-lv2-hd|2x|[edge-smoothing/xbr/other presets/xbr-lv2-hd.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv2-hd.slangp)|
|xbr-mlv4-multipass|2x|[edge-smoothing/xbr/other presets/xbr-mlv4-multipass.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-mlv4-multipass.slangp)|
|super-2xbr-3d-2p|2x|[edge-smoothing/xbr/other presets/super-2xbr-3d-2p.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/super-2xbr-3d-2p.slangp)|
|xbr-lv3-9x-multipass|9x|[edge-smoothing/xbr/other presets/xbr-lv3-9x-multipass.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv3-9x-multipass.slangp)|
|xbr-lv2-standalone|2x|[edge-smoothing/xbr/other presets/xbr-lv2-standalone.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv2-standalone.slangp)|
|xbr-lv2-multipass|2x|[edge-smoothing/xbr/other presets/xbr-lv2-multipass.slangp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/other%20presets/xbr-lv2-multipass.slangp)|

# Respect
- [librashader - cool library which implements RetroArch shader presets.](https://github.com/SnowflakePowered/librashader)
- [RetroArch - greatest emulators forntend.](https://github.com/libretro/RetroArch)
