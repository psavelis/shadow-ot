#ifndef SHADOW_BLOCKCHAIN_WALLET_H
#define SHADOW_BLOCKCHAIN_WALLET_H

#include <string>
#include <vector>
#include <memory>
#include <functional>
#include <unordered_map>

namespace shadow {
namespace blockchain {

/// Supported blockchain networks
enum class Network {
    Starknet,
    Ethereum,
    Polygon,
    Bitcoin,
    Spark
};

/// Wallet connection status
enum class WalletStatus {
    Disconnected,
    Connecting,
    Connected,
    Error
};

/// NFT asset representation
struct NFTAsset {
    std::string tokenId;
    std::string contractAddress;
    Network network;
    std::string name;
    std::string description;
    std::string imageUrl;
    std::string rarity;
    std::unordered_map<std::string, std::string> attributes;
    uint32_t inGameItemId;  // Mapped to game item
    bool isEquipped;
};

/// Wallet balance
struct Balance {
    Network network;
    std::string symbol;
    std::string amount;
    std::string usdValue;
};

/// Transaction info
struct Transaction {
    std::string hash;
    Network network;
    std::string from;
    std::string to;
    std::string amount;
    std::string status;
    uint64_t timestamp;
    uint32_t confirmations;
};

/// Wallet connection callback types
using ConnectionCallback = std::function<void(WalletStatus, const std::string&)>;
using NFTCallback = std::function<void(const std::vector<NFTAsset>&)>;
using BalanceCallback = std::function<void(const std::vector<Balance>&)>;
using TransactionCallback = std::function<void(const Transaction&)>;

/**
 * @brief Wallet manager for blockchain integration
 *
 * Handles wallet connections, NFT loading, and transaction monitoring
 * for the Shadow OT client.
 */
class Wallet {
public:
    static Wallet& instance();

    /// Connection management
    void connect(Network network, const ConnectionCallback& callback);
    void disconnect();
    bool isConnected() const { return m_status == WalletStatus::Connected; }
    WalletStatus status() const { return m_status; }

    /// Address management
    std::string address() const { return m_address; }
    Network currentNetwork() const { return m_network; }

    /// NFT operations
    void loadNFTs(const NFTCallback& callback);
    const std::vector<NFTAsset>& nfts() const { return m_nfts; }
    NFTAsset* findNFT(const std::string& tokenId);
    bool equipNFT(const std::string& tokenId);
    bool unequipNFT(const std::string& tokenId);

    /// Balance operations
    void loadBalances(const BalanceCallback& callback);
    const std::vector<Balance>& balances() const { return m_balances; }

    /// Transaction operations
    void sendTransaction(const std::string& to, const std::string& amount,
                        const TransactionCallback& callback);
    void watchTransaction(const std::string& hash, const TransactionCallback& callback);

    /// NFT marketplace
    void listNFTForSale(const std::string& tokenId, const std::string& price,
                       const TransactionCallback& callback);
    void buyNFT(const std::string& contractAddress, const std::string& tokenId,
               const TransactionCallback& callback);

    /// Signature verification (for login/auth)
    void signMessage(const std::string& message,
                    const std::function<void(const std::string&)>& callback);
    bool verifySignature(const std::string& message, const std::string& signature,
                        const std::string& address);

private:
    Wallet() = default;
    ~Wallet() = default;
    Wallet(const Wallet&) = delete;
    Wallet& operator=(const Wallet&) = delete;

    void processConnectResult(bool success, const std::string& address,
                             const std::string& error);

    WalletStatus m_status = WalletStatus::Disconnected;
    Network m_network = Network::Starknet;
    std::string m_address;
    std::vector<NFTAsset> m_nfts;
    std::vector<Balance> m_balances;
    std::unordered_map<std::string, TransactionCallback> m_pendingTransactions;
    ConnectionCallback m_connectionCallback;
};

/// Network utility functions
std::string networkToString(Network network);
Network stringToNetwork(const std::string& str);
std::string formatAddress(const std::string& address, int prefixLen = 6, int suffixLen = 4);

} // namespace blockchain
} // namespace shadow

#endif // SHADOW_BLOCKCHAIN_WALLET_H
