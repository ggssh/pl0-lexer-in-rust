use std::fs;

// pub type Result<T> = Result<T,>;

// 从文件中读入源代码
// todo:将返回值类型改为Result
pub fn read_in(path: String) -> String {
    if let Ok(res) = fs::read_to_string(path) {
        return res;
    } else {
        return String::from("");
    }
    // String::new()
}
// 将词法分析的结果写入文件output.txt
pub fn write_out(path: String) {}