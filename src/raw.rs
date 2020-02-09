#[link(name = "p2p-compat")]

pub type MessageHandler = fn(id: *const u8, id_size: usize, data: *mut u8, data_size: usize);
pub type NodeHandler = fn(id: *const u8, id_size: usize);

extern {
    pub fn host_start();
    pub fn host_stop();

    pub fn set_host_message_handler(pfn: MessageHandler);
    pub fn set_host_discover_node_handler(pfn: NodeHandler);
    pub fn set_host_lost_node(pfn: NodeHandler);
}
