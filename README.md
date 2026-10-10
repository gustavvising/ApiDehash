# API Dehash

API Dehash is a malware analysis tool that helps identify Windows API functions hidden through API hashing. It recovers the original Windows API names from hashed references, providing valuable insight into a program’s behavior and functionality.

## API Hashing

API hashing is a technique commonly encountered during malware analysis.
Instead of storing Windows API names directly, a program can store a
hash derived from the API name and resolve it at runtime.

For example:

```text
CreateFileW
VirtualAlloc
LoadLibraryA
```

may be represented by their corresponding hash values within a binary.

API Dehash compares these hashes against a database of known Windows API
functions to recover their original names.

## Installation

Get the latest release from the [Releases](../../releases) page.

## Usage
ApiDehash `<COMMAND>` [OPTIONS]

### Commands
```text
file - Generate and save hashes.csv
hashes - Generate API hashes in memory
algorithm - Display the selected hashing algorithms and seeds
extract <file> - Extract 32-bit hash candidates from an executable
compare <file> - Compare extracted candidates against generated API hashes
scan <file> - Extract candidates and compare them against generated API hashes
help - Print this message or the help of the given subcommand(s)
version - Print version information
```

### Options
```text
--syswhispers2 <SEED>  SysWhispers2 seed (decimal or hexadecimal)
--djb2 <SEED>          DJB2 seed (decimal or hexadecimal)
```

### Examples
```bash
Generate hashes using DJB2:
ApiDehash hashes --djb2 0x12345678
```

```bash
Generate hashes using SysWhispers2:
ApiDehash hashes --syswhispers2 0x12345678
```

```bash
Generate hashes using both algorithms:
ApiDehash hashes --syswhispers2 0x12345678 --djb2 0x12345678
```

```bash
Create hashes.csv:
ApiDehash file --syswhispers2 0x12345678 --djb2 0x12345678
```

```bash
Extract candidates from an executable:
ApiDehash extract malware.exe --djb2 0x12345678
```

```bash
Compare candidates against generated API hashes:
ApiDehash compare malware.exe --syswhispers2 0x12345678
```

```bash
Scan an executable using both algorithms:
ApiDehash scan malware.exe --syswhispers2 0x12345678 --djb2 0x12345678
```

```bash
Display help:
ApiDehash --help
```

## Development

### Requirements

- Rust
- Cargo
- Python (Optional, for generating a custom api list)

### Clone the repository

```bash
git clone https://github.com/gustavvising/ApiDehash.git
cd ApiDehash
```

### Build

```bash
cargo build --release
```

Output:

```text
target/release/ApiDehash
```
