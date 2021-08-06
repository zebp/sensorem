# sensorem

[![Crates.io][crates-badge]][crates-url]
[![Unlicense][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/sensorem.svg
[crates-url]: https://crates.io/crates/sensorem
[license-badge]: https://img.shields.io/badge/license-Unlicense-blue.svg
[license-url]: https://github.com/zebp/worker-kv/blob/master/LICENSE

Colorful sensors!

## Features

- Colorful output, yay!
- Watch mode

## Todo

### Config files

Ideally sensorem could have config files so we could turn a name like `k10temp-pci-00c3` into `AMD Ryzen 9 5950x`, whitelist sensors that should be displayed, ranges for certain display colors, etc.

### Fan RPM support

Currently sensorem only supports temperatures, but showing fan rpm would also be useful.

### Non sensor based input sources

For some devices it might be nice to know why a device might be running at a given temperature, so things like clock speed or IO throughput could be displayed.
