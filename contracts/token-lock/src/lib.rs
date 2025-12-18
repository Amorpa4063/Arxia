//! Token locking contract for vesting and staking.

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// A token lock entry.
#[derive(Debug, Clone)]
pub struct TokenLock {
    /// Owner account.
    pub owner: String,
    /// Amount locked (micro-ARX).
    pub amount: u64,
    /// Unlock timestamp (unix ms).
    pub unlock_at: u64,
    /// Whether claimed.
    pub claimed: bool,
}

impl TokenLock {
    /// Create a new token lock.
    pub fn new(owner: String, amount: u64, unlock_at: u64) -> Self {
        Self {
            owner,
            amount,
            unlock_at,
            claimed: false,
        }
    }

    /// Attempt to claim the locked tokens.
    pub fn claim(&mut self, current_time: u64) -> Result<u64, &'static str> {
        if self.claimed {
            return Err("tokens already claimed");
        }
        if current_time < self.unlock_at {
            return Err("lock period has not elapsed");
        }
        self.claimed = true;
        Ok(self.amount)
    }

    /// Check if the lock has expired.
    pub fn is_unlocked(&self, current_time: u64) -> bool {
        current_time >= self.unlock_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_lock_claim() {
        let mut lock = TokenLock::new("alice".into(), 1_000_000, 1000);
        assert!(lock.claim(500).is_err());
        assert!(!lock.is_unlocked(500));
        let amount = lock.claim(1000).unwrap();
        assert_eq!(amount, 1_000_000);
        assert!(lock.claimed);
    }

    #[test]
    fn test_token_lock_double_claim() {
        let mut lock = TokenLock::new("alice".into(), 1_000_000, 1000);
        lock.claim(1000).unwrap();
        assert!(lock.claim(2000).is_err());
    }
}
