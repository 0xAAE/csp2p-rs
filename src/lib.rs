#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod raw;

pub struct CSHost {
    running: bool
}

impl CSHost {

    pub fn new() -> Option<CSHost> {
        Some(CSHost {
            running: false
        })
    }

    pub fn start(&mut self) -> bool {
        if self.running {
            return false;
        }

        unsafe {
            raw::host_start();
        }

        self.running = true;
        true
    }

    pub fn stop(&mut self) {
        if self.running {
            unsafe {
                raw::host_stop();
            }
            self.running = false;
        }
    }
}
