# ReRename

Smart file renaming with regular expressions.

## Installation

```bash
$ git clone https://gitlab.com/petervaro/rerename.git
$ cd rerename
$ cargo install --path .
```

## Usage

```bash
$ rerename --help
```

## Example

```bash
$ ls -l
03 - foo.flac
04 - bar.flac
1 - hello.flac
2 - world.flac
10 - ham.flac
22 - spam.flac

$ rerename -s '(\d+)(.*)' -t '@{index:0>2}$2' -o '1:int' -i 1 *.flac
1 - hello.flac -> 01 - hello.flac
2 - world.flac -> 02 - world.flac
03 - foo.flac -> 03 - foo.flac
04 - bar.flac -> 04 - bar.flac
10 - ham.flac -> 05 - ham.flac
22 - spam.flac -> 06 - spam.flac

Checked 6 file(s)
Renamed 4 file(s)
```

## License

Copyright &copy; 2020 Peter Varo

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see [https://www.gnu.org/licenses][license].


<!-- LINKS -->
[license]: https://www.gnu.org/licenses
