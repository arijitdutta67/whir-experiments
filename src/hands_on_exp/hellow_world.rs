fn main () {
    println!("fib(10) = {}", fibonacci_nonrec(10));
}


fn fibonacci_nonrec(n: u64) -> u64 {
    match n {
        1 | 2 => 1,
        _ => {
            let mut curr = 1;
            let mut prev = 1;
            let mut sum = 0;
            
            for _i in 2..n {
                sum = curr + prev;
                prev = curr;
                curr = sum;
            }
            sum
        }
    }
}

#[cfg(test)]
mod fibtests {
    use super::*;

    #[test]
    fn test_basecases() {
        assert_eq!(fibonacci_nonrec(1), 1);
        assert_eq!(fibonacci_nonrec(2), 1);
    }

    #[test]
    fn test_fib10() {
        assert_eq!(fibonacci_nonrec(10), 55);
    }
}