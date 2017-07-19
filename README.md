# Novluno
The Novluno project is currently active in reverse-engineering of the data files of an old game "Redmoon Online".

## Building

### Core Compat(ability)
This folder `core_compat` contains a rust crate to help with parsing of the original file formats.
The layout of some of the formats are not 100% correct but should be close enough for a useful result.
Any help in this area is appreciated.

The Current file types than can be successfully parsed are:
- List files (*.lst)
- Sprite files (*.rle)
- Data files (*.rmd)
- Map files (*.rmm)

### Required External Files
The project expects the original data files of the game to be in the `./data` directory.
These files are not supplied with this project!


## Contributors
brian steffens: For the initial RLE file format.
Robin Brunekreeft: For the great amount of help with the other various file formats.
