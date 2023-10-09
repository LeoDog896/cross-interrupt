use std::process::Child;
use std::io::Result;

#[cfg(unix)]
fn terminate_unix(pid: i32) -> Result<()> {
    // SAFETY: https://man7.org/linux/man-pages/man2/kill.2.html this function doesn't seem to panic
    cvt::cvt(unsafe { libc::kill(pid, libc::SIGKILL) }).map(drop)
}

#[cfg(not(unix))]
fn terminate_windows(pid: i32) -> Result<()> {
    
}

pub fn terminate(child: &mut Child) -> Result<()> {
    if child.try_wait()?.is_some() {
        Ok(())
    } else {
        let pid = child.id();

        #[cfg(unix)]
        terminate_unix(pid);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
