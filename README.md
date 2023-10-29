## 测试场景的代码路径

```path
./webgl-vs-webgpu/src/comparative_scenario/
```

## 安装 Wasm 构建工具

```sh
# 添加构建目标
rustup target add wasm32-unknown-unknown
# 安装构建工具
cargo install wasm-bindgen-cli --version=0.2.87
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

## Wasm 编译并运行

```sh
# 构建 webgpu 包并运行 (不会使用自定义模板，也不会自动打开浏览器)
cargo run-wasm
# 构建 webgl 包并运行
cargo run-wasm --features=webgl
```

## 以网站形式运行
```sh
npm install
npm run docs:dev 
```