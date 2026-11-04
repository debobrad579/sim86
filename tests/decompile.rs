use std::fs;
use std::path::Path;
use std::process::Command;

use sim86::instruction::Instruction;

fn decode(data: &[u8]) -> String {
    let mut out = String::from("bits 16\n");

    let len = data.len();
    let mut count = len;
    while count > 0 {
        let i = len - count;
        let mut bytes = [0u8; 6];

        let bytes_left = count.min(6);

        bytes[..bytes_left].copy_from_slice(&data[i..i + bytes_left]);

        let instruction = Instruction::from(bytes);

        out.push_str(&format!("{}\n", instruction));

        count -= instruction.size;
    }

    out
}

#[test]
fn decompile_to_assembly() {
    let test_dir = Path::new("test_cases");

    let test_files = fs::read_dir(test_dir).expect("Failed to read test_cases directory");

    for entry in test_files {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("asm") {
            continue;
        }

        let test_name = path.file_name().unwrap().to_string_lossy();

        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        let original_binary = temp_dir.path().join("original");
        let decoded_file = temp_dir.path().join("decoded.asm");
        let decoded_binary = temp_dir.path().join("decoded");

        let output = Command::new("nasm")
            .arg("-o")
            .arg(&original_binary)
            .arg(&path)
            .output()
            .expect("Failed to execute nasm");

        assert!(
            output.status.success(),
            "Original NASM compilation failed for {test_name}:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        let original_data: Box<[u8]> = fs::read(&original_binary)
            .expect("Failed to read original binary")
            .into();

        let decoded_asm = decode(&original_data);

        fs::write(&decoded_file, decoded_asm).expect("Failed to write decoded assembly");

        let output = Command::new("nasm")
            .arg("-o")
            .arg(&decoded_binary)
            .arg(&decoded_file)
            .output()
            .expect("Failed to execute nasm");

        assert!(
            output.status.success(),
            "Decoded NASM compilation failed for {test_name}:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        let decoded_data = fs::read(&decoded_binary)
            .expect("Failed to read decoded binary")
            .into();

        assert_eq!(
            original_data, decoded_data,
            "Decompiled assembly does not match for {test_name}"
        );
    }
}
