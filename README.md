# Media Proxy Server

The main component of [Media Proxy](https://github.com/ThePicoNerd/mediaproxy), used to compress and resize images on the web. Documentation soon to be added!

![CI](https://github.com/ThePicoNerd/mediaproxy-server/workflows/CI/badge.svg)

## Output formats

This application uses the [image](https://github.com/image-rs/image) crate for the majority of processing. Currently, these are the possible output formats:

- JPEG
- PNG
- GIF (**Not animated.** If an animated gif is provided as source, the first frame is selected.)
- WebP (Provided by the awesome [webp](https://github.com/jaredforth/webp) crate.)
