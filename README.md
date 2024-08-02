# Scale PixelArt images using RetroArch shaders.
[RetroArch](https://github.com/libretro/RetroArch) contains a lot of cool shaders for enlarging images (for example, scalefx) which can be useful when adopting pixel-art images for HIDPI screens.

This tool allows to use RetroArch shaders for static images.

# Usage
```
Pixel Art scaling algorithms from RetroArch

Usage: ra-pixelart-scale [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -m, --method <METHOD>                Scale method [default: scalefx-9x]
      --list-methods                   List available methods
  -i, --input <INPUT>                  Input image
  -s, --scale <SCALE>                  Output scale [default: 0]
  -o, --output <OUTPUT>                Output filename
      --resize <RESIZE>                Resize after scale (WxH or Nx)
      --resize-method <RESIZE_METHOD>  Resize method: nearest, triangle, catmullrom, gaussian, lanczos3 [default: nearest]
      --alpha <ALPHA>                  Alpha mode: auto, strip, bypass, split [default: split]
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

# Resize after scale

You can apply additional for resize after scaling.

```bash
# Resize image 9x larger and then scale it back to the original WxH using lanczos3
ra-pixelart-scale -m scalefx-9x --resize 100% --resize-method lanczos3 -i /tmp/pixels.png -o /tmp/pixels-smooth.png

# Resize image 9x larger and then resize it to 128x128 using lanczos3
ra-pixelart-scale -m scalefx-9x --resize 128x128 --resize-method lanczos3 -i /tmp/pixels.png -o /tmp/pixels-smooth.png
```

# Custom shaders

Although this program is designed for image enlargement, you can also use it to process images using any RetroArch shaders.

```bash
ra-pixelart-scale --custom-preset shaders/presets/crt-royale-pal-r57shell.slangp -i /tmp/lenna.png -o /tmp/lenna_crt.png
```

# Headless

Since this tool uses OpenGL, then a display server is required for running. Alternatively, you can use Xvfb for running this tool on a server.

```bash
xvfb-run target/debug/ra-pixelart-scale -i /tmp/lenna.png -o /tmp/test.png
```

# Supported scaling methods
| Name    | Scale |
|---------|-------|
| **scalefx:** | |
|[scalefx-hybrid](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx-hybrid.slangp)|3x|
|[scalefx+rAA](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx+rAA.slangp)|3x|
|[scalefx](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx.slangp)|3x|
|[scalefx-9x](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/scalefx-9x.slangp)|9x|
|[scalefx-old](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/shaders/old/scalefx.slangp)|3x|
|[scalefx-9x-old](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalefx/shaders/old/scalefx-9x.slangp)|9x|
| **hqx:** | |
|[hq3x](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq3x.slangp)|3x|
|[hq2x](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq2x.slangp)|2x|
|[hq2x-halphon](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq2x-halphon.slangp)|2x|
|[hq4x](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/hqx/hq4x.slangp)|4x|
| **xbrz:** | |
|[2xbrz-linear](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/2xbrz-linear.slangp)|2x|
|[4xbrz-linear](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/4xbrz-linear.slangp)|4x|
|[5xbrz-linear](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/5xbrz-linear.slangp)|5x|
|[6xbrz-linear](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/6xbrz-linear.slangp)|6x|
|[xbrz-freescale](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/xbrz-freescale.slangp)|6x|
|[xbrz-freescale-multipass](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbrz/xbrz-freescale-multipass.slangp)|6x|
| **xbr:** | |
|[xbr-lv2](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv2.slangp)|2x|
|[xbr-lv2-sharp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv2-sharp.slangp)|2x|
|[xbr-lv3](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv3.slangp)|2x|
|[xbr-lv3-sharp](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/xbr-lv3-sharp.slangp)|2x|
|[super-xbr](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/super-xbr.slangp)|2x|
|[super-xbr-fast](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xbr/super-xbr-fast.slangp)|2x|
| **scalenx:** | |
|[scale2xSFX](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale2xSFX.slangp)|2x|
|[scale2xplus](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale2xplus.slangp)|2x|
|[mmpx](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/mmpx.slangp)|2x|
|[scale3x](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale3x.slangp)|3x|
|[epx](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/epx.slangp)|2x|
|[scale2x](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalenx/scale2x.slangp)|2x|
| **omniscale:** | |
|[omniscale](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/omniscale/omniscale.slangp)|2x|
|[omniscale-legacy](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/omniscale/omniscale-legacy.slangp)|2x|
| **eagle:** | |
|[super-2xsai-fix-pixel-shift](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/super-2xsai-fix-pixel-shift.slangp)|2x|
|[2xsai](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/2xsai.slangp)|2x|
|[super-2xsai](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/super-2xsai.slangp)|2x|
|[2xsai-fix-pixel-shift](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/2xsai-fix-pixel-shift.slangp)|2x|
|[supereagle](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/eagle/supereagle.slangp)|2x|
| **sabr:** | |
|[sabr-hybrid-deposterize](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/sabr/sabr-hybrid-deposterize.slangp)|2x|
|[sabr](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/sabr/sabr.slangp)|2x|
| **xsoft:** | |
|[4xsoft](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsoft/4xsoft.slangp)|4x|
|[4xsoftSdB](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsoft/4xsoftSdB.slangp)|4x|
| **scalehq:** | |
|[4xScaleHQ](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalehq/4xScaleHQ.slangp)|4x|
|[2xScaleHQ](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/scalehq/2xScaleHQ.slangp)|2x|
| **xsal:** | |
|[2xsal](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/2xsal.slangp)|2x|
|[4xsal-level2](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/4xsal-level2.slangp)|4x|
|[4xsal-level2-hq](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/xsal/4xsal-level2-hq.slangp)|4x|
| **fsr:** | |
|[smaa+fsr](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/fsr/smaa+fsr.slangp)|2x|
|[fsr](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/fsr/fsr.slangp)|2x|
|[fsr-easu](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/fsr/fsr-easu.slangp)|2x|
| **cleanEdge:** | |
|[cleanEdge-scale](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/cleanEdge/cleanEdge-scale.slangp)|2x|
| **ddt:** | |
|[ddt-jinc-linear](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-jinc-linear.slangp)|2x|
|[cut](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/cut.slangp)|2x|
|[3-point](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/3-point.slangp)|2x|
|[ddt-jinc](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-jinc.slangp)|2x|
|[ddt](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt.slangp)|2x|
|[ddt-extended](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-extended.slangp)|2x|
|[ddt-xbr-lv1](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/ddt/ddt-xbr-lv1.slangp)|2x|
| **nnedi3:** | |
|[nnedi3-nns32-2x-rgb-nns32-4x-luma](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns32-2x-rgb-nns32-4x-luma.slangp)|4x|
|[nnedi3-nns64-2x-nns32-4x-nns16-8x-rgb](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns64-2x-nns32-4x-nns16-8x-rgb.slangp)|8x|
|[nnedi3-nns16-2x-luma](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns16-2x-luma.slangp)|2x|
|[nnedi3-nns16-4x-luma](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns16-4x-luma.slangp)|4x|
|[nnedi3-nns32-4x-rgb](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns32-4x-rgb.slangp)|4x|
|[nnedi3-nns16-2x-rgb](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns16-2x-rgb.slangp)|2x|
|[nnedi3-nns64-2x-nns32-4x-rgb](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nnedi3/nnedi3-nns64-2x-nns32-4x-rgb.slangp)|4x|
| **nedi:** | |
|[nedi-hybrid-sharper](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi-hybrid-sharper.slangp)|2x|
|[nedi-hybrid](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi-hybrid.slangp)|2x|
|[nedi](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi.slangp)|2x|
|[nedi-sharper](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/nedi-sharper.slangp)|2x|
|[fast-bilateral-nedi](https://github.com/libretro/slang-shaders/tree/master/edge-smoothing/nedi/fast-bilateral-nedi.slangp)|2x|

# Respect
- [librashader - cool library which implements RetroArch shader presets.](https://github.com/SnowflakePowered/librashader)
- [RetroArch - greatest emulators forntend.](https://github.com/libretro/RetroArch)
