# simple-library-explorer
A command line tool to search the library's collection

## Configuration

the config file is at `$XDG_CONFIG_HOME/simple-library-explorer/config.toml`.

```toml
api_url = 'https://api.calil.jp/check'
api_key = 'your calil.jp api key'
systemid = 'your favourite libraries'
isbn = [
  '978-4-**-******-*',  # 13-digits isbn
  '9784*********',      # -> without hyphen
  '0123456789',         # 10-digits isbn
  #...
]
```

to find a library's `systemid`, use **http://api.calil.jp/library**

## Example output

```sh
$ simple-library-explorer
[978-4-**-******-*]
  hoge図書館: 貸出中
  fuga図書館: 貸出中
[0123456789]
  piyo図書館: 貸出可
[9999999999]

```
