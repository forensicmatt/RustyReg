# RustyReg
Registry to JSON

```
RusyReg 0.1.1
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
The following list shows how data types are being decoded: [Decoded Value Data](https://github.com/forensicmatt/r-winreg/blob/master/README.md#decoded-value-data). Currently only REG_SZ is being decoded.

## Example Output
Output is in the JSONL format. Here is a record representation:

```json
{
  "fullpath": "CsiTool-CreateHive-{00000000-0000-0000-0000-000000000000}/Software/Microsoft/Office/15.0/Wef/Providers/3ZB89tq5xmYoOQ8NXfCuXQ==/UniqueId",
  "security": {
    "owner_sid": "S-1-5-21-718126207-1171771683-1750804747-1001",
    "group_sid": "S-1-5-21-718126207-1171771683-1750804747-1001",
    "dacl": {
      "revision": 2,
      "count": 5,
      "entries": [
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE",
          "data": {
            "access_rights": 131097,
            "sid": "S-1-15-3-2929230137-1657469040"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE",
          "data": {
            "access_rights": 983103,
            "sid": "S-1-5-21-718126207-1171771683-1750804747-1001"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE",
          "data": {
            "access_rights": 983103,
            "sid": "S-1-5-18"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE",
          "data": {
            "access_rights": 983103,
            "sid": "S-1-5-32-544"
          }
        },
        {
          "ace_type": "ACCESS_ALLOWED",
          "ace_flags": "OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE",
          "data": {
            "access_rights": 131097,
            "sid": "S-1-5-12"
          }
        }
      ]
    }
  },
  "value": {
    "data_size": 66,
    "data_type": "REG_SZ",
    "flags": "VK_VALUE_COMP_NAME",
    "value_name": "UniqueId",
    "data": "500EEDB6E7C74345A6A7F87197C2EC44"
  }
}
```

## Change Log
#### RustyReg v0.1.1 (2017-09-28)
- Updated r-winreg for additional data types.
