# Ecosystem

  iced生态



This document describes the Iced ecosystem.

介绍Iced相关的生态环境。目的是为明确适合那些使用者，以及解释不同的组装之间相互的关系。

It quickly lists the different audiences of the library and explains how the different crates relate to each other.



## [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#users)Users

使用者：

两种典型的代表：

Iced is meant to be used by 2 different types of users:

-   **End-users**. They should be able to:
    -   get started quickly,
    -   have many widgets available,
    -   keep things simple,
    -   and build applications that are  **maintainable**  and  **performant**.
-   **GUI toolkit developers / Ecosystem contributors**. They should be able to:
    -   build new kinds of widgets,
    -   implement custom runtimes,
    -   integrate existing runtimes in their own system (like game engines),
    -   and create their own custom renderers.

## [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#crates)Crates

Iced consists of different crates which offer different layers of abstractions for our users. This modular architecture helps us keep implementation details hidden and decoupled, which should allow us to rewrite or change strategies in the future.

[![Ecosystem graph](https://github.com/hecrj/iced/raw/master/docs/graphs/ecosystem.png)](https://github.com/hecrj/iced/blob/master/docs/graphs/ecosystem.png)

### [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#iced_core)[`iced_core`](https://github.com/hecrj/iced/blob/master/core)

[`iced_core`](https://github.com/hecrj/iced/blob/master/core)  holds basic reusable types of the public API. For instance, basic data types like  `Point`,  `Rectangle`,  `Length`, etc.

This crate is meant to be a starting point for an Iced runtime.

### [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#iced_native)[`iced_native`](https://github.com/hecrj/iced/blob/master/native)

[`iced_native`](https://github.com/hecrj/iced/blob/master/native)  takes  [`iced_core`](https://github.com/hecrj/iced/blob/master/core)  and builds a native runtime on top of it, featuring:

-   A custom layout engine, greatly inspired by  [`druid`](https://github.com/xi-editor/druid)
-   Event handling for all the built-in widgets
-   A renderer-agnostic API

To achieve this, it introduces a bunch of reusable interfaces:

-   A  `Widget`  trait, which is used to implement new widgets: from layout requirements to event and drawing logic.
-   A bunch of  `Renderer`  traits, meant to keep the crate renderer-agnostic.
-   A  `Windowed`  trait, leveraging  [`raw-window-handle`](https://github.com/rust-windowing/raw-window-handle), which can be implemented by graphical renderers that target  _windows_. Window-based shells (like  [`iced_winit`](https://github.com/hecrj/iced/blob/master/winit)) can use this trait to stay renderer-agnostic.

### [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#iced_web)[`iced_web`](https://github.com/hecrj/iced/blob/master/web)

[`iced_web`](https://github.com/hecrj/iced/blob/master/web)  takes  [`iced_core`](https://github.com/hecrj/iced/blob/master/core)  and builds a WebAssembly runtime on top. It achieves this by introducing a  `Widget`  trait that can be used to produce VDOM nodes.

The crate is currently a simple abstraction layer over  [`dodrio`](https://github.com/fitzgen/dodrio).

### [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#iced_wgpu)[`iced_wgpu`](https://github.com/hecrj/iced/blob/master/wgpu)

[`iced_wgpu`](https://github.com/hecrj/iced/blob/master/wgpu)  is a  [`wgpu`](https://github.com/gfx-rs/wgpu-rs)  renderer for  [`iced_native`](https://github.com/hecrj/iced/blob/master/native). For now, it is the default renderer of Iced in native platforms.

[`wgpu`](https://github.com/gfx-rs/wgpu-rs)  supports most modern graphics backends: Vulkan, Metal, DX11, and DX12 (OpenGL and WebGL are still WIP). Additionally, it will support the incoming  [WebGPU API](https://gpuweb.github.io/gpuweb/).

Currently,  [`iced_wgpu`](https://github.com/hecrj/iced/blob/master/wgpu)  supports the following primitives:

-   Text, which is rendered using  [`wgpu_glyph`](https://github.com/hecrj/wgpu_glyph). No shaping at all.
-   Quads or rectangles, with rounded borders and a solid background color.
-   Images, lazily loaded from the filesystem.
-   Clip areas, useful to implement scrollables or hide overflowing content.

### [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#iced_winit)[`iced_winit`](https://github.com/hecrj/iced/blob/master/winit)

[`iced_winit`](https://github.com/hecrj/iced/blob/master/winit)  offers some convenient abstractions on top of  [`iced_native`](https://github.com/hecrj/iced/blob/master/native)  to quickstart development when using  [`winit`](https://github.com/rust-windowing/winit).

It exposes a renderer-agnostic  `Application`  trait that can be implemented and then run with a simple call. The use of this trait is optional. A  `conversion`  module is provided for users that decide to implement a custom event loop.

### [](https://github.com/hecrj/iced/blob/master/ECOSYSTEM.md#iced)[`iced`](https://github.com/hecrj/iced/blob)

Finally,  [`iced`](https://github.com/hecrj/iced/blob)  unifies everything into a simple abstraction to create cross-platform applications:

-   On native, it uses  [`iced_winit`](https://github.com/hecrj/iced/blob/master/winit)  and  [`iced_wgpu`](https://github.com/hecrj/iced/blob/master/wgpu).
-   On the web, it uses  [`iced_web`](https://github.com/hecrj/iced/blob/master/web).

This is the crate meant to be used by  **end-users**.