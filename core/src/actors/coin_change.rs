/*
   Appellation: coin_change
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CoinChange {
    pub amount: usize,
    pub coins: Vec<usize>,
}

impl CoinChange {
    pub fn new(amount: usize, coins: Vec<usize>) -> Self {
        Self { amount, coins }
    }
    pub fn change(&mut self) -> Option<usize> {
        let mut dp = vec![std::usize::MAX; self.amount + 1];
        dp[0] = 0;

        for i in 0..=self.amount {
            for j in 0..self.coins.len() {
                if i >= self.coins[j] && dp[i - self.coins[j]] != std::usize::MAX {
                    dp[i] = dp[i].min(dp[i - self.coins[j]] + 1);
                }
            }
        }

        match dp[self.amount] {
            std::usize::MAX => None,
            _ => Some(dp[self.amount]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 11 = 5 * 2 + 1 * 1
        assert_eq!(Some(3), CoinChange::new(11, vec![1, 2, 5]).change(),);
        // 119 = 11 * 10 + 7 * 1 + 2 * 1
        assert_eq!(
            Some(12),
            CoinChange::new(119, vec![2, 3, 5, 7, 11]).change()
        )
    }

    #[test]
    fn coins_empty() {
        assert_eq!(None, CoinChange::new(1, Vec::<usize>::new()).change());
    }

    #[test]
    fn amount_zero() {
        assert_eq!(Some(0), CoinChange::new(0, vec![1, 2, 3]).change());
    }

    #[test]
    fn fail_change() {
        assert_eq!(None, CoinChange::new(3, vec![2]).change());
        assert_eq!(None, CoinChange::new(5, vec![10, 20, 50, 100]).change());
    }
}
