use std::fmt::Display;

/// Solution to an aoc problem.
#[derive(PartialEq, Eq, Debug)]
pub enum Solution {
    /// A numeric solution.
    ///
    /// # Example
    /// ```
    /// # use aoc::Solution;
    /// let s = Solution::Num(52);
    /// let s = Solution::from(52u32);
    /// let s = Solution::from(52i32);
    /// let s = Solution::from(52u64);
    /// let s = Solution::from(52i64);
    /// ```
    Num(i64),

    /// A textual solution.
    ///
    /// # Example
    /// ```
    /// # use aoc::Solution;
    /// let s = Solution::from("Xt454dDdz56");
    /// let s = Solution::from("
    /// #######################\n\
    /// ###    ##    ##     ###\n\
    /// ###### ## ##### #######\n\
    /// ####   ##    ##     ###\n\
    /// ###### ## ## ###### ###\n\
    /// ###    ##    ##     ###\n\
    /// #######################\n\
    /// ");
    /// ```
    String(String),

    /// Indicates when a function fails to solve the problem,
    /// e.g. fir djisktra, dfs, ...
    NotFound,

    /// For a solution that is not yet implemented.\
    /// This is particulary usefull part1 is being implemented, but part2 isn't yet.
    NotImplemented,

    Day25Part2,
}

impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Solution {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Num(x) => write!(f, "{x}"),
            Solution::String(s) => write!(f, "{s}"),
            Solution::NotFound => write!(f, "Not found"),
            Solution::NotImplemented => write!(f, "Not implemented yet"),
            Solution::Day25Part2 => write!(f, "-"),
        }
    }
}

/// Nice workaround for avoiding conflicting implementations :
/// https://stackoverflow.com/questions/39159226/conflicting-implementations-of-trait-in-rust
pub trait Numeric {}
impl Numeric for i32 {}
impl Numeric for u32 {}
impl Numeric for i64 {}
impl Numeric for u64 {}

impl<T> From<T> for Solution
where
    T: num_traits::NumCast + Numeric,
{
    fn from(value: T) -> Self {
        Self::Num(num_traits::cast(value).unwrap())
    }
}
