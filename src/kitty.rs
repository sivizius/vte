//! The Kitty-Part is defined in the backend (vte-parser) and not frontend (terminal-application),
//!   because I think, the parsing-part is huge enough to be justified to define this here.
//! Of course, another way to solve this is by consuming the full Application Program Command and
//!   letting the frontend decide, how to deal with it.
//! This way it might be easier to add other APCs,
//!   because nothing have to be changed here.
//! On the other hand, the frontend would be full of parsers then.

use core::fmt;

/// A Kitty Object, which will be build by vte-parse, but handled by performer.
#[derive(Debug)]
pub struct Kitty {
    /// The overall action this graphics command is performing (`a` key, default is `t`).
    pub action:       KittyAction,
    /// The format in which the image data is sent (`f` key, default is `32`).
    /// Values can be `24` for RGB, `32` for RBGA and `100` for PNG.
    pub format:       usize,
    /// The transmission medium used (`t` key, default is `d`).
    pub medium:       KittyMedium,
    /// The width of the image being sent (`s` key, default is `0`).
    pub width:        usize,
    /// The height of the image being sent (`v` key, default is `0`).
    pub height:       usize,
    /// The size of data to read from a file (`S` key, default is `0`).
    pub size:         usize,
    /// The offset from which to read data from a file (`O` key, default is `0`).
    pub offset:       usize,
    /// The image id (`i` key, default is `0`).
    /// A valid id can be an arbitrary positive integer up to 4294967295, but must not be zero.
    pub id:           usize,
    /// The type of data compression (`o` key, default is not set).
    pub compression:  KittyCompression,
    /// Whether there is more chunked data available (`m` key, default is `0`).
    /// Valid values are `0` for `false` and `1` for `true`.
    pub chunked:      bool,
    /// The left edge (in pixels) of the image area to display (`x` key, default is `0`).
    pub x_pos:        usize,
    /// The top edge (in pixels) of the image area to display (`y` key, default is `0`).
    pub y_pos:        usize,
    /// The width (in pixels) of the image area to display (`w` key, default is `0`).
    /// By default, the entire width is used.
    pub x_size:       usize,
    /// The height (in pixels) of the image area to display (`h` key, default is `0`).
    /// By default, the entire height is used.
    pub y_size:       usize,
    /// The x-offset within the first cell at which to start displaying the image (`X` key, default is `0`).
    pub x_offset:     usize,
    /// The y-offset within the first cell at which to start displaying the image (`Y` key, default is `0`).
    pub y_offset:     usize,
    /// The number of columns to display the image over (`c` key, default is `0`).
    pub columns:      usize,
    /// The number of rows to display the image over (`r` key, default is `0`).
    pub rows:         usize,
    /// The z-index vertical stacking order of the image (`z` key, default is `0`).
    pub z_index:      usize,
    /// What to delete (`d` key, default is `a`).
    /// Valid values are `a`/`A`, `c`/`C`, `i`/`I`, `p`/`P`, `q`/`Q`, `x`/`X`, `y`/`Y` and `z`/`Z`.
    pub delete:       KittyDelete,
}

/// Constructor for `Kitty`.
#[allow(non_snake_case)]
pub fn Kitty() -> Kitty {
    Kitty {
        action:       KittyAction::Immediate,
        format:       32,
        medium:       KittyMedium::Direct,
        width:        0,
        height:       0,
        size:         0,
        offset:       0,
        id:           0,
        compression:  KittyCompression::None,
        chunked:      false,
        x_pos:        0,
        y_pos:        0,
        x_size:       0,
        y_size:       0,
        x_offset:     0,
        y_offset:     0,
        columns:      0,
        rows:         0,
        z_index:      0,
        delete:       KittyDelete::All,
    }
}

impl Default for Kitty {
    fn default() -> Kitty {
        Kitty()
    }
}

