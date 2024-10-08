// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n == 1  {
        return n;
    }
    let mut reverse_counter: u32 = n; // 2 2 
    let mut sum: u32 = 0;  // 0
    while reverse_counter > 1 {
        if reverse_counter == n { 
            sum = n * (n-1); 
        } else {
            sum = sum * (reverse_counter - 1);
        }
        reverse_counter-=1;
    }
    sum
    
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
