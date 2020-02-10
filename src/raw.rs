pub type MessageHandler = extern "C" fn(id: *const u8, id_size: usize, data: *mut u8, data_size: usize);
pub type NodeHandler = extern "C" fn(id: *const u8, id_size: usize);

#[link(name = "p2p-compat")]
extern "C" {
    pub fn host_init(key: *const u8, key_size: usize);
    pub fn host_add_entry_point(key: *const u8, key_size: usize, ip: *const u8, ip_size: usize, port: u16);
    pub fn host_start();
    pub fn host_stop();

    pub fn set_message_handler(pfn: MessageHandler);
    pub fn set_node_discovered_handler(pfn: NodeHandler);
    pub fn set_node_removed_handler(pfn: NodeHandler);
}