impl fmt::Display for Kitty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!
        (
            f,
            "action: {}, format: {}, medium: {}, width: {}, height: {}, size: {}, offset: {}, id: {}, compression: {}, chunked: {}, x_pos: {}, y_pos: {}, x_size: {}, y_size, x_offset: {}, y_offset: {}, columns: {}, rows: {}, z_index: {}, delete: {} {}",
            match self.action
            {
              KittyAction::Delete       =>  "Delete",
              KittyAction::Immediate    =>  "Immediate",
              KittyAction::Load         =>  "Load",
              KittyAction::Query        =>  "Query",
              KittyAction::Store        =>  "Store",
            },
            self.format,
            match self.medium
            {
              KittyMedium::Direct       =>  "Direct",
              KittyMedium::File         =>  "File",
              KittyMedium::SharedMemory =>  "Shared Memory",
              KittyMedium::Temporary    =>  "Temporary File",
            },
            self.width,
            self.height,
            self.size,
            self.offset,
            self.id,
            match self.compression
            {
              KittyCompression::None    =>  "None",
              KittyCompression::ZLib    =>  "ZLib",
            },
            self.chunked,
            self.x_pos,
            self.y_pos,
            self.x_size,
            self.y_size,
            self.x_offset,
            self.y_offset,
            self.columns,
            self.rows,
            self.z_index,
            match self.delete
            {
              KittyDelete::All              =>  "All",
              KittyDelete::ById             =>  "By ID",
              KittyDelete::ByZIndex         =>  "By Z-Index",
              KittyDelete::IntersectCell2D  =>  "Intesect Cell",
              KittyDelete::IntersectCell3D  =>  "Inersect Cell and Z-Index",
              KittyDelete::IntersectColumn  =>  "Intersect Column",
              KittyDelete::IntersectCursor  =>  "Intersect Cursor",
              KittyDelete::IntersectRow     =>  "Intersect Row",
            },
        )
    }
}

/// Specify the action of this `Kitty`.
#[derive(Debug)]
pub enum KittyAction {
    /// Draw image imediatly on screen (`a=t`,default).
    Immediate,
    /// Delete no, one or multiple images (`a=d`).
    Delete,
    /// Draw a stored image on screen (`a=p`).
    Load,
    /// Ask for various data (`a=q`).
    Query,
    /// Store a image, can later be drawn with a load (`a=T`).
    Store,
}

/// Compression used for stream.
/// The client can send compressed image data to the terminal emulator, by specifying the `o` key.
/// Currently, only zlib based deflate compression is supported, which is specified using `o=z`.
#[derive(Debug)]
pub enum KittyCompression {
    /// Uncompressed stream, might be compressed PNG (default).
    None,
    /// Compressed with zlib (`o=z`).
    ZLib,
}

/// Delete some images.
/// Images can be deleted by using the delete action `a=d`.
/// If specified without any other keys, it will delete all images visible on screen.
/// To delete specific images, use the `d` key as described below.
#[derive(Debug)]
pub enum KittyDelete {
    /// Delete all images visible on screen (`d=a/A`, default).
    All,
    /// Delete all images with the specified id, specified using the `i` key (`d=i`/`d=I`).
    ById,
    /// Delete all images that intersect with the current cursor position (`d=c`/`d=C`).
    IntersectCursor,
    /// Delete all images that intersect a specific cell, the cell is specified using the `x` and `y` keys (`d=p`/`d=P`).
    IntersectCell2D,
    /// Delete all images that intersect a specific cell having a specific z-index.
    /// The cell and z-index is specified using the `x`, `y` and `z` keys (`d=q`/`d=Q`).
    IntersectCell3D,
    /// Delete all images that intersect the specified column, specified using the `x` key (`d=x`/`d=X`).
    IntersectColumn,
    /// Delete all images that intersect the specified row, specified using the `y` key (`d=y`/`d=Y`).
    IntersectRow,
    /// Delete all images that have the specified z-index, specified using the `z` key (`d=z`/`d=Z`).
    ByZIndex,
}

/// The transmission medium is specified using the `t` key.
#[derive(Debug)]
pub enum KittyMedium {
    /// Direct: The data is transmitted within the escape code itself (`t=d`, default).
    Direct,
    /// A simple file (`t=f`).
    File,
    /// A temporary file, the terminal emulator will delete the file after reading the pixel data (`t=t`).
    /// For security reasons the terminal emulator should only delete the file if it is in a known temporary directory,
    ///   such as `/tmp`, `/dev/shm`, `TMPDIR` environment variable if present and any platform specific temporary directories.
    Temporary,
    /// A POSIX shared memory object. The terminal emulator will delete it after reading the pixel data (`t=s`).
    SharedMemory,
}
