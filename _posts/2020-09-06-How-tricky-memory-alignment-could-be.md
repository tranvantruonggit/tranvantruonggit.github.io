---
title:  "How tricky the memory alignment would be."
last_modified_at: 2021-06-09T16:20:02-05:00
categories:
   - Engineering
tags:
   - Embedded system
   - Knowledge sharing
---
{:.text-justify}
I am going to discuss about the memory aligment issue which we faced many times during development of the low level software. This topic is applied for any kind of MCU and it peripherals, but I will may refer to the PowerPC architecture or ARM Cortex-M which I have more experience with.

# What is memory alignment? #
It is how the memory access multiple bytes, for insance, if we want to access 32-bit data on the memory concurrently, we have to be consious about the memory aligment. Why?

You may remember from the day in the university, when you learn about the computer architecture, you had learnt about how memory is load to the CPU register. This is done by a n-bit address decoder, where the output of the circuit is the content of the memory at the specified address. The memory is byte addressable, the normally, the CPU can load one byte in one cycle. However, for the 32-bit address CPU, the regiter is 32-bit, then the CPU designer optizime the memory load operation to be 4-byte for one load instruction. At this point, they often omit 2 LSB of the address for some optimization, that means the CPU can natively fetch 4-byte in the memory array IN ONE CYCLE if the address is 4-byte aligned, which mean the address of the whole 4-byte data shall have the 2LSBs to be zeros.
[Need to put figures here]

# What compiler deal with the memory aligment. #

That means, if a 32-bit variable is doesn't have 4-byte aligned address, it will be 2 load instructions for 16-MSB and 16-LSB, then the concatenate happen behind the scene. However, most of the C compiler writer understand this and they (nearly) always put the 32-bit variable to the 4-byte aligned address during linking phase, and thus, improve the performance.

However, insome rare cases, the compiler is not aware of the aligment (such as a byte array as an member of a struct), they oftens provide a language extension feature to mark a specific variable to be aligned. For most of gcc-base compiler it is the keyword **\_\_attribute__ ((aligned (N)))**, which N is the number of bit to be aligned.

Moreover, the memory aligment can also be forced for a memory section in the linker script. This practice is very common in defining the vector table, in which, the base address of the table is need to be X bit aligned as required by the hardware design of the interrupt controller (for example, the register holding the base address of the vector table is effective for only 16 MSBs).

# How is it tricky? #
During my career so far, I have faced a decent amount of problems related to memory aligment, these problem is not obvious as we have seen. For instance, your MCU does'n run at all after adding a few harmless line of code, while it had worked flawlessly previously. Another exapmle, it happens on the very rare curcumstance that is very hard to reproduce bay any kind of test cases. Thankfully, we have a solid understanding of how things working underhood, by analyze the memory map of the executable, we can have some good hypothesis to describe what happen with our system, then we just have to check with the MCU reference manual in a grain detail and found the root cause. As you know, the important of solving the problem is finding the root cause, the solution is an obvious thing comes after. Below line are some specific case of the memory aligment that may be interesting to know.

## The slave cores don't boot in the multicores software platform. ##
We was working with the multicore platform software on MPC5xxx. In our platform, one core will be the the master core that start from POR, the other slave cores are started by the master core. The start address of the slave core are written to the register named MC_ME_CADDRx by software prior triggering the mode change. We have used this type of mechanism many times in the past without any problem, however, we weren't luck that time. We have checked every register during that bootup and realize that the core address is not fetched correctly. The problem is that the MC_ME_CADDRx has only 30MSB to store the start address, the other 2-LSBs is for other purpose/reserved, but the start address of the function we defined in assembly code is only 2-bit aligned (as we compiled with the VLE mode), so by just forcing the startup function to be 4 byte aligned, our software can boot all cores sucessfully.

## Software crashes because of the misaligned-memory address feeding to DMA ##
This problem happen when we working with the CAN communication module, we have used the DMA to offload the CPU, the DMA copy from CAN FIFO buffer to software buffer on RAM. My co-worker working on that issue, told me that the software crashed right after he send the CAN message from the tester, while it works flawlessly previously with the interrupt scheme. I have asked him to dump the memory address of where the destination of the DMA copies to, he told me the address is odd address, I asked him how many byte per cycle the DMA carries and he reports that it was configured to 4-bytes. Then I told him tried to amend his code that the buffer in RAM shall be 4-byte aligned. And of course, the software doesn't crash then. Obviously, the problem is very clear that whether the misaligned memory may cause a) Bus Access Error if the hardware is designed to work like so; b) The actual memory address the DMA working on is replaced, that is the content of the adjacent memory of the intended destination memory. This adjacent memory may be the control block of the OS or some critical state of something else.
This problem is quite commn, but also often be overlooked during setting up the feature related to memory like DMA, or setting up the buffer address of some kind. Anyway, these kind of problem is not difficult to spot out.

## The lasts problem is kind of a bad practice of C programming ##
Sometimes, design constrains force someone to do so. It is where we have to transfer a structured data over the network as a byte stream. Some unexperience engineer may using the pointer of the target struct type, then during the scanning for the token denote the begining of the data structure, the assign the address of the found token to the pointer, then accessing the member of the struct to get the value. Obviously, it is not the "always" correct way of parsing data, because normally, the struct data type is often 4 or 2 byte aligned by it nature, but the position of the token in the bit stream may not be aligned at all. Then doing so would cause the Bus Error Exception. The correct way to do so is defined a temporary struct inside a function (stack) then, copy data from the byte stream to this struct variable, then we can get whatever we want from this struct. This obviously cost us some memory in stack and overhead in copying data, but it is obviously the correct way to handle the stuff.