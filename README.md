[![Build Status](https://travis-ci.org/XOSplicer/comment-strip.svg?branch=master)](https://travis-ci.org/XOSplicer/comment-strip)
[![Latest Version](https://img.shields.io/crates/v/comment-strip.svg)](https://crates.io/crates/comment-strip)
# comment-strip
Strip comments away.\
Delete comments out of text files.

Comments are helpful in configuration files, source code,
shell scripts and any other text based files.
But sometimes they only distract from the actual content.
So with this tool you can _strip_ away the comments of the most common formats.

Currently supported comment styles:
- Shell style
- XML style (CDATA tag is not escaped)

Planned comment styles:
- C style


I started this project to enhance my practical knowledge of the rust programming language.
I could not find an easy applicable solution to the problem with detecting comments,
except for some _sed_ magic, so I decided to give it a try.
If you like the tool and would like to contribute or simply leave a note,
feel free to open an issue.


## Installing / Compiling
To install comment-strip you can use cargo.
Simply run
`cargo install comment-strip`
to install the latest version.

To compile comment-strip yourself, you need somewhat recent version of rust.
If you are using cargo, it's easy to get an executable.
1. `git clone git@github.com:XOSplicer/comment-strip.git`
2. `cd comment-strip`
3. `cargo build --release`
4. `cargo test` (optional)
5. `./target/release/strip --help` to display usage

## Usage
```
$ comment-strip --help
comment-strip 0.1.2
Felix Stegmaier <stegmaier.felix@gmail.com>
Strip comments away, default style is Shell comment style

USAGE:
    comment-strip [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -c, --c-style                  Strip away C style comments e.g. `/* some comment */` or `// some line comment`, only one style may be specified
    -h, --help                     Prints help information
    -B, --no-remove-blank-lines    don't remove multiple blank lines
    -s, --shell-style              Strip away shell style comments e.g. `# some line comment`, only one style may be specified
    -V, --version                  Prints version information
    -x, --xml-style                Strip away XML style comments e.g. `<!-- some comment -->`, only one style may be specified

OPTIONS:
    -o, --output <output>    Sets the output file to uses, uses stdout if not set

ARGS:
    <INPUT>    Sets the input file to use, uses stdin if not set
```

## Example
```
$ cat ./my_shell_script.sh
#!/bin/bash # you can comment in shebang
##########################
# My awsome shell script #
##########################
uname -a # display the current os
# let's find out who we are
whoami
pwd #and where we are
# uncomment to also list files
# ls -lah
echo 'Hello, "world" #no comment' #comment

$ comment-strip --shell-style my_shell_script.sh
#!/bin/bash
uname -a
whoami
pwd
echo 'Hello, "world" #no comment'

```


## Authors
- Felix Stegmaier

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
