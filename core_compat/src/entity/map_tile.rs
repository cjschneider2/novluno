use crate::entity::entry::Entry;

#[derive(Debug, Eq, PartialEq)]
pub struct MapTile {
    pub obj_rmd_entry: Entry,
    pub tle_rmd_entry: Entry,
    pub warp: u32,
    pub collision: u32,
}

// NOTE: The `Entry` struct Looks something like :
//       Entry {
//           file_num: u32,
//           file_idx: u32,
//       }
// Where for an object file for example, `file_num` points to a
// `/DATAs/Obj/*.rmd file and `file_idx` to an index in this file.
