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

        static std::unique_ptr<HostParams*> ptr_params;
    };

    /*static*/
    HostParams& instance() {
        if(!ptr_params) {
            ptr_params = std::make_unique<HostParams>();
        }
        return *HostParams::ptr_params.get();
    }

    /*static*/
    std::unique_ptr<HostParams*> HostParams::ptr_params;

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
                if(config.use_default_boot_nodes && !config.custom_boot_nodes.empty()) {
                    ptr_config->use_default_boot_nodes = true;
                    ptr_config->custom_boot_nodes = config.custom_boot_nodes;
                }

                ptr_host = std::make_unique<Host>(*ptr_config, *this);
            }

            virtual ~HostHandler() = default;

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
    if(id != nullptr && id.size() == id_size) {
        const auto& id = HostParams::instance().id;
        std::copy(id, id + id_size(), reinterpret_cast<uint8_t*>(id.GetPtr()));
    }
    else {
        // todo panic! not valid id provided
    }
}

void host_start() {
    HostHandler::instance();
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
