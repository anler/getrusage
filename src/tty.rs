use libc;

pub fn in_tty() -> bool {
    let fd = libc::STDOUT_FILENO;
    unsafe { libc::isatty(fd) != 0 }
}
