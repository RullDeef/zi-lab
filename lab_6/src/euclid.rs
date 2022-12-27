use num::Integer;

pub fn extended_gcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Integer + Clone + std::fmt::Display
{
    if a < b {
        return extended_gcd(b, a);
    }

    let mut r0 = a;
    let mut r1 = b;
    let mut s0 = T::one();
    let mut s1 = T::zero();
    let mut t0 = T::zero();
    let mut t1 = T::one();

    while !r1.is_zero() {
        let q = r0.clone() / r1.clone();

        let r_next = r0.clone() - q.clone() * r1.clone();
        let s_next = s0.clone() - q.clone() * s1.clone();
        let t_next = t0.clone() - q.clone() * t1.clone();

        r0 = r1.clone();
        s0 = s1.clone();
        t0 = t1.clone();
        r1 = r_next;
        s1 = s_next;
        t1 = t_next;
    }

    (r0, s0, t0)
}

#[test]
fn validate_gcd() {
    let a = 5;
    let b = 13;

    let (gcd, c, d) = extended_gcd(a, b);

    println!("{gcd}, {c}, {d}");

    println!("{}", a * c + b * d);
    println!("{}", a * d + b * c);
}