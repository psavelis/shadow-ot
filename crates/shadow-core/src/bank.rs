//! Bank System
//!
//! Handles player banking operations including deposits, withdrawals,
//! transfers, and bank account management. Similar to Tibia's bank
//! system with additional features.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Bank account ID
pub type BankAccountId = Uuid;
/// Transaction ID
pub type TransactionId = Uuid;

/// Bank account for a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    /// Account ID
    pub id: BankAccountId,
    /// Character ID that owns this account
    pub character_id: Uuid,
    /// Current balance in gold coins
    pub balance: u64,
    /// Total deposited over lifetime
    pub total_deposited: u64,
    /// Total withdrawn over lifetime
    pub total_withdrawn: u64,
    /// When the account was opened
    pub opened_at: DateTime<Utc>,
    /// Last transaction time
    pub last_transaction: Option<DateTime<Utc>>,
    /// Account status
    pub status: AccountStatus,
    /// Daily withdrawal limit remaining
    pub daily_withdrawal_remaining: u64,
    /// Last withdrawal reset time
    pub withdrawal_reset_at: DateTime<Utc>,
}

impl BankAccount {
    /// Create a new bank account for a character
    pub fn new(character_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            character_id,
            balance: 0,
            total_deposited: 0,
            total_withdrawn: 0,
            opened_at: now,
            last_transaction: None,
            status: AccountStatus::Active,
            daily_withdrawal_remaining: Self::default_daily_limit(),
            withdrawal_reset_at: now,
        }
    }

    /// Default daily withdrawal limit
    fn default_daily_limit() -> u64 {
        100_000_000 // 100M gold
    }

    /// Check if account is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, AccountStatus::Active)
    }

    /// Deposit gold into the account
    pub fn deposit(&mut self, amount: u64) -> Result<(), BankError> {
        if !self.is_active() {
            return Err(BankError::AccountInactive);
        }

        self.balance = self.balance.checked_add(amount)
            .ok_or(BankError::BalanceOverflow)?;
        self.total_deposited += amount;
        self.last_transaction = Some(Utc::now());

        Ok(())
    }

    /// Withdraw gold from the account
    pub fn withdraw(&mut self, amount: u64) -> Result<(), BankError> {
        if !self.is_active() {
            return Err(BankError::AccountInactive);
        }

        if amount > self.balance {
            return Err(BankError::InsufficientFunds);
        }

        // Check daily limit
        self.check_and_reset_daily_limit();
        if amount > self.daily_withdrawal_remaining {
            return Err(BankError::DailyLimitExceeded);
        }

        self.balance -= amount;
        self.total_withdrawn += amount;
        self.daily_withdrawal_remaining -= amount;
        self.last_transaction = Some(Utc::now());

        Ok(())
    }

    /// Check and reset daily withdrawal limit if needed
    fn check_and_reset_daily_limit(&mut self) {
        let now = Utc::now();
        let hours_since_reset = (now - self.withdrawal_reset_at).num_hours();
        
        if hours_since_reset >= 24 {
            self.daily_withdrawal_remaining = Self::default_daily_limit();
            self.withdrawal_reset_at = now;
        }
    }

    /// Check balance without modification
    pub fn check_balance(&self) -> u64 {
        self.balance
    }

    /// Get account statement summary
    pub fn get_statement(&self) -> AccountStatement {
        AccountStatement {
            account_id: self.id,
            balance: self.balance,
            total_deposited: self.total_deposited,
            total_withdrawn: self.total_withdrawn,
            opened_at: self.opened_at,
            last_transaction: self.last_transaction,
        }
    }
}

/// Account status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

/// Account statement summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatement {
    pub account_id: BankAccountId,
    pub balance: u64,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub opened_at: DateTime<Utc>,
    pub last_transaction: Option<DateTime<Utc>>,
}

/// Transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    TransferIn,
    TransferOut,
    GuildDeposit,
    GuildWithdrawal,
    HousePayment,
    MarketPurchase,
    MarketSale,
    Interest,
    Fee,
}

