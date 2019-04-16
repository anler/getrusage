use std::mem;
use std::os;

use libc;

pub struct Rusage(libc::rusage);

impl Rusage {
    pub fn children() -> Self {
        let mut data = unsafe { mem::uninitialized() };
        let _result = get_resource_usage(&mut data);

        Rusage(data)
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

fn get_resource_usage(data: &mut libc::rusage) -> os::raw::c_int {
    let result = unsafe { libc::getrusage(libc::RUSAGE_CHILDREN, data) };

    result
}
