extern crate bitcoin;
use bitcoin::util::key::PublicKey;
mod raw;

#[test]
fn it_works() {
    unsafe {
        raw::host_start();
        raw::set_message_handler(on_message);
        raw::set_node_discovered_handler(on_node_found);
        raw::set_node_discovered_handler(on_node_lost);
    }
}

extern "C" fn on_message(id: *const u8, id_size: usize, data: *mut u8, data_size: usize) {
    println!("message received");
}

extern "C" fn on_node_found(id: *const u8, id_size: usize) {
    println!("node found");
}

extern "C" fn on_node_lost(id: *const u8, id_size: usize) {
    println!("node lost");
}

pub struct CSHost {
    running: bool
}

impl CSHost {

    pub fn new(public_key: &PublicKey) -> Option<CSHost> {
        unsafe {
            let bytes = public_key.to_bytes();
            let data: *const u8 = bytes.as_ptr();
            let len = bytes.len(); 
            raw::host_init(data, len);
        }

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
            raw::set_message_handler(CSHost::on_message);
            raw::set_node_discovered_handler(CSHost::on_node_found);
            raw::set_node_removed_handler(CSHost::on_node_lost);
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

    extern "C" fn on_message(id: *const u8, id_size: usize, data: *mut u8, data_size: usize) {
        println!("message received");
    }

    extern "C" fn on_node_found(id: *const u8, id_size: usize) {
        println!("node found");
    }

    extern "C" fn on_node_lost(id: *const u8, id_size: usize) {
        println!("node lost");
    }
}
