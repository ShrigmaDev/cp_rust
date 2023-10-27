use std::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    // start
}

// sum of numbers l, l+1, l+2 ... r
fn range_sum(l: i128, r: i128) -> i128 {
    ((l + r) * (r - l + 1)) / 2
}

fn lcm<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialEq
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Rem<Output = T>
        + std::default::Default
        + Copy,
{
    return a / gcd(a, b) * b;
}

fn gcd<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + std::default::Default + Copy,
{
    if b == Default::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

// use .into() for matching into tuple
fn split_into_arr<T, const N: usize>(delim: char) -> [T; N]
where
    T: FromStr,
    T::Err: Debug,
    T: Debug,
{
    read_into_vec(delim).try_into().unwrap()
}

// if need empty delim, read into string and loop over .chars()
fn read_into_vec<T>(delim: char) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let input: String = read_parse_trim_line();
    input
        .split(delim)
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

fn read_parse_trim_line<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
}
