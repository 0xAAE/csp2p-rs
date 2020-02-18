use std::{ptr, slice};
use std::sync::mpsc::Sender;

extern crate log;
use log::{debug, info, warn, error};

pub const NODE_ID_SIZE: usize = 32;
pub type NodeId = [u8; NODE_ID_SIZE];

extern crate hex; // hex::encode(node_id)
extern crate base58;
use base58::ToBase58; // [u8].to_base58()

mod raw;

pub type RawPacket = (NodeId, Vec<u8>);

static mut BYTES_SENDER: Option<Sender<RawPacket>> = None;

pub struct NodeInfo {
    pub id: Vec<u8>,
    pub ip: String,
    pub port: u16
}

pub struct CSHost {
    running: bool
}

impl CSHost {

    pub fn new(node_id: &[u8], tx: Sender<RawPacket>) -> Option<CSHost> {
        let len = node_id.len();
        if len != NODE_ID_SIZE {
            error!("panic! incorrect id size {}, must be {}", len, NODE_ID_SIZE);
            return None;
        }
        unsafe {
            let data: *const u8 = node_id.as_ptr();
            raw::host_init(data, len);
        }

        unsafe {
            BYTES_SENDER = Some(tx);
        }

        Some(CSHost {
            running: false
        })
    }

    pub fn add_known_hosts(&self, hosts: Vec<NodeInfo>) {
        for h in hosts {
            if h.id.len() != NODE_ID_SIZE {
                error!("panic! incorrect id size {}, must be {}", h.id.len(), NODE_ID_SIZE);
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
            error!("panic! incorrect id size {}, must be {}", id_size, NODE_ID_SIZE);
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
        debug!("message of {} bytes received from {}", &payload.len(), node_id[..].to_base58());
        unsafe {
            match BYTES_SENDER.clone() {
                None => (),
                Some(tx) => {
                    if tx.send((node_id, payload)).is_err() {
                        warn!("failed to send {} bytes to packet_collecor", data_size);
                    }
                }
            };
        }
    }

    extern "C" fn on_node_found(id: *const u8, id_size: usize) {
        if id_size != NODE_ID_SIZE {
            error!("panic! incorrect id size {}, must be {}", id_size, NODE_ID_SIZE);
            return;
        }
        let mut node_id: NodeId = Default::default();
        unsafe {
            ptr::copy(id, node_id.as_mut_ptr(), node_id.len());
        }
        info!("node {} found",  node_id[..].to_base58());
        unsafe {
            match BYTES_SENDER.clone() {
                None => (),
                Some(tx) => {
                    let payload = vec![1u8, 253u8]; // neighbour command == 1, found == 253
                    if tx.send((node_id, payload)).is_err() {
                        warn!("failed to send node found to packet_collector");
                    }
                }
            };
        }
    }

    extern "C" fn on_node_lost(id: *const u8, id_size: usize) {
        if id_size != NODE_ID_SIZE {
            error!("panic! incorrect id size {}, must be {}", id_size, NODE_ID_SIZE);
            return;
        }
        let mut node_id: NodeId = Default::default();
        unsafe {
            ptr::copy(id, node_id.as_mut_ptr(), node_id.len());
        }
        info!("node {} lost", node_id[..].to_base58());
        unsafe {
            match BYTES_SENDER.clone() {
                None => (),
                Some(tx) => {
                    let payload = vec![1u8, 254u8]; // neighbour command == 1, lost == 254
                    if tx.send((node_id, payload)).is_err() {
                        warn!("failed to send node lost to packet_collector");
                    }
                }
            };
        }
    }

    pub fn send_to(node_id: &[u8], data: &[u8]) {
        let key_size = node_id.len();
        if key_size != NODE_ID_SIZE {
            error!("panic! node_id must be {} bytes", NODE_ID_SIZE);
            return;
        }
        let key: *const u8 = node_id.as_ptr();
        let data_size = data.len();
        let data_ptr: *const u8 = data.as_ptr();

        unsafe {
            raw::send_to(key, key_size, data_ptr, data_size);
        }
    }

    pub fn broadcast(data: &[u8]) {
        let data_size = data.len();
        let data_ptr: *const u8 = data.as_ptr();

        unsafe {
            raw::broadcast(data_ptr, data_size);
        }
    }

    pub fn send_or_broadcast(node_id: &[u8], data: &[u8]) {
        let key_size = node_id.len();
        if key_size != NODE_ID_SIZE {
            error!("panic! node_id must be {} bytes", NODE_ID_SIZE);
            return;
        }
        let key: *const u8 = node_id.as_ptr();
        let data_size = data.len();
        let data_ptr: *const u8 = data.as_ptr();

        unsafe {
            raw::send_or_broadcast(key, key_size, data_ptr, data_size);
        }
    }
}
