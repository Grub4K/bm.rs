# bm

***b***ook***m***ark paths and quickly change into bookmarked directories

## Info
Use `bm help` to list all subcommands and options, or `bm help <subcommand>` for more information on the given subcommand.

To be able to change directory in your shell, we have to run shell code.
This is done by `eval`ing an aliased function that performs the `cd` for us.
You can generate the code to eval using `bm init <shell>` where `shell`
is one of `bash`, `posix` or `powershell`.
Instead of `bm` you can provide a custom alias via `--alias <other alias>`.

Use `bm cd <name>` to change to the bookmarked path for that name.\
Note that `bm cd` will only print out the path it would use
and only after the initial `eval` alias will it perform the `cd` operation.
Instead of a full name, a glob pattern is also supported.
If more than one bookmark matches that pattern, an error is thrown.

To add a new bookmark, use `bm set <name> [path]`.\
If `path` is omitted, the current path is added as a bookmark instead.
If the bookmark already exists an error will be thrown.
To force overwriting existing bookmarks, use `--force`/`-f`.

Bookmarks can be deleted using `bm del <name>`.

You can also directly edit the config file. Use `bm conf` to print out the path
to the config file. You can also append `--edit` to open the default editor
(as per `$EDITOR`, defaulting to `vi`) with the config file.
The default bookmark file location is at `~/.config/bm/default.rc`.

Use `bm ls [pattern]` to list available config items.\
If `pattern` is omitted, all items are output.
`pattern` can again be a glob, in which case it will print all matching items.
