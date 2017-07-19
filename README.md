# Novluno
The Novluno project is currently active in reverse-engineering of the data files of the game "Redmoon Online".

# File Parsers
The folder `core_compat` contains a rust crate to help with parsing of the original file formats.
The layout of some of the formats are not 100% correct but should be close enough for a useful result.
Any help in this area is appreciated.

## File types than can be successfully parsed:
- List files (*.lst)
- Sprite files (*.rle)
- Data files (*.rmd)
- Map files (*.rmm)

## Remaining file types which need parsers:
- Sound files (*.rms)
- Midi file (*.mid)
- Info files (*.rmi)

## Data flow
The flow of data is somewhat convoluted, but goes something like this.
- The map files tiles (RMM) hold references to entries in the respective data files (RMD) for tiles and objects.
- The data files (RMD) hold references to an index in the list (LST) files.
- The list files (LST) hold specific mappings from the type's id number to the file and index in the file for the RLE data.

RMM -> RMD -> LST -> RLE

# Viewer appliation
In the folder `rm_viwer` is a utility written using the GTK bindings.
The viewer can currently only view RLE files.
Open up one of the folders with RLE files in them to get a listing of the RLE files.


Select a file to load the RLE.
Use the left and right arrow keys to change the current index of the RLE.

## Building the `rm_viwer`
The rm viewer may be difficult to build on any other platform besides linux due to the dependency on GTK.
I have not played around with any other sort of cross platform solution but I welcome to any ideas.

# Required External Files
The project expects the original data files of the game to be in the `./data` directory.
The data files which the parsers are based upon come from verson 3.9 of the game.
If any of these parsers don't work with a specific file please file an issue;
I have noticed that there are some file which advertise the wrong format header.


At the moment, the data files are required to run the tests in this project!


The game files are not supplied with this project!


# Contributors
- Brian Steffens: For the initial RLE file format.
- Robin Brunekreeft: For the great amount of help with the other various file formats.
