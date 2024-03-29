
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;

use core_compat::entity::resource_file::ResourceFile;
use core_compat::parser::rle::parse_rle;

use app_error::AppError;

pub fn read_rle_from_bytes( data: &[u8] ) -> Result<ResourceFile, AppError> {
    let rle = parse_rle(0, data)?;
    Ok(rle)
}

pub fn read_rle_from_file( file: &PathBuf ) -> Result<ResourceFile, AppError> {
    let mut file = File::open(file)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let rle = parse_rle(0, &data)?;
    Ok(rle)
}
