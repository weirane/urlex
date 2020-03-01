# urlex
Extract URL from stdin and print them to stdout. It can be used with `dmenu` to
mimic [`urlview`][urlview]:

    urlex | dmenu | xargs -r xdg-open


[urlview]: https://github.com/sigpipe/urlview

## Install
Use cargo:

    cargo install urlex

For Arch users, `urlex` can be installed from the AUR:

    yay -S urlex-git
