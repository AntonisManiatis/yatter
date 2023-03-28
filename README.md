# Yatter

[![license-mit](https://badgen.net/badge/license/MIT/blue)](https://github.com/AntonisManiatis/yatter/blob/master/LICENSE_MIT)
[![license-apache](https://badgen.net/badge/license/Apache-2.0/blue)](https://github.com/AntonisManiatis/yatter/blob/master/LICENSE_APACHE)
[![Rust](https://github.com/AntonisManiatis/yatter/actions/workflows/rust.yml/badge.svg)](https://github.com/AntonisManiatis/yatter/actions/workflows/rust.yml)

Yet another Time Tracker is a CLI punch in & out time tracker.

<!-- TODO: Maybe a gif here? -->

Another one? Aren't there like a bunch of better options out there?

Certainly! I just needed an excuse to start learning Rust. 😄

## Installation

There isn't a released version of yatter yet and so if you want to install it you'd have to build the binaries yourself. To do that you'd need to have [Rust](https://www.rust-lang.org/tools/install) installed.

After that you can clone this repo:

```
git clone https://github.com/AntonisManiatis/yatter.git
```

cd into your cloned repo and do:

```
cargo build --release
```

After `cargo` is finished you'll find the compiled binary for your OS & architecture in `./target/release`.

Finally set a [Path environment variable](<https://www3.ntu.edu.sg/home/ehchua/programming/howto/Environment_Variables.html#:~:text=To%20set%20(or%20change)%20a,it%20to%20an%20empty%20string.>) pointing to the binary so you can use `yatter` from your shell.

### Starship

If you have [starship](https://starship.rs/) you can add this to your starship.toml.

<!-- TODO: Not final! -->

```
[custom.yatter]
command = "yatter status"
when = "yatter status"
symbol = "" # test one, I don't want to get sued.
format = " [$symbol($output)]($style) "
style = "bold yellow"
```

Here's how that would look like:

<!-- TODO: Add a picture here. -->

## Usage

Let's assume you have your work projects structured like this:

```
.
├── ...
├── work
│   ├── project_x
│   │   ├── ...
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
yatter init ./work/project_x
```

To punch in for that project:

```
yatter punch ./work/project_x
```

After you are done with your task you again use `punch` to punch out:

```
yatter punch ./work/project_x -d "Added search functionality to navbar."
```

To check if you have punched in or out for a project:

```
yatter status ./work/project_x
```

For additional info please use:

```
yatter --help
```

## Contributing

If you have any ideas or features to suggest, or found any bugs.

See [CONTRIBUTING.md](https://github.com/AntonisManiatis/yatter/tree/master/.github/CONTRIBUTING.md) for details.

## License

Yatter is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE_APACHE](https://github.com/AntonisManiatis/yatter/blob/master/LICENSE_APACHE) and [LICENSE_MIT](https://github.com/AntonisManiatis/yatter/blob/master/LICENSE_MIT) for details.
