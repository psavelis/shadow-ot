/**
 * Shadow OT Client - Container Implementation
 */

#include "container.h"
#include <algorithm>

namespace shadow {
namespace client {

Container::Container() = default;

void Container::addItem(ItemPtr item) {
    if (!item || isFull()) return;

    m_items.push_back(item);

    if (m_onItemAdd) {
        m_onItemAdd(static_cast<int>(m_items.size()) - 1, item);
    }
}

void Container::insertItem(int slot, ItemPtr item) {
    if (!item) return;

    if (slot < 0 || slot > static_cast<int>(m_items.size())) {
        addItem(item);
        return;
    }

    m_items.insert(m_items.begin() + slot, item);

    if (m_onItemAdd) {
        m_onItemAdd(slot, item);
    }
}

void Container::removeItem(int slot) {
    if (slot < 0 || slot >= static_cast<int>(m_items.size())) return;

    m_items.erase(m_items.begin() + slot);

    if (m_onItemRemove) {
        m_onItemRemove(slot);
    }
}

void Container::updateItem(int slot, ItemPtr item) {
    if (slot < 0 || slot >= static_cast<int>(m_items.size())) return;

    m_items[slot] = item;

    if (m_onItemUpdate) {
        m_onItemUpdate(slot, item);
    }
}

void Container::clear() {
    m_items.clear();
}

ItemPtr Container::getItem(int slot) const {
    if (slot >= 0 && slot < static_cast<int>(m_items.size())) {
        return m_items[slot];
    }
    return nullptr;
}

// ContainerManager implementation

ContainerManager& ContainerManager::instance() {
    static ContainerManager instance;
    return instance;
}

void ContainerManager::clear() {
    m_containers.clear();
}

ContainerPtr ContainerManager::createContainer(uint8_t id) {
    auto container = std::make_shared<Container>();
    container->setId(id);
    m_containers[id] = container;

    if (m_onOpen) {
        m_onOpen(container);
    }

    return container;
}

void ContainerManager::removeContainer(uint8_t id) {
    auto it = m_containers.find(id);
    if (it != m_containers.end()) {
        m_containers.erase(it);

        if (m_onClose) {
            m_onClose(id);
        }
    }
}

ContainerPtr ContainerManager::getContainer(uint8_t id) const {
    auto it = m_containers.find(id);
    if (it != m_containers.end()) {
        return it->second;
    }
    return nullptr;
}

ContainerPtr ContainerManager::getContainerByItemId(uint16_t itemId) const {
    for (const auto& [id, container] : m_containers) {
        if (container->getContainerItemId() == itemId) {
            return container;
        }
    }
    return nullptr;
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::ContainerManager& g_containers = shadow::client::ContainerManager::instance();
