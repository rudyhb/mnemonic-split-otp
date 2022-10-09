# Splitting mnemonic phrases using One-time pad

Split a mnemonic phrase `S` into either

- 2 of 2: where you will need both `A` and `B` to recover `S`
- 2 of 3: where you will split into 3 "cards" `A1 & A2`, `A3 & B1`, `B2 & B3`.
  In this case you need any 2 of the 3 cards to recover `S`.

Based on https://tylerchambers.net/posts/2of3/

**Note: in these examples I use mnemonics length 3 for simplicity. For normal cases these should be length 12 or 24.**
## 2 of 2 Splitting

You will need both of these outputs (`A` and `B`) to recover secret `S`.

secret mnemonic `S = "wink crawl gossip"`

one-time-pad `A = "rocket boat cup"`

```shell
cargo run -- 2_2 "wink crawl gossip" "rocket boat cup"
```
will output:
```shell
to recover secret seed S, use A + B = S
    A: rocket boat cup
    B: document boring consider
```

## 2 of 3 Splitting

You will need any 2 of these 3 outputted cards to recover secret `S`.

secret mnemonic `S = "wink crawl gossip"`

one-time-pad `A1 = "rocket boat cup"`

one-time-pad `A2 = "afraid slide search"`

one-time-pad `A3 = "solve frozen unknown"`

```shell
cargo run -- 2_3 "wink crawl gossip" "rocket boat cup" "afraid slide search" "solve frozen unknown"
```
will output:
```shell
to recover secret seed S, use any of A1 + B1 = S or A2 + B2 = S or A3 + B3 = S
    Write it in 3 separate places:

    Card 1 ------------------------------
    A1: rocket boat cup
    A2: afraid slide search
    -------------------------------------

    Card 2 ------------------------------
    A3: solve frozen unknown
    B1: document boring consider
    -------------------------------------

    Card 3 ------------------------------
    B2: warrior grunt pepper
    B3: coconut steak jacket
    -------------------------------------
```

## Recovering the secret mnemonic

To recover S, you need any `Ai` + `Bi`.

`A = "rocket boat cup"`

`B = "document boring consider"`

```shell
cargo run -- recover "rocket boat cup" "document boring consider"
```
will output:
```shell
original secret mnemonic: wink crawl gossip
```
