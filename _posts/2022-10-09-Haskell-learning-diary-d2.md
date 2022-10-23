---
title:  "Learning haskell diary (day #2 & day #3)"
categories:
   - Engineering
tags:
   - Learning
   - Haskell
---
{:.text-justify}
Theses postt is just my personal note when I learn haskell. It started on 2020 during COVID quarantine period. I have suspended it for a bit then start over again. This diary is based on [Yann's writting](https://yannesposito.com/Scratch/en/blog/Haskell-the-Hard-Way/#navigation).
{: .notice--info}

# Recap previous lesson

Last week (day #1 of learning), I has defined an exercise to work on, it is the re-invent-the-wheel CRC implementation, however, somehow working with byte/bit in haskell is not straight forward like doing it in C, in fact, it need a special data resprentation for bit/byte data. So I was reinvent the wheel something to work on hex data, it is some utility to covert hex string to integer. Now I need to expand it to an array. Let's get started.

# Continue with hex string

Firstly, I need some function to split the hex string to array of byte-string is like tho following:

```haskell
-- This function slit long string into series of 2-char string.
hexsplit (x:y:xs)  = [[x]++[y]]++ hexsplit xs
-- the following line handle if the input doesn't have enough nibble at the end of the string
hexsplit (x:[])  = [[x]++"0"]
```

The above function look quite complicated, as string in haskell is treated as the array of Char, so the array of string is array of array of char, so we will need multiple square braket for that to work.

Then now, we need to convert the array of byte string to Integer, I mean the `uint8_t` in C programming language. With the function we learn from the previous day, we come up with using the  `map` function, that convert series of data to another series of data. The  mapping is done by a function that convert **one** data to another **one** data.

```haskell
hexstring_2_byte_array' xs = map hex2uint_no_prefix $ hexsplit xs
-- Which in turn can be eta-reduced. Be noticed to the dot "." function. \
-- This called the function composition, it is the same terminology in math:
-- H = F . G <=> h(x) = f(g(x))
hexstring_2_byte_array = map hex2uint_no_prefix . hexsplit
```

Pretty cool, huh?

## Move to the main part

Let's say how do we implement the CRC algorithm well. I see another problem is the XOR operation, like `a ^ b` is pretty easy in C but in haskell, I should do the following (pretenting Haskell doesn't have any library for that):

* Define xor for bit

* Define xor for double bit

* Define xor for nibble

* Define xor for a byte

* And soon until I make xor for uint32

Interesting enough, at first, a was intended to do the xor for bit, then do the xor for array of bit, but the xor for array of bit is quite frutstrated for me, so I decide to do it like the above, interesting enough, I got to know that haskell allow me to define the operation quite easy, the following snippet show the way I define the leftshift and right shift operator.

```haskell
-- Right shift
bit_shr_u x n = div x (2^n)
-- Defnie right shift operator
(<>>>):: Int -> Int -> Int
a <>>> b  = a `bit_shr_u` b

-- left shift
bit_shl x n =  (2^n) * x
-- Define left shift using << symbol
(<<<>):: Int -> Int -> Int
a <<<> b = a `bit_shl` b
```

It seems that haskell only accept the  special character in side the square braket `< >  `.

Then the following is all the fundamental function needed to work with CRC implementation.

```haskell
-- Masking n LSB bit
and_msk_lsb a n = a `mod` (2^n) 


take_n_th_bit x n = mod  (bit_shr_u x n) 2

xor_u2 a b = (bit_shl (bit_xor (take_n_th_bit a 1) (take_n_th_bit b 1)) 1) + 
               (bit_shl (bit_xor (take_n_th_bit a 0) (take_n_th_bit b 0)) 0)

xor_u4 a b = (( xor_u2 (a <>>> 2) (b <>>> 2) ) <<<> 2 ) + 
                  ( xor_u2 (and_msk_lsb a  2) (and_msk_lsb b 2) ) 

xor_u8 :: Int -> Int -> Int
xor_u16 :: Int -> Int -> Int
xor_u32 :: Int -> Int -> Int 

xor_u8 a b = ((xor_u4 (a <>>> 4) (b <>>> 4)) <<<> 4) +  (xor_u4 (and_msk_lsb a  4) (and_msk_lsb b 4)) 
xor_u16 a b = ((xor_u8 (a <>>> 8) (b <>>> 8)) <<<> 8) +  (xor_u8 (and_msk_lsb a  8) (and_msk_lsb b 8)) 
xor_u32 a b = ((xor_u16 (a <>>> 16) (b <>>> 16)) <<<> 16) +  (xor_u16 (and_msk_lsb a  16) (and_msk_lsb b 16)) 

```

Then talking about the CRC implementation, the origin of CRC implementation is the XOR circuit, in which the output is calculated bit-by-bit. One of the enhance implementation using the byte-by-byte implementation which is based on the 256 byte look-up-table. For more details about CRC implementation using the look-up-table method, you can check on [Wikipedia]([Cyclic redundancy check - Wikipedia](https://en.wikipedia.org/wiki/Cyclic_redundancy_check).

At this pont, I has been on the second day of the learning, as it is the first time I have spent time on a serious exercise of the haskell, so the time worths spending. Now, I will describe abit about the method to generate CRC, as I mentioned above, the method I used is look-up-table which I come frome the fact that for X input to the CRC algorithm, it would produce Y output, and when X1=X2, the CRC(X1) equal CRC(X2). So the input is kind of immutable data which is the perfectly problem can be solved using haskell. The idea is that we form the look-up-table which the input data is the bype range from 0-255. The following is an example the C-implementation generating the CRC look-up-table (source code copies from [Barr-Group's post](https://barrgroup.com/embedded-systems/how-to/crc-calculation-c-code).

```c
crc  crcTable[256];

void
crcInit(void)
{
    crc  remainder;
    
    /*
     * Compute the remainder of each possible dividend.
     */
    for (int dividend = 0; dividend < 256; ++dividend)
    {
        /*
         * Start with the dividend followed by zeros.
         */
        remainder = dividend << (WIDTH - 8);

        /*
         * Perform modulo-2 division, a bit at a time.
         */
        for (uint8_t bit = 8; bit > 0; --bit)
        {
            /*
             * Try to divide the current data bit.
             */			
            if (remainder & TOPBIT)
            {
                remainder = (remainder << 1) ^ POLYNOMIAL;
            }
            else
            {
                remainder = (remainder << 1);
            }
        }

        /*
         * Store the result into the table.
         */
        crcTable[dividend] = remainder;
    }

}   /* crcInit() */
```

 We can see that the code inside the inner for loop iterates exactly 8 time, and the expression in the if/else case is consistent, thus, we can rewrite the content of this for loop like this.

```haskell
-- Implementation inside the inner for-loop
crc_low_level_compute remainder poly bit_w =  and_msk_lsb ( (remainder  <<<> 1 ) `xor_u32`  ( poly * take_n_th_bit remainder (bit_w -1))) bit_w

-- The following code iterate the above function 8 times
crc_cal_remainder input poly bit_w = foldl ( \i x -> crc_low_level_compute i poly bit_w) (input <<<> (bit_w - 8 )) ([0..7])


```

Then, we call this 256 time to make the look-up-table? **I think NO!**, because it add no performance boot in haskell, because haskell may be smart enough that there will be repeated pattern for the data that is smaller than 256 (byte value), so the `crc_cal_remainder` is optimized to take only input frame from 0-255. I will make an investigate about this when things finish. Now, just move on with the CRC calculation itself. The full implementation of the CRC would include the following parameters:

* The polynomial: This would indicate if the CRC in the type of CRC-8, CRC-16 or CRC-32. So there would be the function to figure out the bit-width from the polynomial number.

* The Initial value: The prefix value added to the beginning of the data to be checked with CRC

* The final XOR value:  The value that is used to XOR with the final remainder.

Then, the final solution of CRC calculation in haskell will be:

```haskell
crc_calculation poly initVal finalVal input = finalVal `xor_gnr` foldl (\i x -> (i <<<> 8) `xor_gnr` crc_cal_remainder (x `xor_gnr` (i <>>> (bitW - 8) )) poly bitW ) initVal input
                                                where bitW = crc_calc_bit_w poly
                                                      xor_gnr = case bitW of
                                                            8 -> xor_u8
                                                            16 -> xor_u16
                                                            32 -> xor_u32

```

In the snipet above, we have learn something:

* The generic function (`xor_gnr`) can be declared on the fly, that helps us to write the reusable code easily. Less code and more generic mean less bug to produce.

* The switch case equivalent structure `case <compare_value> of`

Then let's test some data with the know-to-be good CRC implementation online at ([Sunshine's Homepage - Online CRC Calculator Javascript](http://www.sunshine2k.de/coding/javascript/crc/crc_js.html):

* Firstly, at the moment, I didn't support reflecting the data and remainder so let's choose some algorithm that dont use reflection

* Input data is `AB CD EF 12`

* Initial value is `FFFF FFFF`

* Final XOR value is `FFFF FFFF`

* Polynomial is `0x4C11DB7`

* Expected result is: `0x6416342B`

I think this is near the end of this exercise, and the remain thing is some improvement which I will do in next week. Actually, I intended to spending less time for this exercise, but it expanded to 3 days up to now, and will be more.
