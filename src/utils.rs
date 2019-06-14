use crate::constants::*;
use crate::errors::*;
use log::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_export]
macro_rules! fatal {
    ($msg:tt) => {{
        error!("{} in file {} line {}", $msg, file!(), line!());
        clean();
        std::process::exit(1)
    }};
}

pub fn clean() {
    info!("remove pid and socket file");
    let _ = remove_file(OUT_DIR.join(PID_FILE));
    let _ = remove_file(OUT_DIR.join(SOCKET_FILE));
}

pub fn clean_and_exit() {
    clean();
    info!("exit ...");
    std::process::exit(0);
}

pub fn remove_file<P: AsRef<Path>>(p: P) -> Result {
    if p.as_ref().exists() {
        std::fs::remove_file(p).map_err(|e| {
            error!("{}", e);
            Error::RemoveFileError
        })
    } else {
        Err(Error::FileNotExistError)
    }
}

pub fn read_file<P: AsRef<Path>>(p: P) -> Result<String> {
    File::open(p)
        .map_err(|e| {
            error!("open file error {}", e);
            Error::OpenFileError
        })
        .and_then(|mut f: File| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).map_err(|e| {
                error!("read to string error {}", e);
                Error::ReadToStringError
            })?;
            Ok(buf)
        })
}
