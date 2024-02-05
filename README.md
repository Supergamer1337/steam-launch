# Steam-Launch

A simple program to add extra functionality to launching Steam games, simply by editing the game's launch options. Currently, the program supports launching extra programs alongside the game as well as running [Ludusavi](https://github.com/mtkennerly/ludusavi) backups when the game exits.

Currently the following feature is still planned to be added:

- Having multiple launch profiles for each game, e.g. one for playing the game normally, one for playing the game with mods, etc. This would open a controller friendly menu when launching the game, allowing you to choose which profile to use.

## Installation

Currently, the program does not have a pre-built installer/binary, so you will have to build it yourself. This will be added in the future when the program is more feature complete.

### Supported Platforms

Currently, the program only supports (or well, is tested on at least) Windows, but I might add support for Linux if I decide to use it on my Linux machine (or switch operating system on my main/gaming computer).

## Usage

If you are using Steam, the `%command%` parameter is used to launch the game, so you will have to add `-- %command%` to the end of the command. If you are not using Steam (for some reason), you have to manually specify the command to launch the game (by replacing `%command%`, for example `path/to/game.exe`).

The program currently supports the two following commands:

### Extra

This done by using `-e` or `--extra` which takes a path (or command) to the program you want to launch alongside the game. This can be used multiple times to launch multiple programs. When the game closes, the programs will also close.

#### Example

```ps
"path/to/steam-launch.exe" -e "path/to/program.exe" -e "path/to/another/program.exe" -- %command%
```

### Save Backup

This is done by first installing and configuring [Ludusavi](https://github.com/mtkennerly/ludusavi), and then configuring the program to use it. This is done by adding a `config.toml` alongside the program with the following content:

```toml
[ludusavi]
path = "path/to/ludusavi.exe"
```

Then you add `-s` or `--save` along with the name of the game you want to save the backup of (as it is named in Ludusavi). This will run the backup for that game when the game exits.

#### Example

```ps
"path/to/steam-launch.exe" -s "game-name" -- %command%
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit)
