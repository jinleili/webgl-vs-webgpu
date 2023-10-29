use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
mod wgsl_preprocess;
use wgsl_preprocess::preprocess_wgsl;

// build.rs 配置：https://blog.csdn.net/weixin_33910434/article/details/87943334
fn main() -> Result<()> {
    // 这一行告诉 cargo 如果 /wgsl/ 目录中的内容发生了变化，就重新运行脚本
    println!("cargo:rerun-if-changed=../assets/wgsl/");
    let _ = preprocess_wgsl();

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["../assets/preprocessed-wgsl/"];

    let out_dir = std::path::Path::new("../docs/public/assets/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;
    Ok(())
}
