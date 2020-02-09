extern crate bitcoin;
use bitcoin::util::key::PublicKey;

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
            raw::set_host_message_handler(CSHost::on_message);
            raw::set_host_discover_node_handler(CSHost::on_node_found);
            raw::set_host_lost_node(CSHost::on_node_lost);
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

    fn on_message(id: *const u8, id_size: usize, data: *mut u8, data_size: usize) {
        println!("message received");
    }

    fn on_node_found(id: *const u8, id_size: usize) {
        println!("node found");
    }

    fn on_node_lost(id: *const u8, id_size: usize) {
        println!("node lost");
    }
}
