# Yatter

Yet another Time Tracker.

## About

I don't think there's a single developer out there that enjoys time tracking and if you do, you are as rare as a unicorn. I find it to be a tedious chore and the web based solutions do not help. The entire process is so slow and in my case I have to add entries in more than one system and so I avoid using them until I absolutely have to use them. Instead I write the entries in a text file and go through the pain of importing everything by hand every week.

### A potential solution?

I've been thinking there has to be a way to automate this process (and I also need an excuse to learn Rust 😄) and thus `yatter` was born.

Imagine a world where you can type a command like `sync` and have a timesheet uploaded to multiple external systems.

_Magic 🪄_

As of now, it only solves the "manually typing in entries in a file" problem. But I'm exploring the possibilities of integrating it with other systems.

See the [Roadmap](#roadmap) section for info on that and if you have a similar problem and want to contribute can have a look at the [Contributing](#contributing) section. 😊

## Installation

I'd be surprised if anyone else finds this tool useful and so if you want to install it you'd have to build the binaries. To do that you'd need to have [Rust](https://www.rust-lang.org/tools/install) installed.

After that you can clone this repo:

```
git clone https://github.com/AntonisManiatis/yatter.git
```

cd into your cloned repo and do:

```
cargo build --release
```

After `cargo` is finished you'll find the compiled binary for your OS & architecture in `./target/release`.

Finally set a [Path environment variable](<https://www3.ntu.edu.sg/home/ehchua/programming/howto/Environment_Variables.html#:~:text=To%20set%20(or%20change)%20a,it%20to%20an%20empty%20string.>) pointing to the binary so you can use `yatter` from your shell (is this correct?).

## Usage

Let's assume you have your files structured like this:

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

<!-- TODO: Should I add comments for what the tool prints to the screen? -->

Open up a terminal and type:

```
yatter init ./work/project_one
```

To punch in for that project:

```
yatter punch ./work/project_one
```

After you are done with your task you again use `punch` again to punch out:

```
yatter punch ./work/project_x -d "Added search functionality to navbar."
```

For additional info please use:

```
yatter --help
```

## Roadmap

Here are a few ideas/features that I'll implement in the future:

- Automatic punch-out when you switch projects.
- Possibility to send timesheets to other web-based time tracking services like HeavenHR, etc with a push of a button (or command in our case 😄).
- Explore the possibility for a `git` integration using hooks.

## Contributing

<!-- TODO: Touch this up a bit. -->

If you have any ideas, you found any nastyyy bugs or just want to write some Rust contributions are welcomed!

## License

Yatter is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE_APACHE](https://github.com/AntonisManiatis/yatter/blob/master/LICENSE_APACHE) and [LICENSE_MIT](https://github.com/AntonisManiatis/yatter/blob/master/LICENSE_MIT) for details.
