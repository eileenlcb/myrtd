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

pub fn update_item(item:Item) -> Result<()>{
    let to_update_item = get_item_by_id(item.id)?;
    let offset = get_offset_by_id(item.id)?;
    Csv::new()?.splice(offset as u64, to_update_item.to_string().len() as u64, &item.to_string())
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

pub fn get_item_by_id(id:u32) -> Result<Item>{
    let content = Csv::new()?.content()?;
    let item_str = content.lines().find(|line| {
        if let Ok(item) = line.parse::<Item>() {
            item.id() == id
        } else {
            false
        }
    });
    if let Some(item_str) = item_str {
        Ok(item_str.parse().unwrap())
    }else {
        Err(io::Error::new(io::ErrorKind::NotFound, "item not found"))
    }
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

    fn splice(&mut self, offset: u64, delete_size: u64, write_content: &str) -> Result<()> {
        use std::io::SeekFrom;
        let file = &self.file;

        // Create a buffered reader form csv file
        let mut reader = BufReader::new(file);

        // Adjust the appropriate reading position
        reader.seek(SeekFrom::Start(offset + delete_size))?;

        // Save the rest of the file,
        // starting at the position after the last character that was deleted
        let mut rest_content = String::new();
        reader.read_to_string(&mut rest_content)?;

        // The final to be write content is spliced
        // by the `write_content` and the `rest_content`
        let write_content = write_content.to_owned() + &rest_content;

        // Create a buffered writer from csv file
        let mut writer = BufWriter::new(file);

        // Adjust the appropriate writing position
        writer.seek(SeekFrom::Start(offset))?;

        // Insert `write_content` and overwrite old file content
        writer.write_all(write_content.as_bytes())?;

        // Make sure there is no redundant old file content left
        file.set_len(offset + write_content.len() as u64)?;

        Ok(())
    }
}


fn get_offset_by_id(id:u32) ->Result<usize>{
    let mut csv = Csv::new()?;
    let content = csv.content()?;
    let prev_lines = content.lines().take_while(|line| {
        if let Ok(item) = line.parse::<Item>() {
            item.id() != id
        } else {
            true
        }
    });

    let offset = prev_lines.map(|line| line.len() + 1).sum();
    Ok(offset)
}

// pub enum StorageError {
//     FileHandle(FileHandleError),
//     ParseItem(ParseItemError),
//     ItemNoExist(u32),
// }