fn main() {
    println!("Hello, world!");
}

const COINS: [usize; 6] = [1, 2, 5, 10, 20, 50];

fn calculate_change(change: usize) -> Vec<usize> {
    // TODO
    let mut coins = COINS;
    coins.reverse();
    let mut c_change = change;
    let mut coins_to_return: Vec<usize> = vec![0; 6];
    for (idx, coin) in coins.into_iter().enumerate() {
        if coin <= c_change {
            let coin_count: usize = c_change / coin;

            c_change -= coin * coin_count;
            coins_to_return[idx] += coin_count;
        }
        if c_change == 0 {
            coins_to_return.reverse();
            return coins_to_return;
        }
    }
    coins_to_return.reverse();
    coins_to_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_15() {
        assert_eq!(calculate_change(15), vec![0, 0, 1, 1, 0, 0])
    }

    #[test]
    fn works_with_45() {
        assert_eq!(calculate_change(45), [0, 0, 1, 0, 2, 0])
    }
}
