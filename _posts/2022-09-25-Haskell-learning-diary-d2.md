---

title:  "Learning haskell diary (day #2)"
categories:

- Engineering
  tags:
- Learning
- Haskell

---

{:.text-justify}
Theses postt is just my personal note when I learn haskell. It started on 2020 during COVID quarantine period. I have suspended it for a bit then start over again. This diary is based on [Yann's writting](https://yannesposito.com/Scratch/en/blog/Haskell-the-Hard-Way/#navigation).
{: .notice--info}

# Recap what previous lesson

Last week, I learn some basic concept of the FP and the Haskell language itself. I was instructed by Yann's post and I derived it a bit just for a sake of learning, I was pausing at the `foldl` function. This week, I will try to wrote a simple function myself to calculate CRC for a series of data, as it seem to be very common in communication. So I will start it by writing requirement for it first

# Requirement for CRC function

* The input data of the function is series of byte with arbitrary length.

* The another input of the function is the CRC function, the CRC function is defined by polynomial, XOR value of the inptu state and XOR value of the output state.

* The output of the function is the CRC result.

* For phase 2, it shalls be written as the library module..

* It shall be the reinvent-the-wheel stuff (do not care if there are solution before).

# Implementation

What? I didn't follow the SDLC, I waived the design phase, am I? Yes, this is learning by failure, stupid.

## The CRC-32 implementation

We use Koopman CRC 32 as the source for implementation. It's polynomial is 0x741B8CD7.

### Binary data in Haskell

TIL that function in haskell shall start with lower case.

At first, I need some function that I can parse hex string to the Binary type in Haskell. So I make that function in the following simple design:

* Check if Hex string is correct by evaluate the first 2 character of hex string segment and then verify the rest of the hex value.

* If the condition is fulfiled, then we convert that hex string to Integer, I am commited to convert upto 64 byte.

```haskell
-- These 2 function is obvious to check if a a character can be hexa digit
isValidHexChar :: Char -> Bool

-- Here I use lambda function and foldl to practice the language
isValidHexChar z = foldl (\i x -> (||) i ((==) z x))  False "012345789abcdefABCDEF"
```
