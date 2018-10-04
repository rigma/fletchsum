# fletchsum

A CLI program which computes the [Fletch-64 checksum](https://en.wikipedia.org/wiki/Fletcher%27s_checksum) of a file or the standard input.

## Description

`fletchsum` is an experimental implementation of the [Fletch-64 checksum](https://en.wikipedia.org/wiki/Fletcher%27s_checksum) done in [Rust](https://rust-lang.org) which tries to follow the same implementation done by the GNU project for `md5sum` or `sha256sum`.

It's not a system ready application, but you can use as an example of how to use the [Rust](https://rust-lang.org) for a CLI application, but I want to keep the work going to have an efficient version of this program.

## Future works

Actually the [Fletch-64 checksum](https://en.wikipedia.org/wiki/Fletcher%27s_checksum) works with 360 elements chunks, I'll try to use the concurrency capabilities of the Rust language to improve the overall speed of the application.

I also plan to improve the code quality as I learn Rust best practises.

## Known bugs

For now, binary files cannot be opened by `fletchsum`. I will invistigate this issue as soon as other key features are implemented into the application.

## Contributions

If you wish to contribute, feel free to fork this repository and submit merge requests! Learning may be a collective process!

## License

[GPL v3](www.gnu.org/licenses/gpl.html)