/// Bank transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID
    pub id: TransactionId,
    /// Source account (None for deposits from inventory)
    pub from_account: Option<BankAccountId>,
    /// Destination account (None for withdrawals to inventory)
    pub to_account: Option<BankAccountId>,
    /// Amount transferred
    pub amount: u64,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Description/memo
    pub description: String,
    /// When transaction occurred
    pub timestamp: DateTime<Utc>,
    /// Was successful
    pub success: bool,
}

impl Transaction {
    /// Create a new transaction record
    pub fn new(
        from: Option<BankAccountId>,
        to: Option<BankAccountId>,
        amount: u64,
        transaction_type: TransactionType,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_account: from,
            to_account: to,
            amount,
            transaction_type,
            description: description.into(),
            timestamp: Utc::now(),
            success: true,
        }
    }
}

/// Guild bank account with additional features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildBank {
    /// Guild ID
    pub guild_id: Uuid,
    /// Current balance
    pub balance: u64,
    /// Daily withdraw limit per member
    pub member_daily_limit: u64,
    /// Member withdrawal tracking (member_id -> withdrawn_today)
    pub member_withdrawals: HashMap<Uuid, u64>,
    /// Last reset time
    pub last_reset: DateTime<Utc>,
    /// Total deposits ever
    pub total_deposited: u64,
    /// Withdrawal permissions (rank -> can_withdraw)
    pub withdrawal_permissions: HashMap<String, bool>,
}

impl GuildBank {
    /// Create a new guild bank
    pub fn new(guild_id: Uuid) -> Self {
        Self {
            guild_id,
            balance: 0,
            member_daily_limit: 10_000_000, // 10M per member per day
            member_withdrawals: HashMap::new(),
            last_reset: Utc::now(),
            total_deposited: 0,
            withdrawal_permissions: HashMap::new(),
        }
    }

    /// Deposit into guild bank
    pub fn deposit(&mut self, _member_id: Uuid, amount: u64) -> Result<(), BankError> {
        self.balance = self.balance.checked_add(amount)
            .ok_or(BankError::BalanceOverflow)?;
        self.total_deposited += amount;
        Ok(())
    }

    /// Withdraw from guild bank
    pub fn withdraw(&mut self, member_id: Uuid, amount: u64, rank: &str) -> Result<(), BankError> {
        // Check permission
        if !self.can_withdraw(rank) {
            return Err(BankError::NoPermission);
        }

        // Check balance
        if amount > self.balance {
            return Err(BankError::InsufficientFunds);
        }

        // Check member daily limit
        self.check_and_reset_limits();
        let withdrawn_today = self.member_withdrawals.get(&member_id).copied().unwrap_or(0);
        if withdrawn_today + amount > self.member_daily_limit {
            return Err(BankError::DailyLimitExceeded);
        }

        self.balance -= amount;
        *self.member_withdrawals.entry(member_id).or_insert(0) += amount;

        Ok(())
    }

    /// Check if rank can withdraw
    fn can_withdraw(&self, rank: &str) -> bool {
        self.withdrawal_permissions.get(rank).copied().unwrap_or(false)
    }

    /// Reset daily limits if needed
    fn check_and_reset_limits(&mut self) {
        let now = Utc::now();
        if (now - self.last_reset).num_hours() >= 24 {
            self.member_withdrawals.clear();
            self.last_reset = now;
        }
    }

    /// Set withdrawal permission for a rank
    pub fn set_permission(&mut self, rank: String, can_withdraw: bool) {
        self.withdrawal_permissions.insert(rank, can_withdraw);
    }
}

/// Bank manager handles all banking operations
pub struct BankManager {
    /// Character bank accounts
    accounts: HashMap<Uuid, BankAccount>,
    /// Guild bank accounts
    guild_banks: HashMap<Uuid, GuildBank>,
    /// Transaction history (limited to recent)
    transactions: Vec<Transaction>,
    /// Max transaction history size
    max_history: usize,
}

