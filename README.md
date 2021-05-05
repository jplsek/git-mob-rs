# Git Mob (Rust Edition)

git_mob_rs is a rust version of [Git Mob](https://github.com/findmypast-oss/git-mob).

Please see their readme and blog for more information on why this type of tool exists.

This Rust version was made because I felt like learning some Rust and applying it to a tool I use everyday at work.
Since I use this everyday at work, one thing I did not like about git-mob was it's speed.
IMO, it shouldn't take ~50-120ms to set a template, so I wanted it to be faster by using a native implementation.

This version is not a one to one implementation. See the [differences](#differences) for more details.

"Hey this [rust version](https://github.com/Frost/git-mob) exists too!"
I actually didn't search for a native version until I added this readme. I probably should have done that.
Well, if you need a more feature complete version, use theirs!

## Usage

### Add/edit/delete co-author

```
$ git edit-coauthors
```

This will edit `~/.config/git-coauthors` with your default text editor.
Use the following json syntax to make the file.

```json
{
  "coauthors": {
    "fl": {
      "name": "First Last",
      "email": "firstlast@example.com"
    }
  }
}
```

### Mobbing co-author

```
$ git mob fl
```

Or multiple:

```
$ git mob fl ab cd ef
```

### Reset mob, going back solo

```
$ git solo
``` 

## Install

### Mac

TODO

### Linux

TODO

### Windows

TODO

### From cargo

```
cargo install git_mob_rs
```

### From source

```
git clone https://github.com/jplsek/git-mob-rs && cd git-mob-rs
cargo install
```

## Differences

- The XDG config directory is used by default (`~/.config/git-coauthors`) for the configuration, falling back to `~/.git-coauthors` if it exists.
- I personally never used the add/delete/edit coauthor commands from the original, as I edited the config file directly, so I did not include those commands. Instead, I added a `git edit-coauthors` command.
- Since I primarily use the CLI, I won't make editor plugins related to git-mob-rs.

If someone else feels like making some of these missing features, feel free to submit a PR!

## Benchmarks

Summary: **Over 15x faster**

### `git-solo`

```
+ hyperfine --warmup 3 -- git-solo target/release/git-solo
Benchmark #1: git-solo
Time (mean ± σ):      80.4 ms ±  22.6 ms    [User: 84.4 ms, System: 7.7 ms]
Range (min … max):    70.0 ms … 166.5 ms    40 runs

Benchmark #2: target/release/git-solo
Time (mean ± σ):       4.3 ms ±   0.1 ms    [User: 4.4 ms, System: 0.7 ms]
Range (min … max):     4.2 ms …   4.9 ms    497 runs

Summary
'target/release/git-solo' ran
18.52 ± 5.22 times faster than 'git-solo'
```

### `git-mob`

```
+ hyperfine --warmup 3 -- 'git-mob ts' 'target/release/git-mob ts'
Benchmark #1: git-mob ts
Time (mean ± σ):      88.5 ms ±  24.4 ms    [User: 93.0 ms, System: 8.8 ms]
Range (min … max):    80.1 ms … 223.4 ms    36 runs

Benchmark #2: target/release/git-mob ts
Time (mean ± σ):       4.4 ms ±   0.1 ms    [User: 4.4 ms, System: 0.7 ms]
Range (min … max):     4.3 ms …   5.0 ms    510 runs

Summary
'target/release/git-mob ts' ran
20.17 ± 5.59 times faster than 'git-mob ts'
```
