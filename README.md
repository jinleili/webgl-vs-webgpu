## 测试场景的代码路径
```path
./webgl-vs-webgpu/src/comparative_scenario/
```

## 桌面端运行

```sh
cargo run --bin webgl-vs-webgpu
```
**桌面端以 OpenGL ES 模式运行**
需要电脑上已编译并安装好了 [Angle](https://github.com/google/angle/blob/main/doc/DevSetup.md)：
```sh
WGPU_BACKEND=gl cargo run --bin webgl-vs-webgpu --features=webgl
```

## Web 运行

```sh
# 添加构建目标
rustup target add wasm32-unknown-unknown
# 安装 trunk 构建工具
cargo install --locked trunk

# 使用自定义 src/index.html 模板构建，并自动打开 Chrome 浏览器运行（需要电脑上已安装 Chrome 113+）
sh ./run-wasm-local.sh

# 构建并运行, 但不会使用自定义模板，也不会自动打开浏览器（可用于测试 wasm 构建）
cargo run-wasm
```
