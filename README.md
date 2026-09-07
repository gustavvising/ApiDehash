# ApiDehash

ApiDehash helps in malware analysis by defeating the technique called api hashing. Api hashing is a malicious technique used to obfuscate the names of the imported functions from the Windows api. ApiDehash recovers the original Windows API names to get insight into what the program is doing.

Build with `cargo build --release --locked`, then run:

```sh
./target/release/ApiDehash sample.exe
./target/release/ApiDehash sample.exe --djb2-seed 0x7d895397 --sw2-seed 0x12345678
```

The API list is embedded in the binary, so it can run from any working directory without `apis.txt` or a writable cache. It scans code and initialized data sections by their attributes, including renamed sections.

DJB2 uses the standard seed `5381` by default. `--djb2-seed` overrides it; SysWhispers2 matching is enabled only when `--sw2-seed` is supplied. Seeds are unsigned 32-bit integers in decimal or `0x` hexadecimal. Seed inference is not supported: use the sample's seed. The supported variants are additive 32-bit DJB2 and the SysWhispers2 ROR8 algorithm.

Invalid arguments and scan failures return a nonzero exit status. A successful scan with no matches returns zero. Run `cargo fmt --check` and `cargo test --locked` to validate changes.
