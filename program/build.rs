const FAULT_PROOF_BIN_PATH: &str = "elf/fault-proof.bin";

fn get_methods_rs_path() -> String {
    std::env::var("OUT_DIR").unwrap() + "/methods.rs"
}

fn parse_line_including(methods_rs_path: &str, key_word: &str) -> String {
    let methods_rs_content = std::fs::read_to_string(&methods_rs_path).unwrap();
    methods_rs_content
        .lines()
        .find(|line| line.contains(key_word))
        .unwrap()
        .to_string()
}

fn parse_elf_id(methods_rs_path: &str) -> String {
    let elf_line = parse_line_including(methods_rs_path, "FAULT_PROOF_ID");

    let path_start = elf_line.rfind('[').unwrap();
    let path_end = elf_line.rfind(']').unwrap() + 1;
    elf_line[path_start..path_end].to_string()
}

fn replace_elf_id() {
    let methods_rs_path = get_methods_rs_path();
    let new_id = parse_elf_id(&methods_rs_path);

    let src_methods_path = "src/lib.rs";
    let src_methods_content = std::fs::read_to_string(src_methods_path).unwrap();

    let lines: Vec<&str> = src_methods_content.lines().collect();
    let mut new_lines: Vec<String> = Vec::new();
    for line in lines {
        if line.contains("RISC0_FAULT_PROOF_ID") {
            new_lines.push(format!(
                "pub const RISC0_FAULT_PROOF_ID: [u32; 8] = {};",
                &new_id
            ));
        } else {
            new_lines.push(line.to_string());
        }
    }

    std::fs::write(src_methods_path, new_lines.join("\n")).unwrap();
}

fn parse_elf_path(methods_rs_path: &str) -> String {
    let elf_line = parse_line_including(methods_rs_path, "FAULT_PROOF_PATH");

    let path_start = elf_line.find('"').unwrap() + 1;
    let path_end = elf_line.rfind('"').unwrap();
    elf_line[path_start..path_end].to_string()
}

fn copy_elf_bin() {
    let methods_rs_path = get_methods_rs_path();
    let elf_path = parse_elf_path(&methods_rs_path);
    std::fs::copy(elf_path, FAULT_PROOF_BIN_PATH).unwrap();
}

fn main() {
    if cfg!(feature = "rebuild-guest") {
        risc0_build::embed_methods();
        copy_elf_bin();
        replace_elf_id();
    }

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=fault-proof/src");
}
