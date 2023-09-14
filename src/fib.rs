use std::collections::HashMap;

pub fn trib_non_recursive(n: u32) -> usize {
    if n == 0 || n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    let mut hash = HashMap::with_capacity(n as usize);
    for i in 0..n {
        match i {
            0 => hash.insert(i, 0),
            1 => hash.insert(i, 0),
            2 => hash.insert(i, 1),
            _ => {
                let prev_1 = hash.get(&(i - 1)).unwrap_or(&0);
                let prev_2 = hash.get(&(i - 2)).unwrap_or(&0);
                let prev_3 = hash.get(&(i - 3)).unwrap_or(&0);
                hash.insert(i, prev_1 + prev_2 + prev_3)
            }
        };
    }
    hash.get(&(n - 3)).unwrap_or(&0) +
    hash.get(&(n - 2)).unwrap_or(&0) +
    hash.get(&(n - 1)).unwrap_or(&0)
}


pub fn fib_non_recursive(n: u32) -> usize {
    if n == 0 || n == 1 {
        return n as usize;
    }
    let mut hash = HashMap::with_capacity(n as usize);
    for i in 0..n {
        match i {
            0 => hash.insert(i, i as usize),
            1 => hash.insert(i, i as usize),
            _ => {
                let prev_1 = hash.get(&(i - 1)).unwrap_or(&0);
                let prev_2 = hash.get(&(i - 2)).unwrap_or(&0);
                hash.insert(i, prev_1 + prev_2)
            }
        };
    }
    hash.get(&(n - 2)).unwrap_or(&0) + hash.get(&(n- 1)).unwrap_or(&0)
}

pub fn fib_recursive(n: u32) -> usize {
    _fib_recursive(&n, &mut HashMap::with_capacity(n as usize))
}

fn _fib_recursive<'a, 'b>(n: &'a u32, memo: &'b mut HashMap<u32, usize>) -> usize {
    if *n == 0 || *n == 1 {
        memo.insert(*n, *n as usize);
        return *n as usize;
    }
    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap_or(&0);
    }
    let res = _fib_recursive(&(n - 1), memo) + _fib_recursive(&(n - 2), memo);
    memo.insert(*n, res);
    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn fib_non_recursive_1() {
        assert_eq!(fib_non_recursive(12), 144);
        assert_eq!(fib_non_recursive(90), 2880067194370816120);
    }
    #[test]
    fn fib_recursive_1() {
        assert_eq!(fib_recursive(12), 144);
        assert_eq!(fib_recursive(90), 2880067194370816120);
    }
    #[test]
    fn trib_non_recursive_1() {
        assert_eq!(trib_non_recursive(10), 81);
        assert_eq!(trib_non_recursive(8), 24);
    }
}