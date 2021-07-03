---
title:  "Using wireshark and sshdump to capture ethernet packets remotely"
categories:
   - Engineering
tags:
   - Embedded system
   - Knowledge sharing
---
{:.text-justify}
This post is just way I spent my free time to record what I have done during the hard time in the pandemic.

# The background story
{:.text-justify}
During the pandemic in my country in 2021 summer, we are recommended to work from home. Honestly, I prefer working in office because of the professionalism feels and the we have all the supporting tools in the office (fancy tools, big screen, and obviously beautiful ladies as well jk). But WFH now is unavoidable. At that time we are doing the project that working with the ethernet feature, our project have 3 member shared one physically hardware. The solution here is that I will setup the debug station in my house so that the 2 other folks can access to it. It is the trivial task for the very good debugger we had, it does support the remote debugging, the hardware target and the debugger is placed at my home, is connected to the internet, the other person will run the client debugger on their PC, connected to the debugger in my home and debugging like they have the connected the debugger with their PC. Doesn't it sounds cool huh?
{:.text-justify}
The problem rose when we work with the Ethernet feature of the HW target, we would like to analyze what if our board sending out something. You know, that task is trivial if you can connect the the target hardware to the LAN port on your PC.

# I have some valuable tools that the normal person would have overlooked it.
I would describe a bit about my my home network setup in the following diagram.

![Network diagram](https://drive.rtos.dev/f/6347be14a39a47d4b158/?raw=1)

Look at the diagram, you will see that I have a main server works as the debug station as I mentioned. and fortunately, I have a spare [**EspressoBin**](http://espressobin.net/), an SBC board with powerful networking capability, hmm it seems that it's the time it shining. I installed the [**OpenWRT**](https://openwrt.org/) on that, from there, it is the fullfearuted router, I configured it to work like a switch and install **tcpdump** so it can capture the network packages flowing through it, and because our eval board is connected to it, so done, we have the capability to capture the message from the board. Just ssh to the router and run the command *tcmpdum*.
{:.text-justify}


# The whole solution with sshdump

{:.text-justify}
However, we obviously dont want to observe the package via commandline interface when we have a powerful wireshark, the problem is that the OpenWRT board is not capable of installing Wireshark. Luckily, my main server is as full working workstation running Arch Linux so it can obviously run wireshark. At the time of wirting this post, I am running wireshark 3.4.6 on Archlinux, it does has the sshdump which mean you can use ssh as a channel to get the data from remote machine. In this case, it will be like the below diagram. 
![](https://drive.rtos.dev/f/8d584b2db5734d32bf56/?raw=1)

That's the principle, then move to the how. 

**Notice: when you install wireshark on Windows, please remember to opt in to install sshdump, it is not default option for Windows version of wireshark.** 

### Step 1: choose the sshdump as interface
![](https://drive.rtos.dev/f/819811cefad543d2b604/?raw=1)

### Step 2: input the ssh server address and the ssh credential.
Just specify as usual, change or leave it blank to fit your setup.
![](https://drive.rtos.dev/f/ffa9bb42815a42149550/?raw=1)

### Step 3: Specify the interface in the remote machine (this case is interface of my Espressobin).
The only required field is the interface and the checked box **"Use sudo on the remote machine"**. I would like to make a note about this, from my experience, it is likely that we should use the sudo to run *tcpdump* on the OpenWRT, because it there is not easy way to run tcpdump without root access. However, it is not the problem itself because we can set that the user can run sudo without password with for only *tcpdump*. Please refer to the following image.
<img src="https://drive.rtos.dev/f/48ecde620d794b9bbd96/?raw=1" alt="drawing" width="500"/>
We should leave the *"Remote capture command"* to be blank as well, as ssh dump has handled this automatically, unless you have a specific need to specify the command. Then we can press start now.

### And finally, voila.....
![](https://drive.rtos.dev/f/8b53041e2fc047d49197/?raw=1)
