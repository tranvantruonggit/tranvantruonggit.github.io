---
title:  "Learning haskell diary (day #4). Performance evalutation (part 1)"
categories:
   - Engineering
tags:
   - Learning
   - Haskell
   - C
   - Foregin Function Interface
---

{:.text-justify}
Theses posts is just my personal note when I learn haskell. It started on 2020 during COVID quarantine period. I have suspended it for a bit then start over again. This diary is based on [Yann's writting](https://yannesposito.com/Scratch/en/blog/Haskell-the-Hard-Way/#navigation).
{: .notice--info}

# Some review

During previous day, I have been spending time to work on a non hello world function, with the final result is function to calculate CRC for the array of one byte data. So in order to move to the next beautiful of haskell (monad, functor,...) I need to do some evaluation myself. The main evaluation will be the performance point of view, as my understanding, Haskell program, if it is written correctly can reach the performance of the equivalent program written in C/C++. So that is my goal for this post.

# Methodology

## Prepare test data

As I had the function to calculate CRC for the array of data in the 1 byte format. My goal is to measure the time it calculate CRC for about 64KB data. So it is simple as the following snippet:

```haskell
print $ int_2_hexstr $ crc_cal  0x4C11DB7 0xFFFFFFFF 0xFFFFFFFF False False $ map ((<&&&>) 0xff) [0..0xffff]
```

That it, for the sake of quickly testing performance, I create the data block of 64KByte, by define mapping the array form `0` to `0xFFFF` to the consecutive array of data that consists of multiple segment of data that is `[0..255]`. Unfortunately, I saw that the above line take neary forever to finish, so I reduceted it to just mapping from [0..0xFFF] (equivalent to 4KByte data), and it take quite slow. So I will start the evaluation from hear and will optimize it along the road.

## Function to measure time

So we will need the function to measure time duration of our operation. This is from what I have investigated

```haskell
time :: IO t -> IO t
time a = do
    start <- getCPUTime
    v <- a
    end   <- getCPUTime
    let diff = (fromIntegral (end - start)) / (10^12)
    printf "Computation time: %0.3f sec\n" (diff :: Double)
    return v
-- usage
time $ yourFunctionNeedToMeasure
```

Interestingly, I have measure the calculation for my haskell function (the `crc_cal` in my Crkell.hs file), it takes about 7-10 seconds for only 2^12 data byte (4096 bytes). Hmm, I guess my CRC implementation use XOR function, which based on `bitkell.hs` which is quite expensive if it is not using native machine code. Or can be that some of the lazy evaluation doesn't work (because my function was written badly). Let me digging a bit. Firstly, I will hardcoded some of the CRC calculation to check if the algorithm is optimzed in the compile time. Well, I have tried, but the performance seem not to be improved. So I try to improved more, I have seen that the lookup table approach in C is appropriate, but it seems haskell doesn't realize that the input of the CRC calculation is only within the range [0,255] so it doesn't build the lookup table, so I write the enhance implementation by forcing some calculation to be via lookup table.

```haskell
-- original function to calculate remainder
crc_cal_remainder = .....

-- original function to calculate CRC over the array of data
crc_cal_prmtv initVal finalVal input =  fold (\-> ...crc_cal_remainder... ) initVal input


-- now bulding the lookup table
crc_lut = map crc_cal_remainder [0..255]
-- THe following function take the input as the index in the lookup table
enhance_crc_cal input = crc_lut !! input 


-- new function to calculate crc based on lookup table
enhance_crc_cal_prmtv initVal finalVal input =  fold (\-> ...enhance_crc_cal ... ) initVal input

```

By rewritten the `enhance_crc_cal_prmtv` using the `crc_lut` the execution time for 4KB data reduced to 0.xxx second, so it seems we get some good deal. So I increase the input data size to 16 bit (`[0..16383]`) and it take about 3 seconds now. So I will continue to optimize it, but before that, I realize something wrong....

Well, after build the lookup table, I reverted my change in code to reserve the polynomial as input, and I see that the execution time still be as non-optimized in the non-lookup-table approach. Well so my conclusion is that the GHC is not smart enough to automatically decide which way is the best way to do the pre-calculation. I will summaize my finding here for the sake of tracking the idea:

* Original code (no lookup table, dynamic polynomial): 4KB takes ~7 seconds

* Fixed the polynomial as well as the bit width - still no lookup table: 4KB takes ~ 7 seconds.

* Dynamic polynomial, build look-up-table (which also based on the polynomial), then use the lookup table to calculate remainder: 4KB takes 7 seconds.

* Fixed the polynomial as well as the bit width, then build the look-up-table based on this fixed polynomial use this lookup table to calculate remainder: 4KB takes less than 1 second.

So I will fixed the above 4th bullet and continue to optimize from there. I see that even I optimze the CRC remainder calculation, my implementation still use lots of XOR and AND bit operation, then I will finding away to implement these stuff using native machine instruction.

## The Foregin Function Interface - FFI

Interestingly, TIL that `ghc` is somehow sister of `gcc` which it can compile the C file through the `ghc` front end, futhermore, haskell support calling the function written in the other language, and more importantly, this is the built-in designated feature, not the 3rd module or alike so it works like a charm. Basically, there is something that we think if we write in C, it would be more optimal (!?) (well as a guy heavily working with C for my whole career sofar, I do think so, but it takes lots of effort to write an safe implementation). So as I have mentioned, the purpose rides me there is the need to optimize my code by optimizing some low level function that I wrote badly in haskell (the XOR, AND, ... in Bitkell module). Basically, there will be 3 file that need to be used for Ffi feature:

* `Ffi.hs`: This is the Haksell file that other module will import and call the functions which are declared in these files.

* `C_code.h`: This is the header for the function you want to write in C

* `C_code.c`: This has the actual implementation of the function you defined in `.h` file. 

Then here is the content of these file. Basically, I have writing all the low level function in C. Because it is, by far, the utmost fundamental function that somehow the haskell cannot optimize to.

```haskell
-- Ffi.hs
{-# LANGUAGE ForeignFunctionInterface #-}
module Ffi where
import Foreign
import Foreign.C.Types

foreign import ccall "bit_utils.h weird_adder" weird_adder :: Int -> Int -> Int
--Xor
foreign import ccall "bit_utils.h thk_xor32" thk_xor32 :: Int -> Int-> Int
foreign import ccall "bit_utils.h thk_xor16" thk_xor16 :: Int -> Int-> Int
foreign import ccall "bit_utils.h thk_xor8" thk_xor8 :: Int -> Int-> Int

--Or
foreign import ccall "bit_utils.h thk_or32" thk_or32 :: Int -> Int-> Int

--And
foreign import ccall "bit_utils.h thk_and32" thk_and32 :: Int -> Int-> Int
foreign import ccall "bit_utils.h thk_and16" thk_and16 :: Int -> Int-> Int
foreign import ccall "bit_utils.h thk_and8" thk_and8 :: Int -> Int-> Int

--left shift/rightshift
foreign import ccall "bit_utils.h thk_shl" thk_shl :: Int -> Int-> Int
foreign import ccall "bit_utils.h thk_shr" thk_shr :: Int -> Int-> Int

--test bit
foreign import ccall "bit_utils.h thk_tstb" thk_tstb :: Int -> Int-> Int



```

```c
/* bit_utils.h */
#include "Rts.h"
#include <stdint.h>
extern uint32_t weird_adder( uint32_t a, uint32_t b);

/* thk for thingkell */
extern uint32_t thk_xor32( uint32_t a, uint32_t b);
extern uint16_t thk_xor16( uint32_t a, uint32_t b);
extern uint8_t thk_xor8( uint32_t a, uint32_t b);

/* And operation */
extern uint8_t thk_and8( uint32_t a, uint32_t b);
extern uint16_t thk_and16( uint32_t a, uint32_t b);
extern uint32_t thk_and32( uint32_t a, uint32_t b);

extern uint32_t thk_or32( uint32_t a, uint32_t b);

/* Shift */
extern uint8_t thk_shl8( uint32_t a, uint8_t b);
extern uint16_t thk_shl16( uint32_t a, uint8_t b);
extern uint32_t thk_shl32( uint32_t a, uint8_t b);
extern uint32_t thk_shl( uint32_t a, uint8_t b);

extern uint32_t thk_shr( uint32_t a, uint8_t b);

/* Get nth bit */
extern uint32_t thk_tstb( uint32_t val, uint8_t pos);



```

```c
/* bit_utils.c */
/* Yeah, you have to implement it yourself folk, you can told the OpenAI GPTChat to do it for you, can t you ?
```

And by doing this to replace the xor/and/or/left-shift/right-shift I wrote in haskell in the previous day, I see the huge jump in the performance point of view, something like 2MB CRC calculation in the degree of second (3-5 seconds). Obviously, I see that it cannot beat the implementation of other off-the-shelf application I tried on windows (`srec_cat`  for your reference, this is the [home page](https://srecord.sourceforge.net/) of that project ).



*It seems this post take forever, so I will cutting it into 2 part, part 2 will include the benchmark tabular report .*