#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct CustomJsTriggerEvent {
    ty: &'static str,
    _value: String,
}

/// 虽然将 web 与 native 环境的代码写到一起能完全消除重复代码，
/// 但穿插的大量 `#[cfg(target_arch = "wasm32")]` 编译条件使得代码不利于阅读
#[cfg_attr(not(target_arch = "wasm32"), path = "native.rs")]
#[cfg_attr(target_arch = "wasm32", path = "web.rs")]
mod run;
pub use run::*;
