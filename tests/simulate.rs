use std::{fs, process::Command};

use sim86::sim::Sim86;

fn run_sim86(file: &str) -> Sim86 {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let compiled_path = temp_dir.path().join("compiled");

    let output = Command::new("nasm")
        .arg("-o")
        .arg(&compiled_path)
        .arg(file)
        .output()
        .expect("Failed to execute nasm");

    assert!(
        output.status.success(),
        "Original NASM compilation failed for {file}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let program: Box<[u8]> = fs::read(&compiled_path)
        .expect("Failed to read original binary")
        .into();

    let mut sim86 = Sim86::default();
    sim86.load(&program);
    sim86.run(false);

    sim86
}

#[test]
fn immediate_movs() {
    let sim86 = run_sim86("tests/sim/immediate_movs.asm");
    assert_eq!(sim86.registers(), [1, 3, 4, 2, 5, 6, 7, 8]);
}

#[test]
fn register_movs() {
    let sim86 = run_sim86("tests/sim/register_movs.asm");
    assert_eq!(sim86.registers(), [4, 2, 1, 3, 1, 2, 3, 4]);
}

#[test]
fn challenge_movs() {
    let sim86 = run_sim86("tests/sim/challenge_movs.asm");
    assert_eq!(
        sim86.registers(),
        [
            0x4411, 0x6677, 0x7788, 0x3344, 0x4411, 0x3344, 0x6677, 0x7788
        ]
    );
    assert_eq!(sim86.segment_registers(), [0x6677, 0, 0x4411, 0x3344]);
}

#[test]
fn flags() {
    let sim86 = run_sim86("tests/sim/flags.asm");
    assert_eq!(sim86.registers(), [0, 0x0F01, 0, 0xE102, 0x03E6, 0, 0, 0]);
    assert_eq!(sim86.flags(), 0b0000_0000_0100_0100);
}

#[test]
fn challenge_flags() {
    let sim86 = run_sim86("tests/sim/challenge_flags.asm");
    assert_eq!(
        sim86.registers(),
        [0, 0, 0x000A, 0x9CA5, 0x0063, 0x0062, 0, 0]
    );
    assert_eq!(sim86.flags(), 0b0000_0000_1001_0101);
}

#[test]
fn conditional_jumps() {
    let sim86 = run_sim86("tests/sim/conditional_jumps.asm");
    assert_eq!(sim86.registers(), [0, 0, 0, 0x0406, 0, 0, 0, 0]);
    assert_eq!(sim86.ip(), 0x000F);
}

#[test]
fn challenge_jumps() {
    let sim86 = run_sim86("tests/sim/challenge_jumps.asm");
    assert_eq!(sim86.registers(), [0x000D, 0, 0, 0xFFFB, 0, 0, 0, 0]);
    assert_eq!(sim86.ip(), 0x001D);
}

#[test]
fn memory_mov() {
    let sim86 = run_sim86("tests/sim/memory_mov.asm");
    assert_eq!(
        sim86.registers(),
        [0, 0x0002, 0x000A, 0x0001, 0, 0x0004, 0, 0]
    );
    assert_eq!(sim86.ip(), 0x0031);
}

#[test]
fn add_loop() {
    let sim86 = run_sim86("tests/sim/add_loop.asm");
    assert_eq!(
        sim86.registers(),
        [0, 0x0004, 0x0006, 0x0006, 0, 0x03E8, 0x0006, 0]
    );
    assert_eq!(sim86.ip(), 0x0024);
    assert_eq!(sim86.flags(), 0b0000_0000_0100_0100);
}

#[test]
fn add_loop_challenge() {
    let sim86 = run_sim86("tests/sim/add_loop_challenge.asm");
    assert_eq!(sim86.registers(), [0, 0, 0x0006, 0x0006, 0, 0x03E6, 0, 0]);
    assert_eq!(sim86.ip(), 0x0022);
    assert_eq!(sim86.flags(), 0b0000_0000_0100_0100);
}

#[test]
fn stack() {
    let sim86 = run_sim86("tests/sim/stack.asm");
    assert_eq!(sim86.registers(), [1, 0, 0, 2, 0x9C40, 0, 0, 0]);
    assert_eq!(sim86.flags(), 0b0000_0000_1001_0001);
}

#[test]
fn test() {
    let sim86 = run_sim86("tests/sim/test.asm");
    assert_eq!(sim86.registers(), [10, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(sim86.flags(), 0b0000_0000_0100_0100);
}

#[test]
fn xchg() {
    let sim86 = run_sim86("tests/sim/xchg.asm");
    assert_eq!(sim86.registers(), [40, 20, 10, 30, 0, 0, 0, 0]);
}
