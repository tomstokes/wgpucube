# wgpucube

wgpucube is a WebGPU demo app using winit and wgpu. It renders a shaded, spinning cube with GPU acceleration. Inspired by the original kmscube by Rob Clark.

## Overview

wgpucube uses [winit](https://github.com/rust-windowing/winit) for cross-platform window handling and [wgpu](https://github.com/gfx-rs/wgpu) as the cross-platform graphics library. Additionally, [glam](https://github.com/bitshifter/glam-rs) is used to provide fast and ergonomic linear algebra functionality.

The goal is to run on every platform that wgpu and winit support, though this is a work in progress.

## Getting Started

### Desktop (Linux, Windows, macOS)

wgpucube does not currently take any command line arguments. To run in release mode, clone the repository and use cargo to run the project:

```shell
cargo run --release
```

### Other Platforms

Support for web browsers and mobile devices is on the roadmap.
