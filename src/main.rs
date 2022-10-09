use std::env;

use mnemonic::Mnemonic;

mod bip39;
mod mnemonic;

const ERR_INVALID_NUM_ARGUMENTS: &'static str = "invalid number of arguments";

fn main() {
    let mut args = env::args().skip(1).into_iter();
    match args.next().unwrap_or("--help".to_owned()).as_str() {
        "--help" => usage(),
        "2_2" => split_2_2(
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
        ),
        "2_3" => split_2_3(
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
        ),
        "recover" => recover(
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
            args.next().expect(ERR_INVALID_NUM_ARGUMENTS),
        ),
        other => panic!("invalid method: '{}'", other),
    }
}

fn usage() {
    println!(
        r#"
mnemonic-split-opt
Splits a mnemonic S into either 2 of 2 or 2 of 3 splits

For split 2_2: S = A + B
to recover S, provide both A and B

For split 2_3: S = A1 + B1
               S = A2 + B2
               S = A3 + B3
to recover S, provide any of Ai and Bi

USAGE:
    mnemonic-split-opt 2_2 "mnemonic S" "mnemonic A"

    mnemonic-split-opt 2_3 "mnemonic S" "mnemonic A1" "mnemonic A2" "mnemonic A3"

    mnemonic-split-opt recover "mnemonic A" "mnemonic B""#
    )
}

fn split_2_2(s: String, a: String) {
    let s: Mnemonic = s.parse().expect("invalid format for S");
    let a: Mnemonic = a.parse().expect("invalid format for A");

    let b = &s - &a;
    println!(
        r#"to recover secret seed S, use A + B = S
    A: {}
    B: {}"#,
        a, b
    );
}

fn split_2_3(s: String, a1: String, a2: String, a3: String) {
    let s: Mnemonic = s.parse().expect("invalid format for S");
    let a1: Mnemonic = a1.parse().expect("invalid format for A1");
    let a2: Mnemonic = a2.parse().expect("invalid format for A2");
    let a3: Mnemonic = a3.parse().expect("invalid format for A3");

    let b1 = &s - &a1;
    let b2 = &s - &a2;
    let b3 = &s - &a3;
    println!(
        r#"to recover secret seed S, use any of A1 + B1 = S or A2 + B2 = S or A3 + B3 = S
    Write it in 3 separate places:

    Card 1 ------------------------------
    A1: {a1}
    A2: {a2}
    -------------------------------------

    Card 2 ------------------------------
    A3: {a3}
    B1: {b1}
    -------------------------------------

    Card 3 ------------------------------
    B2: {b2}
    B3: {b3}
    -------------------------------------
    "#
    );
}

fn recover(a: String, b: String) {
    let a: Mnemonic = a.parse().expect("invalid format for A");
    let b: Mnemonic = b.parse().expect("invalid format for B");

    let s = &a + &b;
    println!("original secret mnemonic: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_2() {
        let s = "wink crawl gossip";
        let a = "rocket boat cup";

        let s: Mnemonic = s.parse().unwrap();
        let a: Mnemonic = a.parse().unwrap();

        let b = &s - &a;
        assert_eq!("document boring consider", format!("{}", b));
    }

    #[test]
    fn test_2_2_recover() {
        let a = "rocket boat cup";
        let b = "document boring consider";

        let a: Mnemonic = a.parse().unwrap();
        let b: Mnemonic = b.parse().unwrap();

        let s = &a + &b;
        assert_eq!("wink crawl gossip", format!("{}", s));
    }

    #[test]
    fn test_2_3() {
        let s = "wink crawl gossip";
        let a1 = "rocket boat cup";
        let a2 = "afraid slide search";
        let a3 = "solve frozen unknown";

        let s: Mnemonic = s.parse().unwrap();
        let a1: Mnemonic = a1.parse().unwrap();
        let a2: Mnemonic = a2.parse().unwrap();
        let a3: Mnemonic = a3.parse().unwrap();

        let b1 = &s - &a1;
        let b2 = &s - &a2;
        let b3 = &s - &a3;
        println!("{}\n{}\n{}", b1, b2, b3);
        assert_eq!("document boring consider", format!("{}", b1));
        assert_eq!("warrior grunt pepper", format!("{}", b2));
        assert_eq!("coconut steak jacket", format!("{}", b3));
    }

    #[test]
    fn test_2_3_recover() {
        let a1 = "rocket boat cup";
        let b1 = "document boring consider";
        let a2 = "afraid slide search";
        let b2 = "warrior grunt pepper";
        let a3 = "solve frozen unknown";
        let b3 = "coconut steak jacket";

        let a: Mnemonic = a1.parse().unwrap();
        let b: Mnemonic = b1.parse().unwrap();
        let s = &a + &b;
        assert_eq!("wink crawl gossip", format!("{}", s));

        let a: Mnemonic = a2.parse().unwrap();
        let b: Mnemonic = b2.parse().unwrap();
        let s = &a + &b;
        assert_eq!("wink crawl gossip", format!("{}", s));

        let a: Mnemonic = a3.parse().unwrap();
        let b: Mnemonic = b3.parse().unwrap();
        let s = &a + &b;
        assert_eq!("wink crawl gossip", format!("{}", s));
    }
}
