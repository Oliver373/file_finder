use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 将配置文件复制到工作目录下
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    let config_file_src = Path::new(".").join("config.toml");
    let config_file_dest = target_dir.join("config.toml");

    fs::copy(&config_file_src, &config_file_dest).expect("Failed to copy config.toml");
}
