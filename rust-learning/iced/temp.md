# iced生态

本文档介绍冰生态系统。

它快速列出图书馆的不同受众，并解释不同的板条箱如何相互关联。

用户
冰被2个不同类型的用户使用：

最终用户。他们应该能够：
快速入门，
有许多小部件可用，
保持简单，
并构建可维护且具有执行性的应用程序。
GUI 工具包开发人员/生态系统贡献者。他们应该能够：
构建新型小部件，
实现自定义运行时，
将现有的运行时集成到自己的系统中（如游戏引擎），
并创建自己的自定义渲染器。
箱子
冰由不同的板条箱组成，为我们的用户提供不同的抽象层。这种模块化架构有助于我们隐藏和分离实现细节，这将允许我们在未来重写或更改策略。

生态系统图

iced_core
iced_core持有公共 API 的基本可重用类型。例如，基本数据类型，如点、矩形、长度等。

此箱子是冰化运行时的起点。

iced_native
iced_native需要iced_core，并在它之上构建本机运行时，具有：

自定义布局引擎，深受德鲁伊的启发
所有内置小部件的事件处理
与渲染器无关的 API
为此，它引入了一组可重用的接口：

一个 Widget 特性，用于实现新的小部件：从布局要求到事件和绘图逻辑。
一堆渲染器特性，旨在保持箱子渲染器与不可知性。
窗口特征，利用原始窗口句柄，可以通过面向窗口的图形呈现器实现。基于窗口的外壳（如iced_winit）可以使用此特征保持与渲染器无关。
iced_web
iced_web需要iced_core，并在顶部构建 WebAssembly 运行时。它通过引入可用于生成 VDOM 节点的 Widget 特性来实现此目的。

箱子目前是一个简单的抽象层，超过ddrio。

iced_wgpu
iced_wgpu是iced_native的 wgpu 渲染器。目前，它是本机平台中 Iced 的默认渲染器。

wgpu 支持大多数现代图形后端：Vulkan、金属、DX11 和 DX12（OpenGL 和 WebGL 仍然是 WIP）。此外，它将支持传入的 WebGPU API。

目前，iced_wgpu支持以下基元：

文本，使用wgpu_glyph呈现。根本没有整形。
四边形或矩形，具有圆角边框和纯背景色。
图像，从文件系统中懒洋洋地加载。
剪辑区域，可用于实现可滚动或隐藏溢出内容。
iced_winit
iced_winit在使用 winit 时提供一些iced_native快速入门开发方面的一些方便的抽象。

它公开了一个与渲染器无关的应用程序特征，该特征可以实现，然后使用简单的调用运行。这种特性的使用是可选的。为决定实现自定义事件循环的用户提供转换模块。

冰
最后，iced 将所有内容合并为一个简单的抽象，以创建跨平台应用程序：

在本机上，它使用iced_winit和iced_wgpu。
在网络上，它使用iced_web。
这是供最终用户使用的箱子。