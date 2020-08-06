## rsmpc

`mpc`, but implemented in Rust.

Note: This is not meant to be a direct implementation, there will be some differences.

For example: I moved the options of setting things like the volume to a subcommand called `set`<br>
instead of having them as their own commands.

## commands

- `current`: Print the current song in "artist - title" format.
- `next`: Play the next song.
- `playlist`: Print all songs in the current queue, with the same format as `current`.
- `prev`: Play the previous song.
- `restart`: Restarts the currently playing song.
- `set`: Set different MPD options. Currently supported are volume, repeat, random, single and consume.
- `stats`: Display MPD stats.

Example output:

```
Songs:       12449
Albums:      1056
Artists:     144
Uptime:      7d 11h 45m
DB Playtime: 36d 18h 7m
```

- `status`: Display MPD's status.

Example output:

```
System Of A Down - Question!
[Playing] 03:31/03:20
Volume: 100%  Repeat: off  Random: off  Single: off  Consume: off
```

- `toggle`: Toggle between play/pause.
