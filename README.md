# vetis-encode

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-encode)](https://crates.io/crates/vetis-encode) [![crates.io](https://img.shields.io/crates/v/vetis-encode?style=flat-square)](https://crates.io/crates/vetis-encode) [![Build Status](https://github.com/vetis-server/vetis-encode/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-encode/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-encode) [![Documentation](https://docs.rs/vetis-encode/badge.svg)](https://docs.rs/vetis-encode/latest/vetis-encode) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-encode/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-encode/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-encode)

vetis-encode adds diffent compression methods to your host, choose the most appropriate one for your usecase!

## Features

- basic

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-encode = "0.1.0"
```

## Configuration

Below you can find instruction on how to configure many auth options availble
on vetis.

### stdout

#### Using API

```rust,ignore
use vetis_encode::HookConfig as EncodeHookConfig;

let config = EncodeHookConfig::builder()
    .build()?;
```

#### vetis.yaml

```yaml
- name: "encode"
  matcher:
    type: "path"
    paths:
      - "/protected*"
```

#### Configuration Options

- **matcher**: If set, this hook will be only called if request meet the criteria defined by him, otherwise, all requests will call this hook (optional)

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
