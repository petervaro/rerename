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
03-foo.flac
04-bar_baz.flac
1-hello_there.flac
2-world.flac
10-ham.flac
22-spam_and_leek.flac

$ rerename -s '^(\d+)-(.*)$' -t '@{index:0>2} - $2' -o '1:int' -i 1 *.flac
1-hello_there.flac -> 01 - hello_there.flac
2-world.flac -> 02 - world.flac
03-foo.flac -> 03 - foo.flac
04-bar_baz.flac -> 04 - bar_baz.flac
10-ham.flac -> 05 - ham.flac
22-spam_and_leek.flac -> 06 - spam_and_leek.flac

Checked 6 file(s)
Renamed 6 file(s)

$ ls -l
01 - hello_there.flac
02 - world.flac
03 - foo.flac
04 - bar_baz.flac
05 - ham.flac
06 - spam_and_leek.flac

$ rerename -s '_' -t ' ' *.flac
01 - hello_there.flac -> 01 - hello there.flac
02 - world.flac -> 02 - world.flac
03 - foo.flac -> 03 - foo.flac
04 - bar_baz.flac -> 04 - bar baz.flac
05 - ham.flac -> 05 - ham.flac
06 - spam_and_leek.flac -> 06 - spam and leek.flac

Checked 6 file(s)
Renamed 3 file(s)

$ ls -l
01 - hello there.flac
02 - world.flac
03 - foo.flac
04 - bar baz.flac
05 - ham.flac
06 - spam and leek.flac

$ rerename -s '.*' -t '$0' -T title *.flac
01 - hello there.flac -> 01 - Hello There.Flac
02 - world.flac -> 02 - World.Flac
03 - foo.flac -> 03 - Foo.Flac
04 - bar baz.flac -> 04 - Bar Baz.Flac
05 - ham.flac -> 05 - Ham.Flac
06 - spam and leek.flac -> 06 - Spam And Leek.Flac

Checked 6 file(s)
Renamed 6 file(s)

$ ls -l
01 - Hello There.Flac
02 - World.Flac
03 - Foo.Flac
04 - Bar Baz.Flac
05 - Ham.Flac
06 - Spam And Leek.Flac

$ rerename -s '(.*?)\.Flac' -t '$1.flac' *.Flac
01 - Hello There.Flac -> 01 - Hello There.flac
02 - World.Flac -> 02 - World.flac
03 - Foo.Flac -> 03 - Foo.flac
04 - Bar Baz.Flac -> 04 - Bar Baz.flac
05 - Ham.Flac -> 05 - Ham.flac
06 - Spam And Leek.Flac -> 06 - Spam And Leek.flac

Checked 6 file(s)
Renamed 6 file(s)

$ ls -l
01 - Hello There.flac
02 - World.flac
03 - Foo.flac
04 - Bar Baz.flac
05 - Ham.flac
06 - Spam And Leek.flac
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
