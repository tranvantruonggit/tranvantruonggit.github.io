---
title:  "Learning haskell diary (day #4)"
categories:
   - Engineering
tags:
   - Learning
   - Haskell
---
{:.text-justify}
Theses posts is just my personal note when I learn haskell. It started on 2020 during COVID quarantine period. I have suspended it for a bit then start over again. This diary is based on [Yann's writting](https://yannesposito.com/Scratch/en/blog/Haskell-the-Hard-Way/#navigation).
{: .notice--info}

# Recap
In the previous weeks, I was working on the core calculation of the CRC, the outcome is quite satisfying to me and I have just missed implementation for the reflection of the input data and output result. However, these implementation is quite easy to add on top of the previous calculation to make the full CRC solution (cover upto 32 bit CRC). Thus I just need to rename some function to make it a **primitive** functions that will be added the feature reflection upon it.
```(haskell)
crc_cal_prmtv poly initVal finalVal input = finalVal `xor_gnr` foldl (\i x -> (i <<<> 8) `xor_gnr` crc_cal_remainder (x `xor_gnr` (i <>>> (bitW - 8) )) poly bitW ) initVal input
                                                where bitW = crc_calc_bit_w poly
                                                      xor_gnr = case bitW of
                                                            8 -> xor_u8
                                                            16 -> xor_u16
                                                            32 -> xor_u32

-- crc_calc poly initVal finalVal  reflectIn reflectOut input = crc_cal_prmtv initVal finalVal
crc_calc poly initVal finalVal False False  = crc_cal_prmtv poly initVal finalVal 
crc_calc poly initVal finalVal  True False   = crc_cal_prmtv poly initVal finalVal . map  reflect_byte 
crc_calc poly initVal finalVal  False True  = reflect_gnr . crc_cal_prmtv poly initVal finalVal
                                                where bitW = crc_calc_bit_w poly
                                                      reflect_gnr = case bitW of
                                                         8 -> reflect_byte
                                                         16 -> reflect_word
                                                         32 -> reflect_dword
crc_calc poly initVal finalVal  True True   = crc_calc poly initVal finalVal False True . map reflect_byte
```