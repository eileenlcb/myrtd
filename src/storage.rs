use crate::model::{self,*};
use std::env::{self, VarError};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::{self,BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;

const CSV_FILE_NAME: &str = ".rtd.csv";
type Result<T> = std::result::Result<T, io::Error>;

pub fn add_item(item:Item) -> Result<()>{
    let mut csv = Csv::new()?;
    csv.file.seek(std::io::SeekFrom::End(0))?;
    writeln!(csv.file, "{}", item.to_string())?;
    Ok(())
}

#[allow(unused)]
struct Csv{
    filename:String,
    file:File,
}

impl Csv {
    fn new() -> Result<Self>{
        let filename = Csv::filename()?;
        let path  = Path::new(&filename);

        if !path.exists(){
            let mut file = Csv::create(path)?;
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
        Ok(filename)
    }
}



// pub enum StorageError {
//     FileHandle(FileHandleError),
//     ParseItem(ParseItemError),
//     ItemNoExist(u32),
// }