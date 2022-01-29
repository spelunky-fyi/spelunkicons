use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

static SHEETS: &'static [(&str, &str)] = &[
    ("FLOOR_CAVE", "floor_cave.png"),
    ("FLOOR_JUNGLE", "floor_jungle.png"),
];

fn main() {
    let textures_path = fs::canonicalize(PathBuf::from("target/Textures")).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sheets.rs");

    let mut out_file = File::create(&dest_path).unwrap();

    for (name, path) in SHEETS {
        let path = textures_path.join(path);
        let metadata = fs::metadata(&path).unwrap();

        out_file
            .write(
                format!(
                    "pub static {}: &'static [u8; {}] = include_bytes!({:?});\n",
                    name,
                    metadata.len(),
                    &path
                )
                .as_bytes(),
            )
            .unwrap();
    }
    println!("cargo:rerun-if-changed=build.rs");
}
