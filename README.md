# getrusage(1)

`getrusage` is just an executable that displays the information returned by the `getrusage(3P)` function for the executed child command or the `getrusage` command itself if no command is supplied.

## Example

Display resource usage for `getrusage`:

``` shell
$ getrusage
```

Display resource usage for another command:

``` shell
$ getrusage ls -ld /dev
```

## Installation

If you have [Cargo](https://doc.rust-lang.org/stable/cargo/) installed you just need to run:

``` shell
cargo install getrusage
```

## Reporting issues

Found a bug? Please report it on the GitHub [issue tracker](https://github.com/anler/getrusage/issues).

## License

Licensed under the UNLICENSE.
