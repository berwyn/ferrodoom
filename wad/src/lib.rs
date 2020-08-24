mod lumps;

enum WadType {
    Internal,
    Patch,
}

struct WadDirectory {
    file_location: i32,
    size: i32,
    name: String,
}

struct Wad {
    lump_count: i32,
    directory_location: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
