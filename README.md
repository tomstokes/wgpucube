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

## Platform-Specific Quirks and Workarounds

### iOS

#### Broken `request_redraw()` during `WindowEvent::RedrawRequested`
Calling `window.request_redraw()` while handling a `WindowEvent::RedrawRequested` event does not work on iOS in winit v0.30. This breaks the common method of calling `request_redraw()` after handling each `WindowEvent::RedrawRequested` event for continuous rendering. Setting the event loop to `ControlFlow::Poll` is not a workaround because `WindowEvent::RedrawRequested` will not be sent on each event loop wakeup.

The workaround used in this project adds a `request_redraw` boolean flag to the Application structure that is set to `true` after each `WindowEvent::RedrawRequested` event is handled. Then the `about_to_wait` handler checks this flag and calls `window.request_redraw()` if it is set. Calling `window.request_redraw()` from `about_to_wait()` appears to work.

Winit tracking issue: https://github.com/rust-windowing/winit/issues/3406
