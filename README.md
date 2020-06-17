## rsmpc

`mpc`, but implemented in Rust.

Note: This is not meant to be a direct implementation, there will be some differences.
For example: I moved the options of setting things like the volume to a subcommand called `set`
instead of having them as their own commands.

## commands

- `current`: Print the current song in "artist - title" format.
- `next`: Play the next song.
- `playlist`: Print all songs in the current queue, with the same format as `current`.
- `prev`: Play the previous song.
- `restart`: Restarts the currently playing song.
- `set`: Set different MPD options. Currently supported are volume, repeat, random, single and consume.
- `stats`: Display MPD stats (e.g. amount of artists/albums/etc).
- `status`: Display MPD's status (e.g. volume level, modes, state).
- `toggle`: Toggle between play/pause.
