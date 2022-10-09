---
title:  "Learning haskell diary (day #1)"
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

Firstly, I think of the way to check if the string is a hex string, this is obviously checking if the first 2 chars of the string is `0x` or `0X` and the following string shall be an valid series of hex character. That idea is implemented as the following. I was trying to use `foldl` as much as possible to be familiar with that high level function, I also wanna add some prefix notation in the code to make it looks messier. I also try to use the lambda expression to reduce the number exposed function.

```haskell
-- These 2 function is obvious to check if a a character can be hexa digit
isValidHexChar :: Char -> Bool

-- Here I use lambda function and foldl to practice the language
isValidHexChar z = foldl (\i x -> (||) i ((==) z x))  False "012345789abcdefABCDEF"

-- This is the first function to check series of hex digit is valid
isValidHexString_v1 xs = foldl (\ i x->  i && isValidHexChar x ) True xs
isValidHexString = foldl (\ i x->  i && isValidHexChar x ) True 

isStringHex(x:y:xs) = 
   if (([x]++[y] == "0x") || ([x]++[y] == "0X")) && (xs /= "")
   -- this is the correct condition for hex string, then check if this hex string has any weird data
   then isValidHexString xs
   else False


```

Those above validation is necessary in C, I am kind of confusing to decide whether I need to actually use those evaluation. Let's me describe later when I implement the core function of the hex string to integer converter.

Let's take an example "0x123456" , this string is obviously equilavent to `1193046`, which is the result of the following equation

```6*(16^0)
6*(16^0) + 5*(16^1) + 4*(16^2) + 3*(16^3) + 2*(16^4) + 1*(16^5)
```

 This equation naturally come to our mind, however it has some draw back:

* The operand of each multiply operation is going backward (from 6->1) from the end of hex string.

* We need to calculate the number of hexstring in order to know the end of the loop.

My gut tells me that not to use any way that involve using the loop and the "iterator variable" (0, 1, 2 ,... number in the power operation in above equation). Thus, I think of the following way to calculate the hex base number to dec base number.

```
(((((0*16+1)*16+2)*16+3)*16+4)*16)+5)*16+6
```

The above equation doesn't use the iterator variable, and it doesn't need to know the length of the hex string, and it look repeated pattern. So it can be easily implement using `foldr` as the following.

```haskell
hex2uint (x:y:xs) = foldl (\ i x -> i*16 + hexchar2int x) 0 xs
```

The question is how we shall imlement `hexchar2int`, well it shall be lookup table, so how can we implement lookup table in haskell? Well the following full solution will amaze you.

```haskell
hexchar2int '0' = 0 
hexchar2int '1' = 1 
hexchar2int '2' = 2 
hexchar2int '3' = 3 
hexchar2int '4' = 4 
hexchar2int '5' = 5 
hexchar2int '6' = 6 
hexchar2int '7' = 7 
hexchar2int '8' = 8 
hexchar2int '9' = 9 
hexchar2int 'A' = 10
hexchar2int 'B' = 11
hexchar2int 'C' = 12
hexchar2int 'D' = 13
hexchar2int 'E' = 14
hexchar2int 'F' = 15
hexchar2int 'a' = 10
hexchar2int 'b' = 11
hexchar2int 'c' = 12
hexchar2int 'd' = 13
hexchar2int 'e' = 14
hexchar2int 'f' = 15

hex2uint (x:y:xs) = foldl (\ i x -> i*16 + hexchar2int x) 0 xs
```

The way defining lookup table in this case is very intuitive. Now, let's me describe about the evaluation:

* In the above solution, I was assuming not to evaluation the `0x` prefix of the hex string and assuming the content of the hexstring contain only hex characters.

* What if the prefix `0x` is not correct: simply nothing happen becasue the `x:y` is omit in the function implementation.

* What if the number contain invalid hex chars? Let's me show you went I wanna print `hex2uint "0x1y2"`

```sh-session
➜  haskell_learn git:(master) ✗ make run
ghc crc.hs
[1 of 1] Compiling Main             ( crc.hs, crc.o )
Linking crc ...
./crc
"Hello"
'0'
"Bello"
False
True
crc: crc.hs:(27,1)-(48,20): Non-exhaustive patterns in function hexchar2int

make: *** [Makefile:2: run] Error 1
➜  haskell_learn git:(master) ✗ git:(master) ✗
```

You see, haskell does some evaluation natively when it cannot find the way to convert the char `y` to decimal number. 

The final solution for the `hex2uint` will be:

```haskell
hex2uint ('0':'X':xs) = hex2uint $ "0x"++ xs -- Dolar sign replace parenthesis
hex2uint ('0':'x':xs) = foldl (\ i x -> i*16 + hexchar2int x) 0 xs
```

I think I have spent too much to this exercise, and I am still near the half way to finish it so I decided to stop today and continue this exercise in the next week.