impl BankManager {
    /// Create a new bank manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            guild_banks: HashMap::new(),
            transactions: Vec::new(),
            max_history: 10000,
        }
    }

    /// Get or create a bank account for a character
    pub fn get_account(&mut self, character_id: Uuid) -> &BankAccount {
        self.accounts.entry(character_id)
            .or_insert_with(|| BankAccount::new(character_id))
    }

    /// Get mutable account
    pub fn get_account_mut(&mut self, character_id: Uuid) -> &mut BankAccount {
        self.accounts.entry(character_id)
            .or_insert_with(|| BankAccount::new(character_id))
    }

    /// Deposit gold for a character
    pub fn deposit(
        &mut self,
        character_id: Uuid,
        amount: u64,
    ) -> Result<Transaction, BankError> {
        let account = self.get_account_mut(character_id);
        account.deposit(amount)?;

        let transaction = Transaction::new(
            None,
            Some(account.id),
            amount,
            TransactionType::Deposit,
            format!("Deposit of {} gold", amount),
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Withdraw gold for a character
    pub fn withdraw(
        &mut self,
        character_id: Uuid,
        amount: u64,
    ) -> Result<Transaction, BankError> {
        let account = self.get_account_mut(character_id);
        account.withdraw(amount)?;

        let transaction = Transaction::new(
            Some(account.id),
            None,
            amount,
            TransactionType::Withdrawal,
            format!("Withdrawal of {} gold", amount),
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Transfer gold between characters
    pub fn transfer(
        &mut self,
        from_character: Uuid,
        to_character: Uuid,
        amount: u64,
    ) -> Result<Transaction, BankError> {
        // Withdraw from source and capture its id
        let from_account_id = {
            let from_account = self.get_account_mut(from_character);
            from_account.withdraw(amount)?;
            from_account.id
        };

        // Deposit to destination and capture its id
        let to_account_id = {
            let to_account = self.get_account_mut(to_character);
            to_account.deposit(amount)?;
            to_account.id
        };

        let transaction = Transaction::new(
            Some(from_account_id),
            Some(to_account_id),
            amount,
            TransactionType::TransferOut,
            format!("Transfer of {} gold", amount),
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Get balance for a character
    pub fn get_balance(&mut self, character_id: Uuid) -> u64 {
        self.get_account(character_id).balance
    }

    /// Check if character can afford an amount
    pub fn can_afford(&mut self, character_id: Uuid, amount: u64) -> bool {
        self.get_account(character_id).balance >= amount
    }

    /// Deduct gold for a purchase (house, market, etc.)
    pub fn deduct_for_purchase(
        &mut self,
        character_id: Uuid,
        amount: u64,
        transaction_type: TransactionType,
        description: &str,
    ) -> Result<Transaction, BankError> {
        let account = self.get_account_mut(character_id);
        
        if amount > account.balance {
            return Err(BankError::InsufficientFunds);
        }

        account.balance -= amount;
        account.total_withdrawn += amount;
        account.last_transaction = Some(Utc::now());

        let transaction = Transaction::new(
            Some(account.id),
            None,
            amount,
            transaction_type,
            description,
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Credit gold for a sale
    pub fn credit_for_sale(
        &mut self,
        character_id: Uuid,
        amount: u64,
        transaction_type: TransactionType,
        description: &str,
    ) -> Result<Transaction, BankError> {
        let account = self.get_account_mut(character_id);
        account.deposit(amount)?;

        let transaction = Transaction::new(
            None,
            Some(account.id),
            amount,
            transaction_type,
            description,
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Get transaction history for a character
    pub fn get_history(&self, character_id: Uuid, limit: usize) -> Vec<&Transaction> {
        let account_id = match self.accounts.get(&character_id) {
            Some(acc) => acc.id,
            None => return Vec::new(),
        };

        self.transactions.iter()
            .filter(|t| t.from_account == Some(account_id) || t.to_account == Some(account_id))
            .take(limit)
            .collect()
    }

    /// Record a transaction
    fn record_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        
        // Trim history if too large
        if self.transactions.len() > self.max_history {
            let to_remove = self.transactions.len() - self.max_history;
            self.transactions.drain(0..to_remove);
        }
    }

    // Guild bank operations

    /// Get or create guild bank
    pub fn get_guild_bank(&mut self, guild_id: Uuid) -> &GuildBank {
        self.guild_banks.entry(guild_id)
            .or_insert_with(|| GuildBank::new(guild_id))
    }

    /// Get mutable guild bank
    pub fn get_guild_bank_mut(&mut self, guild_id: Uuid) -> &mut GuildBank {
        self.guild_banks.entry(guild_id)
            .or_insert_with(|| GuildBank::new(guild_id))
    }

    /// Deposit to guild bank
    pub fn guild_deposit(
        &mut self,
        guild_id: Uuid,
        member_id: Uuid,
        amount: u64,
    ) -> Result<Transaction, BankError> {
        // Deduct from member's personal account
        {
            let account = self.get_account_mut(member_id);
            if account.balance < amount {
                return Err(BankError::InsufficientFunds);
            }
            account.balance -= amount;
        }

        // Add to guild bank
        let guild_bank = self.get_guild_bank_mut(guild_id);
        guild_bank.deposit(member_id, amount)?;

        let transaction = Transaction::new(
            self.accounts.get(&member_id).map(|a| a.id),
            None,
            amount,
            TransactionType::GuildDeposit,
            format!("Guild deposit of {} gold", amount),
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Withdraw from guild bank
    pub fn guild_withdraw(
        &mut self,
        guild_id: Uuid,
        member_id: Uuid,
        amount: u64,
        rank: &str,
    ) -> Result<Transaction, BankError> {
        // Withdraw from guild bank
        {
            let guild_bank = self.get_guild_bank_mut(guild_id);
            guild_bank.withdraw(member_id, amount, rank)?;
        }

        // Add to member's personal account
        let account = self.get_account_mut(member_id);
        account.deposit(amount)?;

        let transaction = Transaction::new(
            None,
            Some(account.id),
            amount,
            TransactionType::GuildWithdrawal,
            format!("Guild withdrawal of {} gold", amount),
        );

        self.record_transaction(transaction.clone());
        Ok(transaction)
    }

    /// Get guild bank balance
    pub fn get_guild_balance(&mut self, guild_id: Uuid) -> u64 {
        self.get_guild_bank(guild_id).balance
    }
}

impl Default for BankManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Bank errors
#[derive(Debug, Clone)]
pub enum BankError {
    AccountNotFound,
    AccountInactive,
    InsufficientFunds,
    DailyLimitExceeded,
    BalanceOverflow,
    NoPermission,
    TransferToSelf,
    InvalidAmount,
    DatabaseError(String),
}

impl std::fmt::Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankError::AccountNotFound => write!(f, "Bank account not found"),
            BankError::AccountInactive => write!(f, "Bank account is not active"),
            BankError::InsufficientFunds => write!(f, "Insufficient funds"),
            BankError::DailyLimitExceeded => write!(f, "Daily withdrawal limit exceeded"),
            BankError::BalanceOverflow => write!(f, "Balance would overflow"),
            BankError::NoPermission => write!(f, "No permission to perform this action"),
            BankError::TransferToSelf => write!(f, "Cannot transfer to yourself"),
            BankError::InvalidAmount => write!(f, "Invalid amount"),
            BankError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for BankError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_withdraw() {
        let mut account = BankAccount::new(Uuid::new_v4());
        
        account.deposit(1000).unwrap();
        assert_eq!(account.balance, 1000);
        
        account.withdraw(500).unwrap();
        assert_eq!(account.balance, 500);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut account = BankAccount::new(Uuid::new_v4());
        account.deposit(100).unwrap();
        
        let result = account.withdraw(200);
        assert!(matches!(result, Err(BankError::InsufficientFunds)));
    }

    #[test]
    fn test_bank_manager_transfer() {
        let mut manager = BankManager::new();
        let char1 = Uuid::new_v4();
        let char2 = Uuid::new_v4();
        
        manager.deposit(char1, 1000).unwrap();
        manager.transfer(char1, char2, 300).unwrap();
        
        assert_eq!(manager.get_balance(char1), 700);
        assert_eq!(manager.get_balance(char2), 300);
    }
}
