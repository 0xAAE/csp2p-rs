pub type MessageHandler = extern "C" fn(id: *const u8, id_size: usize, data: *mut u8, data_size: usize);
pub type NodeHandler = extern "C" fn(id: *const u8, id_size: usize);
pub type FragmentHandler = extern "C" fn(id: *const u8, id_size: usize, data: *mut u8, data_size: usize);
pub type NoFragmentHandler = extern "C" fn(id: *const u8, id_size: usize);
pub type FragmentIdFactory = extern "C" fn(data: *const u8, data_size: usize, id_buf: *mut u8, id_buf_size: usize);

#[link(name = "p2p-compat")]
extern "C" {
    pub fn host_init(key: *const u8, key_size: usize);
    pub fn host_add_entry_point(key: *const u8, key_size: usize, ip: *const u8, ip_size: usize, port: u16);
    pub fn host_start();
    pub fn host_stop();

    pub fn set_message_handler(pfn: MessageHandler);
    pub fn set_node_discovered_handler(pfn: NodeHandler);
    pub fn set_node_removed_handler(pfn: NodeHandler);
    pub fn set_fragment_handler(pfn: FragmentHandler);
    pub fn set_no_fragment_handler(pfn: NoFragmentHandler);
    pub fn set_fragment_id_factory(pfn: FragmentIdFactory);

    pub fn send_to(key: *const u8, key_size: usize, data: *const u8, data_size: usize);
    pub fn broadcast(data: *const u8, data_size: usize);
    pub fn send_or_broadcast(key: *const u8, key_size: usize, data: *const u8, data_size: usize);
}
