# Subpixel encoding
I came across [Japhy Riddle's video on subpixel art](https://www.youtube.com/watch?v=SlS3FOmKUbE), 
and it got me thinking: what if you store data in a subpixel? Each pixel can hold 3 bits of data. By
stitching together all these pixels, you can encode (and decode) whatever you want! It’s incredibly
inefficient and useless, but it's a fun thing to make!

In theory, any arbitrary data can be encoded, but for now, only strings are implemented. This whole 
project is more a proof of concept rather than anything serious.

![Example](./example.png)

## Details
The encoding works by converting the received data into bytes, and converting that list of bytes 
into pixels. Each pixel consists of 3 subpixels, and each byte is stored in 8 subpixels. This means 
the first byte is spread out over the first 3 pixels. If the bit is high (1) the value for the 
related subpixel will be set to 255 (#FF). If the bit is low (0) the value for the related subpixel
will be set to 0 (#00).

If the given list of bytes does not fit into an exact number of pixels, the remaining subpixels in
the last pixel will be set to 0.

For example a value of `0b0101_0101` will be stored as 3 pixels with the following colors:
`#00FF00` `#FF00FF` `#00FF00`

Decoding works (unsurprisingly) by doing the exact opposite. We loop through each subpixel, collect
them in chunks of 8, and then flatten each chunk into a single byte again. If the value of a
subpixel is greater than 127, it is treated as a 1, otherwise it's a 0.

## Usage
At the moment the only way to run this project is by building it from source (again, it's just a POC).

### Encode text
```sh
cargo run encode -o image.png -m "Hello world"
```

### Decode image
```sh
cargo run decode -i image.png
```