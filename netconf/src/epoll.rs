use crate::AutoCloseFD;
use crate::FD;

pub struct Select<'a> {
    read_fds: Vec<&'a AutoCloseFD>,
}

impl<'a> Select<'a> {
    pub fn new() -> Self {
        Self { read_fds: vec![] }
    }

    pub fn add_read_fd(&mut self, fd: &'a AutoCloseFD) {
        if !self.read_fds.iter().any(|x| x.get() == fd.get()) {
            self.read_fds.push(fd);
        }
    }

    pub fn poll(&self) {
        //let read_fd_set = self.read_fds.iter().map(|fd|
        //					   nc::fd_set_t {
        //					   }
        //);

        //nc::select();
    }
}
