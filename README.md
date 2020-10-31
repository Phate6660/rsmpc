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
The Ocean - Jurassic | Cretaceous
[Paused] #2/32 01:27/13:24
Volume: 100%  Repeat: off  Random: off  Single: off  Consume: off
```

Comparison to `mpc status`:

  ```
  The Ocean - Jurassic | Cretaceous
  [paused]  #2/32   1:28/13:25 (10%)
  volume:100%   repeat: off   random: off   single: off   consume: off
  ```

- `toggle`: Toggle between play/pause.
