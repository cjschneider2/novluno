# Novluno
The Novluno project is currently active in reverse-engineering of the data files of the game "Redmoon Online" and creating a modern client to run the with the original files.

# File Parsers
The folder `core_compat` contains a rust crate to help with parsing of the original file formats.
The layout of some of the formats are not 100% correct but should be close enough for a useful result.
Any help in this area is appreciated.

## File types than can be successfully parsed:
- List files (*.lst)
- Sprite files (*.rle)
- Data files (*.rmd)
- Map files (*.rmm)
- Info files (*.rmi)

## Remaining file types which need parsers:
- Sound files (*.rms)
- Midi file (*.mid)

# Required External Files
The project expects the original data files of the game to be in the `./data` directory.
The data files which the parsers are based upon come from verson 3.9 of the game.
If any of these parsers don't work with a specific file please file an issue;
I have noticed that there are some file which advertise the wrong format header.

## Notes

* The game files are not supplied with this project!
* You may have to 'normalize' the filenames of some of the data files by hand, to the lower/uppercase standard of the rest of the files of that type.
* At the moment, the data files are required to run the tests in this project!


# Contributors
- Brian Steffens: For the initial RLE file format.
- Robin Brunekreeft: For the great amount of help with the other various file formats.
