# Steam-Launch

A simple program to add extra functionality to launching Steam games, simply by editing the game's launch options. Currently, the program only supports launching extra programs alongside the game, but more features will be added in the future (mostly according to my own needs).

Some of the features I plan to add are:

- Having multiple launch profiles for each game, e.g. one for playing the game normally, one for playing the game with mods, etc. This would open a controller friendly menu when launching the game, allowing you to choose which profile to use.
- Save game backups for non-steam/non-cloud games using [Ludusavi](https://github.com/mtkennerly/ludusavi). Making sure you can always backup your save games.

## Installation

Currently, the program does not have a pre-built installer/binary, so you will have to build it yourself. This will be added in the future when the program is more feature complete.

### Supported Platforms

Currently, the program only supports (or well, is tested on at least) Windows, but I might add support for Linux if I decide to use it on my Linux machine (or switch operating system on my main/gaming computer).

## Usage

The program currently only supports one command, `-e` or `--extra` which takes a path (or command) to the program you want to launch alongside the game. This can be used multiple times to launch multiple programs alongside the game. When the game closes, the programs will also close.

### Example

```ps
"path/to/steam-launch.exe" -e "path/to/program.exe" -e "path/to/another/program.exe" -- %command%
```

Where %command% is a special property in the Steam launch options that will be replaced by whatever steam would normally launch. This is usually the path to the game's executable.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit)
