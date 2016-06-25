# Description
Roller is a simple yet robust text truncation utility written in Rust.

It's really easy to use. Here's an example:

```sh
xtitle -s | roller | lemonbar
```

-   xtitle outputs the current window title
-   lemonbar generates a status bar on the top of the screen


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
```sh
$ echo 'Rust' | roller -c 5
Rust 
ust R
st Ru
t Rus
 Rust
```

## Fresh input on stdin
By default, Roller will refresh if there is any new input on stdin.

```sh
$ xtitle -s | roller
~/Code/roller | nvim 
/Code/roller | nvim $
Code/roller | nvim $/
ode/roller | nvim $/C
~ | fish 
 | fish ~
| fish ~ 
```

-   Before the first '~ | fish', I change my currently active window. `xtitle` outputs the new window title and outputs it. Roller picks up the change.

## Truncate
This option is useful if you want to make sure that the text does not exceed a certain length
```sh
$ echo 'Rust' | roller -c 5 -t 5
Rust 
Rust 
Rust 
Rust 
Rust 
```

Same options, different text:

```sh
$ echo 'Rusty.' | roller -c 5 -t 5
Rusty
usty.
sty. 
ty. R
y. Ru
```

-   The `-t` flag causes text to be 'truncated' at a certain length. This means that if the text exceeds that length then it is scrolled and truncated at given length, otherwise it is neither scrolled nor truncated.

## Separator

```sh
$ echo 'Rust' | roller -c 3 -s ' -- '
Rust -- 
ust -- R
st -- Ru
```

By default, the separator is a single space. If the default separator was an empty string (''), the output would look like the following:

```
Rust
ustR
stRu
```

## Static prefix and postfix

```sh
$ echo 'Rust' | roller -a ' -- ' -b ' .. '
 .. Rust  -- 
 .. ust R -- 
 .. st Ru -- 
```


# License
Roller is open source software licensed under the terms of the MIT license.
