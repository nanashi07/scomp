# scomp

## Configuration file format

```json
[
  {
    "host": "localhost",
    "port": 3306,
    "schema": "main",
    "user": "root",
    "password": "password",
    "country": "ke",
    "environment": "uat"
  },
  // more config
]
```

## Arguments

```
scomp - Schema comparison for MySQL 0.2.0
Bruce Tsai

USAGE:
    scomp [OPTIONS] --config <config>

OPTIONS:
    -c, --config <config>           MySQL connection config file
        --column <diff-columns>     Output file of columns comparison [default: diff-columns.csv]
    -h, --help                      Print help information
        --indices <diff-indices>    Output file of indices comparison [default: diff-indices.csv]
        --level <level>             Log level [default: info]
        --source                    Output source data
    -V, --version                   Print version information
```