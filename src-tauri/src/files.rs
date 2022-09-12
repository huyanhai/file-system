use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct PathInfo {
    name: String,
    path: String,
    is_dir: bool,
}

/**
 * 项目根路径
 */
pub fn get_root_dir() -> String {
    return Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .display()
        .to_string();
}

/**
 * 文件列表
 */
pub fn dir_lists(name: &str) -> Vec<PathInfo> {
    let root = get_root_dir();
    let targe_dir = Path::new(root.as_str()).join(name);
    let targe = &targe_dir;

    let mut files: Vec<PathInfo> = Vec::new();
    if Path::new(targe_dir.as_path()).exists() {
        let file_list = fs::read_dir(targe).expect("目录不存在");

        for path in file_list {
            let p = path.unwrap().path().display().to_string();
            let infos: Vec<&str> = p.split("/").collect();

            let dir = PathInfo {
                name: infos[infos.len() - 1].to_string(),
                path: targe.display().to_string().replace(root.as_str(), ""),
                is_dir: Path::new(p.as_str()).is_dir(),
            };

            files.push(dir);
        }
        return files;
    }
    return files;
}

/**
 * 读取文件内容
 */
pub fn file_content(path: &str) -> String {
    let root = get_root_dir();
    let targe_dir = Path::new(root.as_str()).join(path);
    let content = fs::read_to_string(targe_dir).expect("读取文件错误");

    return content;
}
