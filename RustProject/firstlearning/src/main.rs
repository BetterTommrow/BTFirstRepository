mod config;
use config::config::AppConfig;

use std::collections::HashMap;


fn main() {
    let cfg = AppConfig::new();
    // 使用配置
    let sql_dict = parse_sql_dict(&cfg.some_setting);
    // 打印解析后的结果
    for (server_name, server_info) in sql_dict {
        println!("服务器: {}", server_name);
        for (key, value) in server_info {
            println!("  {}: {}", key, value);
        }
    }
    // println!("配置中的设置: {}", cfg.some_setting);


    println!("Hello, world!");
}


fn parse_sql_dict(config: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::new();
    // 以 `SQL_DICT:` 为分隔符，获取后面的部分
    let sql_dict_content = config.split("SQL_DICT:").nth(1).unwrap();
    // 按行分割字符串
    for line in sql_dict_content.lines() {
        // 跳过空行
        if line.trim().is_empty() {
            continue;
        }
        // 以 `:` 分割行内容，获取服务器名称和其对应的键值对字符串
        let (server_name, key_value_pairs) = line.split_once(':').unwrap();
        let mut server_info = HashMap::new();
        // 以 `,` 分割键值对字符串
        for pair in key_value_pairs.split(',') {
            // 再以 `=` 分割每个键值对，获取键和值
            let (key, value) = pair.split_once('=').unwrap();
            server_info.insert(key.trim().to_string(), value.trim().to_string());
        }
        result.insert(server_name.trim().to_string(), server_info);
    }
    result
}