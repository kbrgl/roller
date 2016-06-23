# Description
Roller is a simple yet incredibly robust text truncation utility written in Rust.

Its primary use case lies with window managers, GUIs and TUIs where screen real estate is limited and it is useful to truncate text to fully utilise available space.

Roller has been built with sane defaults and it is incredibly easy to get started. An example:

Here's a simple example
```sh
# xtitle is a command line utility that gets the title of the currently focused window from your desktop environment or window manager. The -s flag causes it to stream the window titles - whenever the active window changes, it outputs the new title
# lemonbar is a simple program for rendering a status bar / panel on your display - it is commonly used with window managers like i3 and bspwm
xtitle -s | roller | lemonbar
```

# Installation
Roller requires Rust and its package manager, Cargo. Once you have these installed:

```sh
git clone https://github.com/kbrgl/roller.git
cd roller
cargo build --release
sudo ln -s ./target/release/roller /usr/bin/roller
```

## Installing Rust and Cargo
### Arch
```sh
sudo pacman -S rust cargo
```

### Ubuntu and Debian
```sh
sudo apt install rust cargo
```

# Usage
```
Roller 1.0.0
Kabir Goel <kabirgoel.kg@gmail.com>
Truncate text by rolling it like a news ticker.

USAGE:
    roller [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -m, --mutate     Roll in place instead of on separate lines
    -r, --reverse    Roll in reverse
    -V, --version    Prints version information

OPTIONS:
    -c, --count <NUMBER>           Only roll this many times
    -i, --interval <INTERVAL>      Set a custom interval in milliseconds
    -a, --postfix <POSTFIX>        Append a static postfix to the text
    -b, --prefix <PREFIX>          Prepend a static prefix to the text
    -s, --separator <SEPARATOR>    Place a separator between consecutive rolls
    -t, --truncate <LENGTH>        Only roll if text is longer than LENGTH, effectively truncating it
```

# Examples
## Count
The `-c 5` option specifies that 5 permutations should be made. In its absence, an infinite number of permutations would be made so the text would be scrolling indefinitely.
```sh
$ echo 'Gumby' | roller -c 5
umby G
mby Gu
by Gum
y Gumb
 Gumby
```

## Fresh input on stdin
By default, Roller will refresh if there is any new input on stdin.
```sh
$ xtitle -s | roller
~/Code/scroller | nvim 
/Code/scroller | nvim $
Code/scroller | nvim $/
ode/scroller | nvim $/C
# now, I switch to another window. this causes xtitle to output a new line of text. scroller picks up this new line of text.
~ | fish 
 | fish ~
| fish ~ 
```

## Truncate
The `-t 5` option means "if the text is less than 5 characters long then do not scroll it". This option is immensely useful if you want to make sure the text does not exceed a certain length.
```sh
$ echo 'John' | scroller -n -c 3 -l 5
John
John
John
```


## Separator
The `-s ' -- '` option means "after the entire string has been displayed, separate the end of the string and the start of string with ' -- '.
```sh
$ echo 'John' | scroller -n -c 3 -s ' -- '
ohn -- J
hn -- Jo
n -- Joh
```
By default, the separator is a single space. If the default separator was an empty string (''), the output would look like the following:
```
ohnJ
hnJo
nJoh
```

## Prefix and postfix
The `-a ' -- ' -b ' .. '` specify a prefix and postfix for the text
```sh
$ echo 'hello, world' | roller -a ' -- ' -b ' .. '
 .. hello, world  -- 
 .. ello, world h -- 
 .. llo, world he -- 
```

# License
Roller is open source software licensed under the terms of the MIT license.
