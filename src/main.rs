fn main() {
    println!("Hello, world!");
}

const COINS: [usize; 6] = [1, 2, 5, 10, 20, 50];

fn calculate_change(change: usize) -> Vec<usize> {
    // TODO
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(calculate_change(15), vec![])
    }
}
