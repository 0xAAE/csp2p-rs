#include "p2p-compat.h"
#include <memory>

using namespace net;

namespace
{

    class HostHandler: public HostEventHandler {
        public:
            HostHandler()
            : ptr_message_handler(nullptr)
            , ptr_node_discovered_handler(nullptr)
            , ptr_node_removed_handler(nullptr) {
                ptr_config = std::make_unique<Config>();
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
