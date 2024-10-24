use std::usize;

fn main() {
    println!("Hello, world!");
}

const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn calculate_change(change: usize) -> Vec<usize> {
    calculate_change_with_coins(change, COINS.to_vec())
}

fn calculate_change_with_coins(change: usize, coins: Vec<usize>) -> Vec<usize> {
    let mut coins = coins.into_iter().enumerate().collect::<Vec<_>>();
    coins.sort_by_key(|(_, c)| *c);
    coins.reverse();
    let mut current_change = change;
    let mut coins_to_return: Vec<usize> = vec![0; coins.len()];
    for (idx, coin) in coins {
        if coin <= current_change {
            let coin_count: usize = current_change / coin;

            current_change -= coin * coin_count;
            coins_to_return[idx] += coin_count;
        }
        if current_change == 0 {
            return coins_to_return;
        }
    }
    coins_to_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_15() {
        assert_eq!(calculate_change(15), vec![0, 0, 1, 1, 0, 0, 0, 0])
    }

    #[test]
    fn works_with_45() {
        assert_eq!(calculate_change(45), [0, 0, 1, 0, 2, 0, 0, 0])
    }

    #[test]
    fn wills_crazy_test_case() {
        assert_eq!(
            calculate_change(314159265),
            vec![0, 0, 1, 1, 0, 1, 0, 1570796]
        )
    }

    #[test]
    fn will_2() {
        assert_eq!(
            calculate_change_with_coins(45, vec![5, 1, 2, 10, 20, 50]),
            [1, 0, 0, 0, 2, 0]
        )
    }
}
