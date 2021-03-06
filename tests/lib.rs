extern crate tiled;

use std::path::Path;
use std::fs::File;
use tiled::{Map, TiledError, PropertyValue, parse, parse_file, parse_tileset};

fn read_from_file(p: &Path) -> Result<Map, TiledError> {
    let file = File::open(p).unwrap();
    return parse(file);
}

fn read_from_file_with_path(p: &Path) -> Result<Map, TiledError> {
    return parse_file(p);
}

#[test]
fn test_gzip_and_zlib_encoded_and_raw_are_the_same() {
    let z = read_from_file(&Path::new("assets/tiled_base64_zlib.tmx")).unwrap();
    let g = read_from_file(&Path::new("assets/tiled_base64_gzip.tmx")).unwrap();
    let r = read_from_file(&Path::new("assets/tiled_base64.tmx")).unwrap();
    let c = read_from_file(&Path::new("assets/tiled_csv.tmx")).unwrap();
    assert_eq!(z, g);
    assert_eq!(z, r);
    assert_eq!(z, c);
}

#[test]
fn test_external_tileset() {
    let r = read_from_file(&Path::new("assets/tiled_base64.tmx")).unwrap();
    let e = read_from_file_with_path(&Path::new("assets/tiled_base64_external.tmx")).unwrap();
    assert_eq!(r, e);
}

#[test]
fn test_just_tileset() {
    let r = read_from_file(&Path::new("assets/tiled_base64.tmx")).unwrap();
    let t = parse_tileset(File::open(Path::new("assets/tilesheet.tsx")).unwrap(), 1).unwrap();
    assert_eq!(r.tilesets[0], t);
}

#[test]
fn test_tile_property() {
    let r = read_from_file(&Path::new("assets/tiled_base64.tmx")).unwrap();
    let prop_value: String = if let Some(&PropertyValue::StringValue(ref v)) = r.tilesets[0].tiles[0].properties.get("a tile property") {
        v.clone()
    } else {
        String::new()
    };
    assert_eq!("123", prop_value);
}
