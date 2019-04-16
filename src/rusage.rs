use std::mem;
use std::os;

use libc;

pub struct Rusage(libc::rusage);

pub type Errno = os::raw::c_int;

impl Rusage {
    pub fn children() -> Result<Self, Errno> {
        get_resource_usage(libc::RUSAGE_CHILDREN).map(Rusage)
    }

    pub fn current() -> Result<Self, Errno> {
        get_resource_usage(libc::RUSAGE_SELF).map(Rusage)
    }

    pub fn data(&self) -> Vec<(&'static str, i64)> {
        let d = self.0;

        vec![
            ("user time used", d.ru_utime.tv_usec),
            ("system time used", d.ru_stime.tv_usec),
            ("maximum resident set size", d.ru_maxrss),
            ("integral shared memory size", d.ru_ixrss),
            ("integral unshared data size", d.ru_idrss),
            ("integral unshared stack size", d.ru_isrss),
            ("page reclaims", d.ru_minflt),
            ("page faults", d.ru_majflt),
            ("swaps", d.ru_nswap),
            ("block input operations", d.ru_inblock),
            ("block output operations", d.ru_oublock),
            ("messages sent", d.ru_msgsnd),
            ("messages received", d.ru_msgrcv),
            ("signals received", d.ru_nsignals),
            ("voluntary context switches", d.ru_nvcsw),
            ("involuntary context switches", d.ru_nivcsw),
        ]
    }
}

fn get_resource_usage(who: os::raw::c_int) -> Result<libc::rusage, Errno> {
    let mut data = unsafe { mem::uninitialized() };

    let result = unsafe { libc::getrusage(who, &mut data) };

    if result == -1 {
        Err(unsafe { *libc::__errno_location() })
    } else {
        Ok(data)
    }
}
