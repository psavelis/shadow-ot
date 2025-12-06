#include "realmmanager.h"
#include <algorithm>

namespace shadow {
namespace realms {

std::string RealmInfo::typeDisplayName() const {
    return realmTypeToString(type);
}

std::string RealmInfo::pvpDisplayName() const {
    return pvpTypeToString(pvpType);
}

std::string RealmInfo::statusDisplayName() const {
    switch (status) {
        case RealmStatus::Online: return "Online";
        case RealmStatus::Maintenance: return "Maintenance";
        case RealmStatus::Offline: return "Offline";
        case RealmStatus::Full: return "Full";
        default: return "Unknown";
    }
}

RealmManager& RealmManager::instance() {
    static RealmManager instance;
    return instance;
}

void RealmManager::refreshRealms(const RealmCallback& callback) {
    // TODO: Fetch from API
    // For now, return hardcoded realms

    m_realms = {
        {
            1,
            "Shadowveil",
            "A realm shrouded in eternal darkness. Dark creatures roam the lands.",
            RealmType::Dark,
            PvPType::Open,
            RealmStatus::Online,
            156,
            300,
            "shadowveil.shadow-ot.com",
            7172,
            1.0f,
            1.0f,
            false,
            "1.0.0"
        },
        {
            2,
            "Aetheria",
            "A mythical realm of ancient legends and powerful magic.",
            RealmType::Mythic,
            PvPType::Optional,
            RealmStatus::Online,
            243,
            300,
            "aetheria.shadow-ot.com",
            7172,
            1.5f,
            1.2f,
            true,
            "1.0.0"
        },
        {
            3,
            "Warbound",
            "A realm of constant warfare. Only the strongest survive.",
            RealmType::PvP,
            PvPType::Hardcore,
            RealmStatus::Online,
            89,
            200,
            "warbound.shadow-ot.com",
            7172,
            2.0f,
            1.5f,
            true,
            "1.0.0"
        },
        {
            4,
            "Eternal Legacy",
            "Experience the classic adventure as it was meant to be.",
            RealmType::Classic,
            PvPType::Open,
            RealmStatus::Online,
            312,
            300,
            "legacy.shadow-ot.com",
            7172,
            1.0f,
            1.0f,
            false,
            "1.0.0"
        }
    };

    callback(m_realms);
}

void RealmManager::selectRealm(uint32_t realmId) {
    m_currentRealm = getRealm(realmId);
}

void RealmManager::connectToRealm(uint32_t realmId, const ConnectCallback& callback) {
    RealmInfo* realm = getRealm(realmId);
    if (!realm) {
        callback(false, "Realm not found");
        return;
    }

    if (!realm->isAvailable()) {
        callback(false, "Realm is not available");
        return;
    }

    m_currentRealm = realm;

    // TODO: Initiate actual connection
    callback(true, "");
}

RealmInfo* RealmManager::getRealm(uint32_t id) {
    for (auto& realm : m_realms) {
        if (realm.id == id) {
            return &realm;
        }
    }
    return nullptr;
}

const RealmInfo* RealmManager::getRealm(uint32_t id) const {
    for (const auto& realm : m_realms) {
        if (realm.id == id) {
            return &realm;
        }
    }
    return nullptr;
}

std::vector<RealmInfo> RealmManager::filterByType(RealmType type) const {
    std::vector<RealmInfo> result;
    for (const auto& realm : m_realms) {
        if (realm.type == type) {
            result.push_back(realm);
        }
    }
    return result;
}

std::vector<RealmInfo> RealmManager::filterByPvP(PvPType pvpType) const {
    std::vector<RealmInfo> result;
    for (const auto& realm : m_realms) {
        if (realm.pvpType == pvpType) {
            result.push_back(realm);
        }
    }
    return result;
}

std::vector<RealmInfo> RealmManager::filterAvailable() const {
    std::vector<RealmInfo> result;
    for (const auto& realm : m_realms) {
        if (realm.isAvailable()) {
            result.push_back(realm);
        }
    }
    return result;
}

void RealmManager::sortByPlayers(bool descending) {
    std::sort(m_realms.begin(), m_realms.end(),
        [descending](const RealmInfo& a, const RealmInfo& b) {
            return descending ? a.playersOnline > b.playersOnline
                             : a.playersOnline < b.playersOnline;
        });
}

void RealmManager::sortByName() {
    std::sort(m_realms.begin(), m_realms.end(),
        [](const RealmInfo& a, const RealmInfo& b) {
            return a.name < b.name;
        });
}

void RealmManager::sortByExpRate(bool descending) {
    std::sort(m_realms.begin(), m_realms.end(),
        [descending](const RealmInfo& a, const RealmInfo& b) {
            return descending ? a.expRate > b.expRate : a.expRate < b.expRate;
        });
}

RealmManager::ThemeColors RealmManager::getRealmTheme(RealmType type) const {
    switch (type) {
        case RealmType::Dark:
            return {
                0x1a0a2e,   // Primary - Deep purple
                0x2d1b4e,   // Secondary - Dark violet
                0x9b59b6,   // Accent - Purple
                0x0d0d0d,   // Background - Near black
                0xe0e0e0    // Text - Light gray
            };
        case RealmType::Mythic:
            return {
                0x1e3a5f,   // Primary - Deep blue
                0x2c5282,   // Secondary - Ocean blue
                0x3498db,   // Accent - Sky blue
                0x0a1929,   // Background - Dark blue
                0xf0f8ff    // Text - Alice blue
            };
        case RealmType::PvP:
            return {
                0x4a0e0e,   // Primary - Dark red
                0x6b1a1a,   // Secondary - Blood red
                0xe74c3c,   // Accent - Bright red
                0x1a0505,   // Background - Very dark red
                0xffd0d0    // Text - Light red
            };
        case RealmType::Classic:
            return {
                0x2d2d2d,   // Primary - Dark gray
                0x404040,   // Secondary - Medium gray
                0xc0a060,   // Accent - Gold
                0x1a1a1a,   // Background - Near black
                0xd4d4d4    // Text - Light gray
            };
        default:
            return {
                0x2d2d2d,
                0x404040,
                0x3498db,
                0x1a1a1a,
                0xd4d4d4
            };
    }
}

std::string realmTypeToString(RealmType type) {
    switch (type) {
        case RealmType::Dark: return "Dark Fantasy";
        case RealmType::Mythic: return "Mythic";
        case RealmType::PvP: return "PvP Arena";
        case RealmType::Classic: return "Classic";
        default: return "Unknown";
    }
}

RealmType stringToRealmType(const std::string& str) {
    if (str == "dark" || str == "Dark Fantasy") return RealmType::Dark;
    if (str == "mythic" || str == "Mythic") return RealmType::Mythic;
    if (str == "pvp" || str == "PvP Arena") return RealmType::PvP;
    if (str == "classic" || str == "Classic") return RealmType::Classic;
    return RealmType::Classic;
}

std::string pvpTypeToString(PvPType type) {
    switch (type) {
        case PvPType::Open: return "Open PvP";
        case PvPType::Optional: return "Optional PvP";
        case PvPType::Hardcore: return "Hardcore PvP";
        case PvPType::NonPvP: return "Non-PvP";
        default: return "Unknown";
    }
}

PvPType stringToPvPType(const std::string& str) {
    if (str == "open" || str == "Open PvP") return PvPType::Open;
    if (str == "optional" || str == "Optional PvP") return PvPType::Optional;
    if (str == "hardcore" || str == "Hardcore PvP") return PvPType::Hardcore;
    if (str == "nonpvp" || str == "Non-PvP") return PvPType::NonPvP;
    return PvPType::Open;
}

} // namespace realms
} // namespace shadow
