# dirent-sys
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generated Rust bindings for Linux `<dirent.h>`. Useful to call methods like
`scandir(3)` and `seekdir(3)`.

## Installation
```sh
$ cargo add dirent-sys
```

## Links
- [documentation][8]
- [crate][2]
- https://docs.rs/libc/0.2.36/libc/struct.dirent.html
- https://github.com/libuv/libuv/blob/master/src/unix/fs.c#L389
- http://man7.org/linux/man-pages/man3/scandir.3.html

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/dirent-sys.svg?style=flat-square
[2]: https://crates.io/crates/dirent-sys
[3]: https://img.shields.io/travis/yoshuawuyts/dirent-sys.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/dirent-sys
[5]: https://img.shields.io/crates/d/dirent-sys.svg?style=flat-square
[6]: https://crates.io/crate/dirent-sys
[7]: https://docs.rs/dirent-sys/badge.svg?version=0.1.0
[8]: https://docs.rs/crate/dirent-sys
