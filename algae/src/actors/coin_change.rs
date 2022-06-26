/*
   Appellation: coin_change
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![std::usize::MAX; amount + 1];
    dp[0] = 0;

    for i in 0..=amount {
        for j in 0..coins.len() {
            if i >= coins[j] && dp[i - coins[j]] != std::usize::MAX {
                dp[i] = dp[i].min(dp[i - coins[j]] + 1);
            }
        }
    }

    match dp[amount] {
        std::usize::MAX => None,
        _ => Some(dp[amount]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 11 = 5 * 2 + 1 * 1
        let coins = vec![1, 2, 5];
        assert_eq!(Some(3), coin_change(&coins, 11));

        // 119 = 11 * 10 + 7 * 1 + 2 * 1
        let coins = vec![2, 3, 5, 7, 11];
        assert_eq!(Some(12), coin_change(&coins, 119));
    }

    #[test]
    fn coins_empty() {
        let coins = vec![];
        assert_eq!(None, coin_change(&coins, 1));
    }

    #[test]
    fn amount_zero() {
        let coins = vec![1, 2, 3];
        assert_eq!(Some(0), coin_change(&coins, 0));
    }

    #[test]
    fn fail_change() {
        // 3 can't be change by 2.
        let coins = vec![2];
        assert_eq!(None, coin_change(&coins, 3));
        let coins = vec![10, 20, 50, 100];
        assert_eq!(None, coin_change(&coins, 5));
    }
}
