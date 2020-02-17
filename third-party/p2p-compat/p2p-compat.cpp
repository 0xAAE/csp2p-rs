#include "p2p-compat.h"
#include <memory>

using namespace net;

namespace
{
    struct HostParams {
        NodeId id;
        std::string listen_address;
        uint16_t listen_port = 0;
        bool traverse_nat = true;
        bool use_default_boot_nodes = true;
        std::vector<NodeEntrance> custom_boot_nodes;

        static HostParams& instance();

    private:

        static std::unique_ptr<HostParams> ptr_params;
    };

    /*static*/
    HostParams& HostParams::instance() {
        if(!HostParams::ptr_params) {
            HostParams::ptr_params = std::make_unique<HostParams>();
        }
        return *HostParams::ptr_params.get();
    }

    /*static*/
    std::unique_ptr<HostParams> HostParams::ptr_params;

    class HostHandler: public HostEventHandler {
        public:
            HostHandler()
            : ptr_message_handler(nullptr)
            , ptr_node_discovered_handler(nullptr)
            , ptr_node_removed_handler(nullptr) {
                
                ptr_config = std::make_unique<Config>();
                const auto& config = HostParams::instance();
                ptr_config->id = config.id;
                if(!config.listen_address.empty()) {
                    ptr_config->listen_address = config.listen_address;
                }
                if(config.listen_port != 0) {
                    ptr_config->listen_port = 0;
                }
                ptr_config->traverse_nat = config.traverse_nat;
                if(!config.use_default_boot_nodes && !config.custom_boot_nodes.empty()) {
                    ptr_config->use_default_boot_nodes = false;
                    ptr_config->custom_boot_nodes = config.custom_boot_nodes;
                }

                std::cout << "init host: use def boot nodes=" << ptr_config->use_default_boot_nodes
                    << ", addr=" << ptr_config->listen_address << ":" << ptr_config->listen_port
                    << ", entry points " << ptr_config->custom_boot_nodes.size()
                    << std::endl;

                ptr_host = std::make_unique<Host>(*ptr_config, *this);
            }

            virtual ~HostHandler() = default;

            void run() {
                if(ptr_host) {
                    std::cout << "calling to host->Run()" << std::endl;
                    ptr_host->Run();
                }
                else {
                    std::cout << "unable to call to host->Run()" << std::endl;
                }
            }

            /*virtual*/
            void OnMessageReceived(const NodeId& from, ByteVector&& message) override;

            /*virtual*/
            void OnNodeDiscovered(const NodeId&) override;
            
            /*virtual*/
            void OnNodeRemoved(const NodeId&) override;

            static HostHandler& instance();
            void destroy();

            void set_message_handler(MESSAGE_HANDLER* proc) {
                ptr_message_handler = proc;
            }

            void set_node_discovered_handler(NODE_HANDLER* proc) {
                ptr_node_discovered_handler = proc;
            }

            void set_node_removed_handler(NODE_HANDLER* proc) {
                ptr_node_removed_handler = proc;
            }

            void send_to(const NodeId& to, ByteVector&& msg) {
                if(ptr_host) {
                    ptr_host->SendDirect(to, std::move(msg));
                }
            }

            void broadcast(ByteVector&& msg) {
                if(ptr_host) {
                    ptr_host->SendBroadcast(std::move(msg));
                }
            }

            void send_or_broadcast(const NodeId& to, ByteVector&& msg) {
                if(ptr_host) {
                    ptr_host->SendBroadcastIfNoConnection(to, std::move(msg));
                }
            }

        private:
            std::unique_ptr<Config> ptr_config;
            std::unique_ptr<Host> ptr_host;

            MESSAGE_HANDLER* ptr_message_handler;
            NODE_HANDLER* ptr_node_discovered_handler;
            NODE_HANDLER* ptr_node_removed_handler;

            static std::unique_ptr<HostHandler> ptr_inst;
    };

    void HostHandler::OnMessageReceived(const NodeId& from, ByteVector&& message) {
        if(ptr_message_handler != nullptr) {
            ptr_message_handler(
                reinterpret_cast<const uint8_t*>(from.GetPtr()),
                from.size(),
                message.data(),
                message.size()
            );
        }
    }

    void HostHandler::OnNodeDiscovered(const NodeId& id) {
        if(ptr_node_discovered_handler != nullptr) {
            ptr_node_discovered_handler(
                reinterpret_cast<const uint8_t*>(id.GetPtr()),
                id.size()
            );
        }
    }

