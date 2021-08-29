# jluniq

Remove duplicate key member lines from sorted [NDJSON](http://ndjson.org/).

**Important: NDJSON file must be sorted by the value of the uniq key member. (e.g. [jlsort](https://github.com/winebarrel/jlsort))**

[![Build Status](https://github.com/winebarrel/jluniq/workflows/CI/badge.svg)](https://github.com/winebarrel/jluniq/actions)

## Usage

```
Usage: jluniq [OPTIONS] [FILE]

Options:
    -k, --key KEY       JSON key to make it unique
    -g, --group         Group rows with the same value
    -v, --version       Print version and exit
    -h, --help          Print usage and exit
```

```
% cat data.ndjson
{"id": 1, "sub_id": 11, "val":"abc"}
{"id": 2, "sub_id": 21, "val":"def"}
{"id": 2, "sub_id": 22, "val":"ghi"}
{"id": 3, "sub_id": 31, "val":"jkl"}
{"id": 4, "sub_id": 41, "val":"mno"}
{"id": 5, "sub_id": 51, "val":"pqr"}
{"id": 5, "sub_id": 52, "val":"slu"}
{"id": 6, "sub_id": 61, "val":"vwz"}

% jluniq -k id data.ndjson
{"id":1,"sub_id":11,"val":"abc"}
{"id":2,"sub_id":21,"val":"def"}
{"id":3,"sub_id":31,"val":"jkl"}
{"id":4,"sub_id":41,"val":"mno"}
{"id":5,"sub_id":51,"val":"pqr"}
{"id":6,"sub_id":61,"val":"vwz"}

% cat data.ndjson | jluniq -k id
{"id":1,"sub_id":11,"val":"abc"}
{"id":2,"sub_id":21,"val":"def"}
{"id":3,"sub_id":31,"val":"jkl"}
{"id":4,"sub_id":41,"val":"mno"}
{"id":5,"sub_id":51,"val":"pqr"}
{"id":6,"sub_id":61,"val":"vwz"}

% jluniq -k id -g data.ndjson
[1,{"id":1,"sub_id":11,"val":"abc"}]
[2,{"id":2,"sub_id":21,"val":"def"},{"id":2,"sub_id":22,"val":"ghi"}]
[3,{"id":3,"sub_id":31,"val":"jkl"}]
[4,{"id":4,"sub_id":41,"val":"mno"}]
[5,{"id":5,"sub_id":51,"val":"pqr"},{"id":5,"sub_id":52,"val":"slu"}]
[6,{"id":6,"sub_id":61,"val":"vwz"}]
```

## Related Links

* https://github.com/winebarrel/jlsort
* https://github.com/winebarrel/jljoin
