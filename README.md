# Media Proxy Server

The main component of [Media Proxy](https://github.com/ThePicoNerd/mediaproxy), used to compress and resize images on the web. Documentation soon to be added!

![](https://github.com/ThePicoNerd/mediaproxy-server/workflows/Master%20Release/badge.svg)

## Output formats

This application uses the [image](https://github.com/image-rs/image) crate for the majority of processing. Currently, these are the possible output formats:

- JPEG
- PNG
- GIF (**Not animated.** If an animated gif is provided as source, the first frame is selected.)
- WebP

### WebP

Provided by the awesome [webp](https://github.com/jaredforth/webp) crate. RGBA images turned out to cause severe glitches. Therefore, all images with alpha channels are converted to RGB images and the alpha channel is simply ditched. The transparency is in other words removed.

The issue [jaredforth/webp #2](https://github.com/jaredforth/webp/issues/2) has been submitted about this problem.
