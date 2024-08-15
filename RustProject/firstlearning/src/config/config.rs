// config.rs

use std::fs;
use std::path::Path;

pub struct AppConfig {
    // 定义配置字段
    pub some_setting: String,
}

impl AppConfig {
    pub fn new() -> Self {
        // 从文件中读取配置并初始化结构体
        let path = Path::new("src/config/app_config.yaml");
        let content = fs::read_to_string(path).expect("无法读取配置文件");
        // 解析配置内容并设置结构体字段
        AppConfig {
            some_setting: content,
        }
    }
}