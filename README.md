[![Build Status](https://travis-ci.org/XOSplicer/comment-strip.svg?branch=master)](https://travis-ci.org/XOSplicer/comment-strip)

# comment-strip
Strip comments away.
Delete comments out of text files.

Currently supported comment styles:
- Shell style

Planned comment styles:
- C style
- XML style

## Compiling and Running
To compile comment-strip you need somewhat recent version of rust.
If you are using cargo, it's easy to get an executable.
1. `git clone git@github.com:XOSplicer/comment-strip.git`
2. `cd comment-strip`
3. `cargo build --release`
4. `cargo test` (optional)
5. `./target/release/strip --help` to display usage

## Usage
```
$ ./strip --help          
strip 0.1.0
Felix Stegmaier <stegmaier.felix@gmail.com>
Strip comments away, default style is Shell comment style

USAGE:
    strip [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -c, --c-style        Strip away C style comments e.g. `/* some comment */` or `// some line comment`, only one style may be specified
    -h, --help           Prints help information
    -s, --shell-style    Strip away shell style comments e.g. `# some line comment`, only one style may be specified
    -V, --version        Prints version information
    -x, --xml-style      Strip away XML style comments e.g. `<!-- some comment -->`, only one style may be specified

OPTIONS:
    -o, --output <output>    Sets the output file to uses, uses stdout if not set

ARGS:
    <INPUT>    Sets the input file to use, uses stdin if not set
```

## Authors
- Felix Stegmaier

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
