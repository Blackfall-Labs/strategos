# engram-cli

A CLI tool for managing Engram archives - create, list, and inspect `.eng` archive files.

## Features

- **Pack**: Create Engram archives from files or directories
- **List**: Display contents of Engram archives
- **Info**: Show metadata and statistics about archives
- **Search**: Search for text patterns within files

## Commands

### Pack Command

Create an Engram archive from files or directories:

```bash
# Pack a directory (creates test_data.eng)
engram pack test_data

# Pack with custom output path
engram pack test_data -o custom.eng

# Pack a single file
engram pack hello.txt
```

### List Command

List all files in an Engram archive:

```bash
engram list archive.eng
# or use alias
engram ls archive.eng
```

### Info Command

Display archive metadata and statistics:

```bash
# Basic info
engram info archive.eng

# Detailed inspection with per-file details
engram info archive.eng --inspect
```

### Search Command

Search for text patterns in files:

```bash
engram search "pattern" file.txt
```

---

## Testing

### Build the CLI

```bash
cargo build --release
```

### Test Pack Command

Create test data and pack it:

```bash
# Create test data
mkdir -p test_data
echo "Hello, World!" > test_data/hello.txt
echo "This is a test file." > test_data/test.txt
mkdir -p test_data/subdir
echo "Nested file content" > test_data/subdir/nested.txt

# Pack the directory
target/release/engram.exe pack test_data
```

**Output:**
```text
Packing: test_data
Output: test_data.eng
  Added: hello.txt
  Added: subdir/nested.txt
  Added: test.txt
Packed 3 files
Archive created successfully: test_data.eng
```

### Test List Command

List archive contents:

```bash
target/release/engram.exe list test_data.eng
```

**Output:**
```text
hello.txt
subdir/nested.txt
test.txt
```

### Test Info Command

Show basic archive information:

```bash
target/release/engram.exe info test_data.eng
```

**Output:**
```text
Archive: test_data.eng
Format Version: 1.0
Total Files: 3
Content Version: 0
Total Size: 55 bytes
Compressed: 55 bytes (100.0%)
```

Show detailed inspection:

```bash
target/release/engram.exe info test_data.eng --inspect
```

**Output:**
```text
Archive: test_data.eng
Format Version: 1.0
Total Files: 3
Content Version: 0
Total Size: 55 bytes
Compressed: 55 bytes (100.0%)

------------------------------------------------------------
Detailed File Information:
------------------------------------------------------------

hello.txt
  Compression: None
  Size: 14 -> 14 bytes (100.0%)
  CRC32: B4E89E84
  Offset: 64
  Modified: 1762463474

subdir/nested.txt
  Compression: None
  Size: 20 -> 20 bytes (100.0%)
  CRC32: 08A2D2D1
  Offset: 78
  Modified: 1762463474

test.txt
  Compression: None
  Size: 21 -> 21 bytes (100.0%)
  CRC32: DA83B85E
  Offset: 98
  Modified: 1762463474

------------------------------------------------------------
Archive Structure:
------------------------------------------------------------
Central Directory Offset: 119
Central Directory Size: 960
Header CRC: 00000000
```

### Test Single File Packing

```bash
target/release/engram.exe pack test_data/hello.txt -o single_file.eng
target/release/engram.exe list single_file.eng
```

**Output:**
```text
Packing: test_data/hello.txt
Output: single_file.eng
  Added: hello.txt
Archive created successfully: single_file.eng
hello.txt
```

### Run Tests

```bash
cargo test
```

All tests pass successfully.