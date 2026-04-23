# Project RF-Ham

Ham Radio libraries and tools for Rust.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-rfham.svg)](<https://github.com/johnstonskj/rust-rfham/stargazers>)

## Crates

| Name               | Crate                                                                                                       | Docs                                                                                   | Description                          | Status                   |
|--------------------|-------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------|--------------------------------------|--------------------------|
| **rfham**          | [![crates.io](https://img.shields.io/crates/v/rfham.svg)](https://crates.io/crates/rfham)                   | [![docs.rs](https://docs.rs/rfham/badge.svg)](https://docs.rs/rfham)                   | Will act as *prelude* combination    | Not Started              |
| **rfham-antennas** | [![crates.io](https://img.shields.io/crates/v/rfham-antennas.svg)](https://crates.io/crates/rfham-antennas) | [![docs.rs](https://docs.rs/rfham-antennas/badge.svg)](https://docs.rs/rfham-antennas) | Provides antenna models/calculations | Started                  |
| **rfham-bands**    | [![crates.io](https://img.shields.io/crates/v/rfham-bands.svg)](https://crates.io/crates/rfham-bands)       | [![docs.rs](https://docs.rs/rfham/badge.svg)](https://docs.rs/rfham-bands)             | Country-specific band plans          | Complete for US          |
| **rfham-cli**      | [![crates.io](https://img.shields.io/crates/v/rfham-cli.svg)](https://crates.io/crates/rfham-cli)           | [![docs.rs](https://docs.rs/rfham-cli/badge.svg)](https://docs.rs/rfham-cli)           | CLI for interacting with the rest    | Tracking complete        |
| **rfham-config**   | [![crates.io](https://img.shields.io/crates/v/rfham-config.svg)](https://crates.io/crates/rfham-config)     | [![docs.rs](https://docs.rs/rfham-config/badge.svg)](https://docs.rs/rfham-config)     | Shared configuration file handling   | Tracking complete        |
| **rfham-core**     | [![crates.io](https://img.shields.io/crates/v/rfham-core.svg)](https://crates.io/crates/rfham-core)         | [![docs.rs](https://docs.rs/rfham-core/badge.svg)](https://docs.rs/rfham-core)         | Core data types                      | Complete                 |
| **rfham-geo**      | [![crates.io](https://img.shields.io/crates/v/rfham-geo.svg)](https://crates.io/crates/rfham-geo)           | [![docs.rs](https://docs.rs/rfham-geo/badge.svg)](https://docs.rs/rfham-geo)           | Grid locators and lookup             | Grid Complete, No Lookup |
| **rfham-itu**      | [![crates.io](https://img.shields.io/crates/v/rfham-itu.svg)](https://crates.io/crates/rfham-itu)           | [![docs.rs](https://docs.rs/rfham-itu/badge.svg)](https://docs.rs/rfham-itu)           | ITU band allocations                 | Complete                 |
| **rfham-markdown** | [![crates.io](https://img.shields.io/crates/v/rfham-markdown.svg)](https://crates.io/crates/rfham-markdown) | [![docs.rs](https://docs.rs/rfham-markdown/badge.svg)](https://docs.rs/rfham-markdown) | Utility crate for writing markdown   | Complete                 |
| **rfham-radios**   | [![crates.io](https://img.shields.io/crates/v/rfham-radios.svg)](https://crates.io/crates/rfham-radios)     | [![docs.rs](https://docs.rs/rfham-radios/badge.svg)](https://docs.rs/rfham-radios)     | Provides radio models                | Not Started              |

## Examples

TBD

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-MIT).
