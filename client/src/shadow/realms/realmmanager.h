#ifndef SHADOW_REALMS_REALMMANAGER_H
#define SHADOW_REALMS_REALMMANAGER_H

#include <string>
#include <vector>
#include <memory>
#include <functional>

namespace shadow {
namespace realms {

/// Realm type/theme
enum class RealmType {
    Dark,       // Shadowveil - Dark fantasy theme
    Mythic,     // Aetheria - High fantasy theme
    PvP,        // Warbound - PvP focused
    Classic     // Eternal Legacy - Classic Tibia
};

/// PvP type for the realm
enum class PvPType {
    Open,       // Standard open PvP
    Optional,   // Optional PvP zones
    Hardcore,   // Full loot PvP
    NonPvP      // No PvP
};

/// Realm status
enum class RealmStatus {
    Online,
    Maintenance,
    Offline,
    Full
};

/// Realm information
struct RealmInfo {
    uint32_t id;
    std::string name;
    std::string description;
    RealmType type;
    PvPType pvpType;
    RealmStatus status;
    uint32_t playersOnline;
    uint32_t maxPlayers;
    std::string ip;
    uint16_t port;
    float expRate;
    float lootRate;
    bool isPremium;
    std::string version;

    /// Get realm type display name
    std::string typeDisplayName() const;

    /// Get PvP type display name
    std::string pvpDisplayName() const;

    /// Get status display name
    std::string statusDisplayName() const;

    /// Calculate load percentage
    float loadPercent() const {
        return maxPlayers > 0 ? static_cast<float>(playersOnline) / maxPlayers * 100.0f : 0.0f;
    }

    /// Check if realm is available for connection
    bool isAvailable() const {
        return status == RealmStatus::Online && playersOnline < maxPlayers;
    }
};

/// Realm selection callback
using RealmCallback = std::function<void(const std::vector<RealmInfo>&)>;
using ConnectCallback = std::function<void(bool success, const std::string& error)>;

/**
 * @brief Realm manager for multi-realm support
 *
 * Handles realm listing, selection, and switching in Shadow OT.
 */
class RealmManager {
public:
    static RealmManager& instance();

    /// Realm list management
    void refreshRealms(const RealmCallback& callback);
    const std::vector<RealmInfo>& realms() const { return m_realms; }
    RealmInfo* currentRealm() { return m_currentRealm; }
    const RealmInfo* currentRealm() const { return m_currentRealm; }

    /// Realm selection
    void selectRealm(uint32_t realmId);
    void connectToRealm(uint32_t realmId, const ConnectCallback& callback);

    /// Get realm by ID
    RealmInfo* getRealm(uint32_t id);
    const RealmInfo* getRealm(uint32_t id) const;

    /// Filtering
    std::vector<RealmInfo> filterByType(RealmType type) const;
    std::vector<RealmInfo> filterByPvP(PvPType pvpType) const;
    std::vector<RealmInfo> filterAvailable() const;

    /// Sorting
    void sortByPlayers(bool descending = true);
    void sortByName();
    void sortByExpRate(bool descending = true);

    /// Get realm theme colors for UI
    struct ThemeColors {
        uint32_t primary;
        uint32_t secondary;
        uint32_t accent;
        uint32_t background;
        uint32_t text;
    };
    ThemeColors getRealmTheme(RealmType type) const;

private:
    RealmManager() = default;
    ~RealmManager() = default;
    RealmManager(const RealmManager&) = delete;
    RealmManager& operator=(const RealmManager&) = delete;

    std::vector<RealmInfo> m_realms;
    RealmInfo* m_currentRealm = nullptr;
};

/// Utility functions
std::string realmTypeToString(RealmType type);
RealmType stringToRealmType(const std::string& str);
std::string pvpTypeToString(PvPType type);
PvPType stringToPvPType(const std::string& str);

} // namespace realms
} // namespace shadow

#endif // SHADOW_REALMS_REALMMANAGER_H
