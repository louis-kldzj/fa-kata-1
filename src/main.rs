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
            c_change -= coin;
            coins_to_return[idx] += 1;
        }
        if coin == 0 {
            return coins_to_return;
        }
    }
    coins_to_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(calculate_change(15), vec![0, 0, 1, 1, 0, 0])
    }
}
