
#[derive(Debug, Eq, PartialEq)]
pub struct MapTile {
    pub object_file_num: u32, // points to a `/DATAs/Obj/*.rmd
    pub object_file_idx: u32, // this is an index into this file
    pub tile_file_num: u32,   // points to a `/DATAs/Tle/*.rmd
    pub tile_file_idx: u32,   // this is an index into this file
    pub warp: u32,
    pub collision: u32,
}
