extern crate seccomp_droundy;
extern crate libc;

use seccomp_droundy::*;

fn main() {
        let mut ctx = Context::default(Action::Allow).unwrap();
        let rule = Rule::new(105 /* setuid on x86_64 */,
            Compare::arg(0)
                .with(1000)
                .using(Op::Eq)
                .build().unwrap(),
            Action::Errno(libc::EPERM) /* return EPERM */
        );
        ctx.add_rule(rule).unwrap();
        ctx.load().unwrap();
        let ret = unsafe { libc::setuid(1000) };
        println!("ret = {}, uid = {}", ret, unsafe { libc::getuid() });
}
