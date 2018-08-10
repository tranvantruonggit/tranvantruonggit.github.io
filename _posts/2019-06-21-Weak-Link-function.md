---
title:  "Weak symbol in C"
date:   2019-06-21 15:03:55 +0700
categories:
   - Engineering
tags:
   - C
   - Tips n Tricks
---
{: .text-justify}
Sometimes, when we develop the library for the customer, we need to provide the callback/hook functions that prototype that shall be implemented by the customers. However, we need an buildable software environement to do the internal testing, or we want to implemented the default implementation of that callbacks/hooks. The weak symbol is the trick to solve your problem here. By default, the C compiler implicitly mark all the functions as strong symbol (we are unawre of it untill we know that there are such a thing called weak symbol). The strong symbol mean that during the linking phase of the compilation, the symbol resolution takes places and all the symbols (variables and symbols) are assigned addresses. The problem arise when you define 2 functions, that obviously cause the multiple definition error during the final linking phase.

And the multiple definition is what exactly what we do with the callback/hook default implementation. Let's say, our requirement is "The X software platform need to provide a callback function upon the alarm event of the Real-Time Clock.". In the software platform, we will have 
