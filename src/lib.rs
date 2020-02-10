mod raw;

#[test]
fn it_works() {
    unsafe {
        let bbb = [0u8; 32];
        let data: *const u8 = bbb.as_ptr();
        let len = bbb.len(); 
        raw::host_init(data, len);
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

pub struct NodeInfo {
    pub id: Vec<u8>,
    pub ip: String,
    pub port: u16
}

pub struct CSHost {
    running: bool
}

impl CSHost {

    pub fn new(public_key: &[u8]) -> Option<CSHost> {
        let len = public_key.len();
        if len != 32 {
            return None;
        }
        unsafe {
            let data: *const u8 = public_key.as_ptr();
            raw::host_init(data, len);
        }

        Some(CSHost {
            running: false
        })
    }

    pub fn add_known_hosts(&self, hosts: Vec<NodeInfo>) {
        for h in hosts {
            if h.id.len() != 32 {
                // todo panic!
                continue;
            }
            unsafe {
                raw::host_add_entry_point(h.id.as_ptr(), h.id.len(), h.ip.as_ptr(), h.ip.len(), h.port);
            }
        }
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
