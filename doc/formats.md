# Redmoon Data formats

## General : Data flow
The flow of data is somewhat convoluted, but goes something like this.
- The map files tiles (RMM) hold references to entries in the respective data files (RMD) for tiles and objects.
- The data files (RMD) hold references to an index in the list (LST) files.
- The list files (LST) hold specific mappings from the type's id number to the file and index in the file for the RLE data.

RMM -> RMD -> LST -> RLE