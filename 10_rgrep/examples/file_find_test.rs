use std::path::PathBuf;
use std::str::FromStr;
use glob::glob;

fn main() {
    let path = "**/test/*.text";
    let path_buf = PathBuf::from_str(path).unwrap();

    let file_name = path_buf.file_name().unwrap();
    println!("{}", file_name.to_str().unwrap());

    // let dir = path_buf.read_dir().unwrap();
    // let result = std::fs::read_dir(dir.into()).unwrap();

    // for entry in dir {
    //     let  entry = entry?;
    //     let entry_path = entry.path();
    //
    // }


    // 通配符查找文件
    let paths= glob(path).unwrap();
    for g in paths {
        println!("{}", g.unwrap().display());
    }
}