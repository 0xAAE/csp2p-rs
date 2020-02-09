#if !defined(_P2P_PORT_INCLUDED)
#define _P2P_PORT_INCLUDED

#include "p2p/include/p2p_network.h"


typedef void MESSAGE_HANDLER(const uint8_t* key, size_t key_size, uint8_t* data, size_t data_size);
typedef void NODE_HANDLER(const uint8_t* key, size_t key_size);

// C interface to Host

extern "C" void host_start();

extern "C" void host_stop();

extern "C" void set_message_handler(MESSAGE_HANDLER* proc);
extern "C" void set_node_discovered_handler(NODE_HANDLER* proc);
extern "C" void set_node_removed_handler(NODE_HANDLER* proc);

#endif // _P2P_PORT_INCLUDED
