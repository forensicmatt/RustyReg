**This Project was for learning purposes and is not maintained.**

# RustyReg
Registry to JSON

```
RusyReg 0.1.3
Matthew Seyer <https://github.com/forensicmatt/RustyReg>
Registry Parser written in Rust.

USAGE:
    RustyReg.exe [FLAGS] [OPTIONS] --source <PATH>

FLAGS:
    -b, --bool_expr    JMES Query as bool only. (Prints whole record if true.)
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -q, --query <QUERY>    JMES Query
    -s, --source <PATH>    Source.
```

## Data Decoding
The following list shows how data types are being decoded: [Decoded Value Data](https://github.com/forensicmatt/r-winreg/blob/master/README.md#decoded-value-data).

## Example Output
Output is in the JSONL format. Here is a record representation:

```json
{
  "fullpath": "\\CsiTool-CreateHive-{00000000-0000-0000-0000-000000000000}\\7-Zip\\Path",
  "nk_last_written": "2013-10-18 00:28:34.576",
  "valuekey": {
    "data_size": 48,
    "data_type": "REG_SZ",
    "flags": "VK_VALUE_COMP_NAME",
    "value_name": "Path",
    "data": "C:\\Program Files\\7-Zip\\"
  },
  "security": {
    "owner_sid": "S-1-5-18",
    "group_sid": "S-1-5-18",
    "dacl": {
      "revision": 2,
      "count": 10,
      "entries": [
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "(empty)",
          "data": {
            "access_rights": 131097,
            "sid": "S-1-5-32-545"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "CONTAINER_INHERIT_ACE | INHERIT_ONLY_ACE",
          "data": {
            "access_rights": 2147483648,
            "sid": "S-1-5-32-545"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "(empty)",
          "data": {
            "access_rights": 983103,
            "sid": "S-1-5-32-544"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "CONTAINER_INHERIT_ACE | INHERIT_ONLY_ACE",
          "data": {
            "access_rights": 268435456,
            "sid": "S-1-5-32-544"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "(empty)",
          "data": {
            "access_rights": 983103,
            "sid": "S-1-5-18"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "CONTAINER_INHERIT_ACE | INHERIT_ONLY_ACE",
          "data": {
            "access_rights": 268435456,
            "sid": "S-1-5-18"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "(empty)",
          "data": {
            "access_rights": 983103,
            "sid": "S-1-5-18"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "CONTAINER_INHERIT_ACE | INHERIT_ONLY_ACE",
          "data": {
            "access_rights": 268435456,
            "sid": "S-1-3-0"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "(empty)",
          "data": {
            "access_rights": 131097,
            "sid": "S-1-15-2-1"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "CONTAINER_INHERIT_ACE | INHERIT_ONLY_ACE",
          "data": {
            "access_rights": 2147483648,
            "sid": "S-1-15-2-1"
          }
        }
      ]
    }
  }
}
```

## Change Log
#### rwinreg 0.1.3 (2017-10-22)
- Updated the r-winreg library for better performance.

#### rwinreg 0.1.2 (2017-10-04)
- Updated r-winreg for more Cell support

#### RustyReg v0.1.1 (2017-09-28)
- Updated r-winreg for additional data types.
