# spelunkicons

Generate Spelunky Identicons

## Building

Copy textures from your extracted assets, i.e. `Mods/Extracted/Data/Textures`, into `target/Textures`. Also copy textures from the [Spelunky 2 Retrofied](https://spelunky.fyi/mods/m/retro/) mod into `target/ClassicTextures`.

```sh
cargo build --release
```

## Debugging

To debug, run the service locally, then open a browser and enter a URL, e.g. `http://127.0.0.1:3000/example.png?size=7&egg=classic`, the result will be served directly to your browser. To run the service:

```sh
cargo run --release
```

Running in Debug mode (i.e. `cargo run`) is required for setting breakpoints. However loading all textures in debug mode is very slow. To speed up debug builds resize all images into the folders `Textures/LowRes` and `ClassicTextures/LowRes` respectively. Ideally scale by a multiple of a half, e.g.

```sh
cd target/Textures
mkdir LowRes
magick mogrify -resize 12.5% -quality 100 -filter Point -path ./LowRes *.png
```

The `LowRes` folder is only used in Debug builds, not in Release builds.