# pngme

A small CLI tool that allows you to encode and decode messages in a PNG.

Thanks to [picklenerd](https://picklenerd.github.io/pngme_book/introduction.html) for the idea and guidance!

## How it works

Every `.png` file is essentially composed of a list of chunks, each containing relevant metadata. Together, these chunks can be decoded to produce images. `pngme` allows you to encode custom chunks containing hidden messages into PNGs and interact with them.

After encoding any custom chunks, you should still be able to view the original image in PNG viewers. However, you may experience unpredictable behaviour when working with [PNG editors](http://www.libpng.org/pub/png/spec/1.2/PNG-Ordering.html).

Check out the [official PNG spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) for more details.

## Usage

```
USAGE:
    pngme [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decode message from png
    encode    Encode message into png
    help      Print this message or the help of the given subcommand(s)
    print     Print message from png
    remove    Remove message from png

For more information on each subcommand, run `pngme [SUBCOMMAND] -h`
```

### Example

```
$ pngme encode example.png rUsT "I'm a secret message!"
$ pngme decode example.png rUsT
> I'm a secret message!
```

## Development

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone or fork this repo and `cd` into it
3. Run `cargo run -- ` with appropriate args
