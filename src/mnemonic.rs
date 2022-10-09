use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;

use super::bip39;

#[derive(Debug)]
pub struct Mnemonic(Vec<usize>);

impl FromStr for Mnemonic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace()
                .map(|word| bip39::eng::get_index(word))
                .collect(),
        ))
    }
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|i| bip39::eng::get_word(*i))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Add for &Mnemonic {
    type Output = Mnemonic;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());
        Mnemonic(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(&lhs, &rhs)| (2048 + lhs + rhs) % 2048)
                .collect(),
        )
    }
}

impl Sub for &Mnemonic {
    type Output = Mnemonic;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());
        Mnemonic(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(&lhs, &rhs)| (2048 + lhs - rhs) % 2048)
                .collect(),
        )
    }
}
