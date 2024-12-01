use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_list<T: FromStr>(input: &str) -> impl Iterator<Item = T> + use<'_, T>
where
    <T as FromStr>::Err: Debug,
{
    input.lines().map(|l| l.parse().unwrap())
}
