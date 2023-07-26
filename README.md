# n2soc

Nioh 2 save data ownership converter

## Usage

```shell
Usage: n2soc --user-data <USER_DATA> --progress-data <PROGRESS_DATA>

Options:
  -u, --user-data <USER_DATA>          Path of save data of your user
  -p, --progress-data <PROGRESS_DATA>  Path of save data of game progress
  -h, --help                           Print help
  -V, --version                        Print version

```

## Example

Asserting `my/SAVEDATA.BIN` is a save data of your own(which contains your ID info), and `download/SAVEDATA.BIN` is an all-cleared save data you downloaded from community.

Surely you want to import this all-cleared data as your own to play build tests.

`n2soc -u my/SAVEDATA.BIN -p download/SAVEDATA.BIN`

On Windows you may execute:

`n2soc.exe -u my\SAVEDATA.BIN -p download\SAVEDATA.bin`

Then you will find imported data under `OUTPUT` directory.
