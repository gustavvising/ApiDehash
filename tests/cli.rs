use std::{fs, process::Command};

// An inert PE32+ image: one renamed code section containing three known hashes.
fn fixture() -> Vec<u8> {
    let mut bytes = vec![0; 1024];
    bytes[..2].copy_from_slice(b"MZ");
    bytes[0x188..0x190].copy_from_slice(b".custom\0");
    for (offset, value) in [
        (0x3c, 0x80u32),      // PE header offset
        (0x80, 0x4550),       // PE signature
        (0x84, 0x0001_8664),  // AMD64, one section
        (0x94, 0x0022_00f0),  // optional header size and executable flags
        (0x98, 0x20b),        // PE32+
        (0x9c, 0x200),        // code size
        (0xac, 0x1000),       // code RVA
        (0xb0, 0x4000_0000),  // image base (low word)
        (0xb4, 1),            // image base (high word)
        (0xb8, 0x1000),       // section alignment
        (0xbc, 0x200),        // file alignment
        (0xd0, 0x2000),       // image size
        (0xd4, 0x200),        // headers size
        (0xdc, 3),            // console subsystem
        (0x104, 16),          // data directory count
        (0x190, 0x200),       // section virtual size
        (0x194, 0x1000),      // section RVA
        (0x198, 0x200),       // section raw size
        (0x19c, 0x200),       // section raw offset
        (0x1ac, 0x6000_0020), // readable, executable code
        (0x200, 0x382c_0f97), // VirtualAlloc, standard DJB2
        (0x204, 0x93b5_b929), // VirtualAlloc, DJB2 seed 0x7d895397
        (0x208, 0x0d9f_0319), // ZwAllocateVirtualMemory, SW2 seed 0x12345678
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes
}

#[test]
fn cli_scans_renamed_sections_with_selected_seeds_and_reports_failures() {
    let root = std::env::temp_dir().join(format!("apidehash-cli-test-{}", std::process::id()));
    let cwd = root.join("cwd");
    fs::create_dir_all(&cwd).unwrap();
    let target = root.join("sample.exe");
    fs::write(&target, fixture()).unwrap();

    let scan = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_ApiDehash"))
            .arg(&target)
            .args(args)
            .current_dir(&cwd)
            .output()
            .unwrap()
    };
    let output = scan(&[]);
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("VirtualAlloc") && stdout.contains("0x382C0F97"));

    let output = scan(&["--djb2-seed", "0x7d895397", "--sw2-seed", "305419896"]);
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("0x93B5B929"));
    assert!(stdout.contains("ZwAllocateVirtualMemory") && stdout.contains("0x0D9F0319"));

    fs::write(root.join("invalid.exe"), b"not a PE file").unwrap();
    for name in ["missing.exe", "invalid.exe"] {
        let output = Command::new(env!("CARGO_BIN_EXE_ApiDehash"))
            .arg(root.join(name))
            .current_dir(&cwd)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
    assert_eq!(fs::read_dir(&cwd).unwrap().count(), 0);
    fs::remove_dir_all(root).unwrap();
}
