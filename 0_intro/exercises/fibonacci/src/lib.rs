pub fn fibo(v: u32) -> u32 {
    match v {
        0 => 0,
        1 => 1,
        x => {
            let mut n1 = 0;
            let mut n2 = 1;

            for _ in 1..x {
                let tmp = n2;
                n2 += n1;
                n1 = tmp;
            }

            n2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibo_0() {
        assert_eq!(0, fibo(0));
    }

    #[test]
    fn fibo_1() {
        assert_eq!(1, fibo(1));
    }

    #[test]
    fn fibo_2() {
        assert_eq!(1, fibo(2));
    }

    #[test]
    fn fibo_big() {
        assert_eq!(610, fibo(15));
    }
}
