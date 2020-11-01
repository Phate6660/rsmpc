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
- `shuffle`: Shuffle the current queue.
- `stats`: Display MPD stats.

Example output:

```
Songs:       12625
Albums:      1077
Artists:     164
Uptime:      5d 4h 17m
DB Playtime: 37d 9h 35m
```

Comparison to `mpc stats`:

```
Artists:    164
Albums:    1077
Songs:    12625

Play Time:    0 days, 11:41:35
Uptime:       5 days, 4:18:16
DB Updated:   Wed Oct 28 22:30:39 2020
DB Play Time: 37 days, 9:35:22
```

- `status`: Display MPD's status.

Example output:

```
After the Minor - Structure Fire
[Playing]   #5/32   04:28/05:06
Volume: 100%   Repeat: on   Random: off   Single: off   Consume: off
```

Comparison to `mpc status`:

```
After the Minor - Structure Fire
[playing] #5/32   4:37/5:07 (90%)
volume:100%   repeat: on    random: off   single: off   consume: off
```

- `toggle`: Toggle between play/pause.
