/**
 * Shadow OT Client - Container
 *
 * In-game container (backpacks, chests, etc.)
 */

#pragma once

#include "item.h"
#include <string>
#include <memory>
#include <vector>
#include <functional>

namespace shadow {
namespace client {

class Container : public std::enable_shared_from_this<Container> {
public:
    Container();

    // Identification
    uint8_t getId() const { return m_id; }
    void setId(uint8_t id) { m_id = id; }

    // Container item
    uint16_t getContainerItemId() const { return m_containerItemId; }
    void setContainerItemId(uint16_t id) { m_containerItemId = id; }

    // Name
    const std::string& getName() const { return m_name; }
    void setName(const std::string& name) { m_name = name; }

    // Capacity
    uint8_t getCapacity() const { return m_capacity; }
    void setCapacity(uint8_t capacity) { m_capacity = capacity; }

    // Contents
    void addItem(ItemPtr item);
    void insertItem(int slot, ItemPtr item);
    void removeItem(int slot);
    void updateItem(int slot, ItemPtr item);
    void clear();

    ItemPtr getItem(int slot) const;
    int getItemCount() const { return static_cast<int>(m_items.size()); }
    const std::vector<ItemPtr>& getItems() const { return m_items; }

    bool isFull() const { return m_items.size() >= m_capacity; }
    bool isEmpty() const { return m_items.empty(); }

    // Parent container
    bool hasParent() const { return m_hasParent; }
    void setHasParent(bool hasParent) { m_hasParent = hasParent; }

    // Unlocked (can be closed by player)
    bool isUnlocked() const { return m_unlocked; }
    void setUnlocked(bool unlocked) { m_unlocked = unlocked; }

    // Pagination (for large containers)
    bool hasPagination() const { return m_hasPagination; }
    void setPagination(bool pagination) { m_hasPagination = pagination; }

    int getCurrentPage() const { return m_currentPage; }
    void setCurrentPage(int page) { m_currentPage = page; }

    int getTotalPages() const { return m_totalPages; }
    void setTotalPages(int pages) { m_totalPages = pages; }

    int getItemsPerPage() const { return m_itemsPerPage; }
    void setItemsPerPage(int items) { m_itemsPerPage = items; }

    // First index (for pagination)
    int getFirstIndex() const { return m_firstIndex; }
    void setFirstIndex(int index) { m_firstIndex = index; }

    // Callbacks
    using ItemAddCallback = std::function<void(int slot, ItemPtr item)>;
    using ItemRemoveCallback = std::function<void(int slot)>;
    using ItemUpdateCallback = std::function<void(int slot, ItemPtr item)>;

    void setOnItemAdd(ItemAddCallback cb) { m_onItemAdd = cb; }
    void setOnItemRemove(ItemRemoveCallback cb) { m_onItemRemove = cb; }
    void setOnItemUpdate(ItemUpdateCallback cb) { m_onItemUpdate = cb; }

private:
    uint8_t m_id{0};
    uint16_t m_containerItemId{0};
    std::string m_name;
    uint8_t m_capacity{0};
    std::vector<ItemPtr> m_items;

    bool m_hasParent{false};
    bool m_unlocked{true};
    bool m_hasPagination{false};

    int m_currentPage{0};
    int m_totalPages{1};
    int m_itemsPerPage{36};
    int m_firstIndex{0};

    ItemAddCallback m_onItemAdd;
    ItemRemoveCallback m_onItemRemove;
    ItemUpdateCallback m_onItemUpdate;
};

using ContainerPtr = std::shared_ptr<Container>;

// Container manager
class ContainerManager {
public:
    static ContainerManager& instance();

    void clear();

    ContainerPtr createContainer(uint8_t id);
    void removeContainer(uint8_t id);

    ContainerPtr getContainer(uint8_t id) const;
    ContainerPtr getContainerByItemId(uint16_t itemId) const;

    int getContainerCount() const { return static_cast<int>(m_containers.size()); }
    const std::map<uint8_t, ContainerPtr>& getContainers() const { return m_containers; }

    // Callbacks
    using OpenCallback = std::function<void(ContainerPtr)>;
    using CloseCallback = std::function<void(uint8_t)>;

    void setOnOpen(OpenCallback cb) { m_onOpen = cb; }
    void setOnClose(CloseCallback cb) { m_onClose = cb; }

private:
    ContainerManager() = default;
    ContainerManager(const ContainerManager&) = delete;
    ContainerManager& operator=(const ContainerManager&) = delete;

    std::map<uint8_t, ContainerPtr> m_containers;

    OpenCallback m_onOpen;
    CloseCallback m_onClose;
};

} // namespace client
} // namespace shadow

extern shadow::client::ContainerManager& g_containers;
