use std::collections::HashMap;

pub fn possibility_min(n: i32, arry: &mut [i32]) -> u32 {
    _possibility_min(&n, arry, &mut HashMap::with_capacity(n as usize))
}

fn _possibility_min<'a>(n: &i32, arry: &mut [i32], memo: &'a mut HashMap<i32, bool>) -> u32 {
    arry.sort();
    println!("{:?}", arry);
    for i in arry.into_iter().rev() {
        println!("{}", i);
    }
    0
}

pub fn possibility(n: i32, arry: &[i32]) -> bool {
    _possibility(&n, arry, &mut HashMap::with_capacity(n as usize))
}

fn _possibility<'a>(n: &i32, arry: &[i32], memo: &'a mut HashMap<i32, bool>) -> bool {
    if n < &0 {
        return false;
    }
    if n == &0 {
        return true;
    }
    if memo.contains_key(n) {
        return false;
    }
    for i in arry {
        if i == &0 {
            continue;
        }
        if _possibility(&(n - i), arry, memo) {
            memo.insert(*n, true);
            return true;
        }
    }
    memo.insert(*n, false);
    false
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn possibility_1() {
        assert_eq!(possibility(5, &[1, 2, 3]), true);
    }
    #[test]
    fn possibility_2() {
        assert_eq!(possibility(15, &[6, 4, 10]), false);
    }
    #[test]
    fn possibility_3() {
        assert_eq!(possibility(15, &[2, 3, 5, 7, 0, 3, 10, 20]), true);
    }
    #[test]
    fn possibility_min_1() {
        assert_eq!(possibility_min(15, &mut [5, 2, 1]), 0);
    }
}
