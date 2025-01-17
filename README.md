<div align="center">
  <h1>️📰 Packard</h1>
  <img alt="GitHub Release" src="https://img.shields.io/github/v/release/troylusty/packard">
  <h5>Packard is a simple RSS aggregator meant to allow you to take a quick glance at what's occurring in topics you care about.</h5>
</div>

![Demo](https://github.com/user-attachments/assets/439f34ef-5c33-4bd1-bce1-1f0fa549794f)

This is my first attempt at making something with Rust so that I may learn alongside creating something that I personally find useful.

## Installation

On NixOS you can install Packard by including it as an input in flake.nix, then adding it to your system packages.

```nix
inputs = {
    packard.url = "github:troylusty/packard";
};
```

```nix
environment.systemPackages = {
    inputs.packard.packages."${pkgs.system}".default
};
```

Alternatively, [the latest release](https://github.com/troylusty/packard/releases/latest) binary is available.

## Configuration

There are now several options available to be configured, with more on their way. As of now Packard's config file should be placed within `$HOME/.config/packard` and formatted in TOML.

1. `count`

This can be adjusted either by using the `-c` flag when running the program, or by setting it in the config. Note that if `count` is set within the config file and then the program is run with the `-c` flag, this flag will overwrite the config file. Unlike other options, there is a default value for `count` which is set to 8.

2. `skip_amount`

If you wish to view older articles without having to scroll through newer ones to get there, use `-s` or the `skip_amount` option to do that. This will skip the amount specified whilst still returning your specified `count` amount of articles. By default this option is set to 0.

3. `selected_list`

Using the configured lists mentioned above, this option selects which of those you want to see. This can be set using the `selected_list` option in the config or by running the program using the `-l` flag. Again, using the flag will overwrite the option if it is specified in the config.

4. `[lists]`

Lists can be configured in Packard so that they can be easily swapped out depending on what you want to see. Please see the example config for reference.

### Example config.toml

```toml
count = 8
#skip_amount = 4
selected_list = "personal"

[lists]
personal = [
    "https://troylusty.com/rss.xml",
    "https://mitchellh.com/feed.xml"
]
news = [
  "https://www.gamingonlinux.com/article_rss.php?newsonly",
  "https://store.steampowered.com/feeds/news.xml",
]
```

## Interaction

After running Packard with your configured settings, the parsed results can be opened in your default browser however your terminal allows for opening URLs. For example the keybind for this with [Foot](https://codeberg.org/dnkl/foot#urls) is `ctrl` + `shift` + `o`.

Currently no keyboard interaction is implemented. To get around this you can pipe the output of Packard into a tool like `less` like so: `packard -c 12 -l news -s 3 | less`.
