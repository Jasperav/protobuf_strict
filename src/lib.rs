use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn get_uuids() -> [&'static str; 3] {
    ["uuid", "repeated_uuids", "also_uuid"]
}

pub fn write_protos(to_dir: &Path) {
    let proto_dir = to_dir.join("protos");

    let _ = std::fs::remove_dir_all(&proto_dir);

    std::fs::create_dir(&proto_dir).unwrap();

    macro_rules! write_proto {
        ($p: expr) => {
            let content = std::include_bytes!(std::concat!("protos/", $p, ".proto"));
            let path_to_proto = proto_dir.join(concat!($p, ".proto"));
            let mut file = File::create(path_to_proto).unwrap();

            file.write(content).unwrap();
        };
    }

    write_proto!("currency");
    write_proto!("gender");
    write_proto!("order");
    write_proto!("size");
    write_proto!("store");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compile() {

    }
}