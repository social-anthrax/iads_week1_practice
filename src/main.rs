#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
mod sorting_algos;

fn main() {
    let a = std::env::args()
        .nth(1)
        .expect("No number given")
        .parse()
        .unwrap();
    let n = std::env::args()
        .nth(2)
        .expect("No number given")
        .parse()
        .unwrap();
    let m = std::env::args()
        .nth(3)
        .expect("No number given")
        .parse()
        .unwrap();

    println!("{}", expmod(a, n, m))
}

fn expmod(a: i32, n: i32, m: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    let d = expmod(a, (n as f32 / 2 as f32).floor() as i32, m);
    if (n & 1) == 0 {
        return (d * d) % m;
    } else {
        return (d * d * a) % m;
    }
}

#[cfg(test)]
mod test {
    use crate::expmod;

    #[test]
    fn test_expmod() {
        assert_eq!(expmod(10, 20, 17) as i128, (10i128.pow(20)) % 17)
    }

    #[ignore = "Quickcheck overflows stack"]
    fn test_predicate(a:i8, n:i8, m:i8 ) -> bool{
        let (a, n, m) = (a.into(), n.into(), m.into());
        expmod(a, n, m) == (a^n)%m
    }
}
