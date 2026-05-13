## Components

- emacs/ — Doom Emacs helpers
- tools/ — Rust CLI tools integrated in emacs

## Build

``` bash
make build
make install
```

## Emacs
Add one line to your config:

```elisp
(load "/absolute/path/to/Doomed_ida/emacs/doomed-ida-init.el")
```

Then use:

M-x doomed-ida-hex-dec \
M-x doomed-ida-num-char \
M-x doomed-ida-arch-file
