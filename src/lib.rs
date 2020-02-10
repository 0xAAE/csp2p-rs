use std::{ptr, slice, str};

pub const NODE_ID_SIZE: usize = 32;
pub type NodeId = [u8; NODE_ID_SIZE];

extern crate hex; // hex::encode(node_id)
extern crate bitcoin;
use bitcoin::util::base58;

mod raw;

#[test]
fn it_works() {
    unsafe {
        let bbb: NodeId = Default::default();
        let data: *const u8 = bbb.as_ptr();
        let len = bbb.len(); 
        raw::host_init(data, len);
        raw::host_start();
        raw::set_message_handler(on_message);
        raw::set_node_discovered_handler(on_node_found);
        raw::set_node_discovered_handler(on_node_lost);
    }
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

    pub fn new(node_id: &[u8]) -> Option<CSHost> {
        let len = node_id.len();
        if len != NODE_ID_SIZE {
            return None;
        }
        unsafe {
            let data: *const u8 = node_id.as_ptr();
            raw::host_init(data, len);
        }

        Some(CSHost {
            running: false
        })
    }

    pub fn add_known_hosts(&self, hosts: Vec<NodeInfo>) {
        for h in hosts {
            if h.id.len() != NODE_ID_SIZE {
                // todo panic!
                println!("add_known_host() panic! incorrect id size {}, must be {}", h.id.len(), NODE_ID_SIZE);
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
        if id_size != NODE_ID_SIZE {
            println!("on_message() panic! incorrect id size {}, must be {}", id_size, NODE_ID_SIZE);
            return;
        }
        let mut node_id: NodeId = Default::default();
        unsafe {
            ptr::copy(id, node_id.as_mut_ptr(), node_id.len());
        }
        let mut payload = Vec::<u8>::new();
        if data_size > 0 {
            unsafe {
                payload = slice::from_raw_parts(data, data_size).to_vec();
            }
        } 
        println!("Message of {} bytes received from {}", payload.len(), base58::encode_slice(&node_id[..]));
    }

    extern "C" fn on_node_found(id: *const u8, id_size: usize) {
        if id_size != NODE_ID_SIZE {
            println!("on_node_found() panic! incorrect id size {}, must be {}", id_size, NODE_ID_SIZE);
            return;
        }
        let mut node_id: NodeId = Default::default();
        unsafe {
            ptr::copy(id, node_id.as_mut_ptr(), node_id.len());
        }
        println!("Node {} found",  base58::encode_slice(&node_id[..]));
    }

    extern "C" fn on_node_lost(id: *const u8, id_size: usize) {
        if id_size != NODE_ID_SIZE {
            println!("on_node_lost() panic! incorrect id size {}, must be {}", id_size, NODE_ID_SIZE);
            return;
        }
        let mut node_id: NodeId = Default::default();
        unsafe {
            ptr::copy(id, node_id.as_mut_ptr(), node_id.len());
        }
        println!("Node {} lost", base58::encode_slice(&node_id[..]));
    }
}
