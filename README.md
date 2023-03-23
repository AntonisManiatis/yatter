# Yatter

Yet another Time Tracker

## Description

I know what you're thinking God another one?? How is this different or better than any other fancy, cloud based time-tracking solution out there?

Well it's not. I kinda needed an excuse to learn Rust and that's the result.

But allow me to provide some context as to what problem this little CLI tool tries to solve.

I hate having to log hours and I know I'm not the only one.

- I have to login.
- wait for the page to load.
- navigate & press buttons, wait some more.
- search for today.
- make a mistake or click off somewhere and now I have to re-do this again.

Not only that but there are cases where I have to do this for _multiple systems!_

And so I have developed an alternative strategy and that is to log the hours into text files and you already know that I'm Agile and follow the [Last Responsible Moment (LRM)](https://blog.codinghorror.com/the-last-responsible-moment/) principle, or in other words when my manager bugs me I have to go through the pain of importing everything to those systems as well.

I say no more! there has to be a better way to do this! and so I thought...

## Installation

I'd be surprised if anyone else finds this thing useful BUT if you needed to install it you'd find the instructions here 😊.

<!--
Inspirations for installation section.
https://github.com/helium/helium-wallet-rs
https://github.com/Rigellute/spotify-tui
-->

<!-- There aren't any yet. XD
### Pre-built binaries

You can download pre-built binaries on the [release page]().
-->

<!-- There's no binaries uploaded to cargo yet.
### Cargo

If there are no pre-built binaries found on the [release page](), you can try installing it using Cargo directly.

Install [Rust](https://www.rust-lang.org/tools/install) (using the recommended rustup installation method) and then

```
cargo install yatter
```
-->

## Usage

Let's assume you have your files structured like this:

```
.
├── ...
├── work
│   ├── project_x
│	│	├── ...
│   │   ├── docs            # project related docs
│   │   ├── be_repo         # back-end git repo
│   │   └── fe_repo         # front-end git repo
│   ├── project_y
│   │   └── ...
│   └── project_z
│       └── ...
└── ...
```

And you have a task that you'd like to track your time for `project_x`.

Open up a terminal and type:

```
yatter punch ./work/project_one
```

if you have a lil' peek into `./work/project_one/` now you'll find that `yatter` has created a directory called `hours/{current-year}/` and in it there's a file called `{MM-YYYY}` where MM and YYYY are the current month & year.

To check the status:

```
yatter status ./work/project_x/
```

<!-- TODO: touch this section up a bit. -->

You get:

```
- 19-03-2023
	- 09:14 to
```

We have a new entry! Splendid! So after our work is done we can do:

```
yatter punch ./work/project_x
```

if we were to re-open the hour log file now we'd see another entry appended at the end:

```
- 19-03-2023
	- 09:14 to 11:42
```

For additional info please use:

```
yatter --help
```

## Roadmap

Here are a few ideas/features that I'll implement in the future:

- Automatic punch-out when you switch projects.
- Explore the possibility for a `git` integration using hooks.
- Possibility to send timesheets to other web-based time tracking services like HeavenHR, etc with a push of a button (or command in our case 😄).

## Contributing

If you have any ideas, you found any nastyyy bugs or just want to write some Rust contributions are welcomed!

## License

Yatter is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE_APACHE and LICENSE_MIT for details.
