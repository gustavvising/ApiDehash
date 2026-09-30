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

```text
ApiDehash <file>
```

For example:

```bash
ApiDehash malware.exe
```

## Development

### Requirements

- Rust
- Cargo
- Python 3.x

### Clone the repository

```bash
git clone https://github.com/gustavvising/ApiDehash.git
cd ApiDehash
```

### Build

```bash
cargo build --release
```

Output to:

```text
target/release/ApiDehash
```