---
title:  ""
categories:
   - Thinking
tags:
   - Funny
---
As a tradition, when some guys on the internet asking about career path in automotive embedded software. This reddit comment pop-up in my mind. I am worrying that some day, this golden comment being destroyed, thus I backup here.

[The original comment](https://www.reddit.com/r/embedded/comments/leq366/comment/gmiq6d0/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)

```(markdown)


    What did AUTOSAR solve?

It's a job creator for application engineers.

    Did software become less complex and easier to develop/configure?

Jesus fucking christ fuck no.

    Did the software become smaller or faster with AUTOSAR?

See above.

Forgive my vulgarity.

I spent 2+ years trying to unfuck an existing AUTOSAR project at a company that builds vehicle telematic ECUs. I can safely say that the AUTOSAR standard and all available implementations (Vector, Mentor Graphics, and Electrobit) is an upside-down on fire dumpster full of dog shit.

I'm not even gonna go into how much trouble it is to get the attention of an AUTOSAR distributor. If you're a student or small business and you just want to see the specification then you can go grab it from autosar.org. It's just a bunch of PDFs and they're all free. But the implementation is where you'll spend money, and if you aren't Ford / GM / some automotive startup with a shit ton of money, nobody will want to talk to you because AUTOSAR is $$$$$$$$$$.

Let's assume you get past all that. After you spend months negotiating contracts with Mentor Fuckups or Eletroshit or whatever AUTOSAR package you decided was the least bad, you'll spend a few more months sitting in online seminars while some talking head explains why it takes 6 hours to configure a million goddamn things so their garbage tool can shit out an entire Italian resaurant's worth of spaghetti code just to blink an LED at 1Hz. Except it's not 1Hz, it's 10Hz, or 0.1Hz, or some other bullshit that you didn't want, because you muttered the wrong incantation to the configuration utility somewhere around step 2 out of 800, so guess what, you get to back and do the entire fucking thing again.

Get the flying fuck out of here if you want to try doing something more complicated than that. You want to send a CAN frame? Get completely fucked. You have to buy a separate tool beyond what you already spent $800k on just to generate the stupid little ARXML "network definitions" files that are compatible with your AUTOSAR configurator, learn how to use that, then fight problems between export / import because of course both tools were written by different teams and aren't 100% compatible (WHY THE FUCK WOULD ANYTHING EVER JUST FUCKING WORK?), then get your application developers to understand how to interface their C code to the new "network interface layer", and etc etc etc.

Now let's assume you're track-side and you want to add a new CAN signal just to get some info out of your ECU. With vanilla C it's a matter of hacking something in quickly OR running some custom interface generation script that takes a few seconds, like the howerj/dbcc scripts. AUTOSAR? Hours. Literally even days if something goes wrong and your entire project blows up. Our firmware test turn-around window was weeks just because it took so long to get the network definition generator to spit out something useful without breaking a million other things, all the while the AUTOSAR vendor gave us excuse after excuse, patch after patch, never really fixing the problem.

The final straw came when we realized we couldn't hire anyone who would want to mess with this shit. Imagine working in the industry for 10 years, and then you take a job where someone says "Hey thanks for spending all that time learning all that cool stuff that we can't use, here's the MSPaint equivalent of embedded software development platforms, we need Mona Lisa by lunch." Our choice was either over-pay seniors to deal with this giant pile of 5th stage super-cancer, or hire kids fresh out of college and hope they wouldn't wise up to the fact that any skillset developed while working with AUTOSHIT was absolutely useless and bail.

After 2 years with 20+ engineers working like this we gave up. Everyone else went on vacation while 3 guys spent a few days getting ADC ports, CAN, LIN, SPI, and a few other things up an running on a development kit. We had a working ECU running on a vehicle less than 2 weeks later. And that was from starting with a clean install of a compiler + IDE and a blank main.c. Our code safety team ran it through the ISO26262 verification process and everyone's stress level was suddenly much lower because they could actually DO THE JOB THEY WERE HIRED TO DO instead of fighting the trash-tier AUTOSAR tool all day.

I would rather shove a shotgun in my ass and blow my god damn balls off than ever lay my eyes on AUTOSHIT ever again. Complete fucking waste of time. If ever you see AUTOSAR on a job description then fucking RUN.

```