    void HostHandler::OnNodeRemoved(const NodeId& id) {
        if(ptr_node_removed_handler != nullptr) {
            ptr_node_removed_handler(
                reinterpret_cast<const uint8_t*>(id.GetPtr()),
                id.size()
            );
        }
    }

    /*static*/
    HostHandler& HostHandler::instance() {
        if(!HostHandler::ptr_inst) {
            HostHandler::ptr_inst = std::make_unique<HostHandler>();
        }
        return *ptr_inst.get();
    }

    void HostHandler::destroy() {
        if(HostHandler::ptr_inst) {
            ptr_inst.reset();
        }
    }

    /*static*/
    std::unique_ptr<HostHandler> HostHandler::ptr_inst;
}

void host_init(const uint8_t* key, size_t key_size) {
    auto& id = HostParams::instance().id;
    if(key != nullptr && id.size() == key_size) {
        std::copy(key, key + key_size, reinterpret_cast<uint8_t*>(id.GetPtr()));
    }
    else {
        std::cout << "host_init(): panic! not valid id provided" << std::endl;
    }
}

void host_add_entry_point(const uint8_t* key, size_t key_size, const uint8_t* ip, size_t ip_size, uint16_t port) {
    if(key_size != 32) {
        std::cout << "panic! wrong public key size " << key_size << ", must be 32" << std::endl;
    }

    std::string node_ip;
    node_ip.resize(ip_size);
    std::copy(ip, ip + ip_size, node_ip.data());

    net::NodeEntrance entry;
    entry.address = net::bi::address::from_string(node_ip);
    entry.udp_port = entry.tcp_port = port;
    std::copy(key, key + key_size, reinterpret_cast<uint8_t*>(entry.id.GetPtr()));

    auto& p = HostParams::instance();
    p.use_default_boot_nodes = false;
    p.custom_boot_nodes.push_back(entry);

    std::cout << "Added entry node, total " << p.custom_boot_nodes.size() << std::endl;
}

void host_start() {
    std::cout << "Starting p2p host" << std::endl;
    HostHandler::instance().run();
    std::cout << "p2p host started" << std::endl;
}

void host_stop() {
    HostHandler::instance().destroy();
}

void set_message_handler(MESSAGE_HANDLER* proc) {
    HostHandler::instance().set_message_handler(proc);
}

void set_node_discovered_handler(NODE_HANDLER* proc) {
    HostHandler::instance().set_node_discovered_handler(proc);
}

void set_node_removed_handler(NODE_HANDLER* proc) {
    HostHandler::instance().set_node_removed_handler(proc);
}

constexpr size_t NodeIdSize() {
    return sizeof(NodeId);
}

void send_to(const uint8_t* key, size_t key_size, const uint8_t* data, size_t data_size) {
    if(key == nullptr || key_size != NodeIdSize()) {
        std::cout << "send_to() panic! key_size must be " << NodeIdSize() << " bytes";
        return;
    }
    NodeId id;
    std::copy(key, key + key_size, reinterpret_cast<uint8_t*>(id.GetPtr()));
    if(data == nullptr || data_size == 0) {
        HostHandler::instance().send_to(id, std::vector<uint8_t>());
    }
    else {
        std::vector<uint8_t> buf(data, data + data_size);
        HostHandler::instance().send_to(id, std::move(buf));
    }
}

void broadcast(const uint8_t* data, size_t data_size) {
    if(data == nullptr || data_size == 0) {
        HostHandler::instance().broadcast(std::vector<uint8_t>());
    }
    else {
        std::vector<uint8_t> buf(data, data + data_size);
        HostHandler::instance().broadcast(std::move(buf));
    }
}

void send_or_broadcast(const uint8_t* key, size_t key_size, const uint8_t* data, size_t data_size) {
    if(key == nullptr || key_size != NodeIdSize()) {
        std::cout << "send_otr_broadcast() panic! key_size must be " << NodeIdSize() << " bytes";
        return;
    }
    NodeId id;
    std::copy(key, key + key_size, reinterpret_cast<uint8_t*>(id.GetPtr()));
    if(data == nullptr || data_size == 0) {
        HostHandler::instance().send_or_broadcast(id, std::vector<uint8_t>());
    }
    else {
        std::vector<uint8_t> buf(data, data + data_size);
        HostHandler::instance().send_or_broadcast(id, std::move(buf));
    }
}
