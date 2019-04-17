# getrusage(1)

`getrusage` is just an executable that displays the information returned by the `getrusage(3P)` function for the executed child command or the `getrusage` command itself if no command is supplied.

## Example

Display resource usage for `getrusage`:

``` shell
$ getrusage

---------------------------

user time used: 37908
system time used: 15122
maximum resident set size: 22652
integral shared memory size: 0
integral unshared data size: 0
integral unshared stack size: 0
page reclaims: 3303
page faults: 0
swaps: 0
block input operations: 0
block output operations: 64
messages sent: 0
messages received: 0
signals received: 0
voluntary context switches: 11
involuntary context switches: 12
```

Display resource usage for another command:

``` shell
$ getrusage ls -ld /dev

drwxr-xr-x 21 root root 3580 Apr 17 11:29 /dev

---------------------------

user time used: 2490
system time used: 0
maximum resident set size: 4152
integral shared memory size: 0
integral unshared data size: 0
integral unshared stack size: 0
page reclaims: 135
page faults: 0
swaps: 0
block input operations: 0
block output operations: 0
messages sent: 0
messages received: 0
signals received: 0
voluntary context switches: 1
involuntary context switches: 0
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
