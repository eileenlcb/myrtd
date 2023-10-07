use crate::model::{self,*};
use std::env::{self, VarError};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::{self,BufReader, BufWriter, Read, Seek, Write, BufRead};
use std::path::Path;

const CSV_FILE_NAME: &str = "rtd.csv";
type Result<T> = std::result::Result<T, io::Error>;

pub fn add_item(item:Item) -> Result<()>{
    let mut csv = Csv::new()?;
    //将文件指针移动到末尾
    csv.file.seek(std::io::SeekFrom::End(0))?;
    writeln!(csv.file, "{}", item.to_string())?;
    Ok(())
}

#[allow(unused)]
struct Csv{
    filename:String,
    file:File,
}

pub fn get_max_id() -> Result<u32>{
    let max_id = get_all()?
        .iter()
        .map(|item| item.id)
        .max()
        .unwrap_or(0);
    Ok(max_id)
}

pub fn get_all() -> Result<Vec<Item>>{
    Ok(Csv::new()?
        .content()?
        .lines()
        .filter_map(|line| line.parse::<Item>().ok())
        .collect())
}

impl Csv {
    fn new() -> Result<Self>{
        let filename = Csv::filename()?;
        let path  = Path::new(&filename);
        println!("path is in {:?}",path);

        if !path.exists(){
            let mut file = Csv::create(path)?;
            //将字符串字面量理解为字节序列，不会进行utf-8编码，适用于文件写入
            file.write_all(b"id,name,completed,deleted,createdAt,completedAt,deletedAt\n")?;
            Ok(Self{
                filename:filename.to_string(),
                file,
            })
        }else{
            Ok(Self {
                filename: filename.to_string(),
                file: Csv::open(path)?,
            })
        }
    }

    fn create(path:&Path) -> Result<fs::File>{
        let csv = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
        Ok(csv)
    }

    fn open(path:&Path) -> Result<fs::File>{
        let csv = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
        Ok(csv)
    }

    fn filename() -> Result<String>{
        //io.error不能用？隐式转换
        let home = env::var("HOME").unwrap();
        let filename = home + "/" + CSV_FILE_NAME;
        println!("filename:{}", filename);
        Ok(filename)
    }

    fn content(&mut self) -> Result<String>{
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;
        Ok(content)
    }
}



// pub enum StorageError {
//     FileHandle(FileHandleError),
//     ParseItem(ParseItemError),
//     ItemNoExist(u32),
// }