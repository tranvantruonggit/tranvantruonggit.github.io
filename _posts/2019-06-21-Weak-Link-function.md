---
title:  "Using weak symbol in C to define default call-back in library"
date:   2019-06-21 15:03:55 +0700
categories:
   - Engineering
tags:
   - C
   - Tips n Tricks
---
# Background
Sometimes, when we develop the library for the customer, we need to provide the callback/hook functions that prototype that shall be implemented by the customers. However, we need an buildable software environement to do the internal testing, or we want to implemented the default implementation of that callbacks/hooks. The weak symbol is the trick to solve your problem here. By default, the C compiler implicitly mark all the functions as strong symbol (we are unawre of it untill we know that there are such a thing called weak symbol). The strong symbol mean that during the linking phase of the compilation, the symbol resolution takes places and all the symbols (variables and symbols) are assigned addresses. The problem arise when you define 2 functions, that obviously cause the multiple definition error during the final linking phase.

# Mulitple definition
And the multiple definition is what exactly what we do with the callback/hook default implementation. Let's say, our requirement is "The X software platform need to provide a callback function upon the event Y.". With this requirement, we will provie the callback prototype that will be defined by the customer.
```c
extern void callback_function_Y(void);
```
Normally, we can get home with that function, however, when it come to the implicit requirement that the delivered software shall be compilable. Then, if we define this function in side our libary, and when the customer define their own implementation, the will be error at linking phase `multiple definitions of callback_function_Y`. Any compiler that is based on GCC supports the `__attribute--((weak))__` to define a symbol (whether function or variable) to be weak.

# Example code
Firstly, we have the library with implementation of the default callback function.

```c 
/* main.c */
#include <stdio.h>
#include "callback.h"

int main( void )
{
	printf("Hello world\n");
	callback_function_Y();
}

void callback_function_Y(void)
{
	printf("This is default implementation of the callback function\n");
}

```
```c
/* callback.h */
#ifndef CALLBACK_H__
#define CALLBACK_H__
extern void callback_function_Y(void);

#endif /* CALLBACK_H__ */

```
<br>
Then build and run it
```sh
$ gcc main.c
$ chmod +x a.out
$ ./a.out
Hello world
This is default implementation of the callback function.
```

Next, when we deliver the software to the customer, as we are developing the platform software, they won't touch to our code, so they just include the header file and define callback function.
```c
/* user_application.c */
#include "callback.h"
#include <stdio.h>


void callback_function_Y(void)
{
	printf("**Note**: This is the user implementation of the callback");
}


```
<br>
Next, the customer build the application and get the linker error.
```sh
$ gcc main.c user_application.c
/usr/bin/ld: /tmp/ccO7uPAE.o: in function `callback_function_Y':
user_application.c:(.text+0x0): multiple definition of `callback_function_Y'; /tmp/cca28g2A.o:main.c:(.text+0x1f): first defined here
collect2: error: ld returned 1 exit status
$
```
<br>
Now if we apply the weak link symbol technique to our default implementation, our main.c (the platform's source code) will be like below snipet. Be notice to the __attribute__((weak)) when we define the callback.
```c
/* main.c */
#include <stdio.h>
#include "callback.h"

int main( void )
{
	printf("Hello world\n");
	callback_function_Y();
}

void __attribute__((weak)) callback_function_Y(void)
{
	printf("This is default implementation of the callback function.\n");
}

```
<br>
Then build and run to see the result.
```sh
$ gcc main.c user_application.c
$ ./a.out
Hello world
**Note**: This is the user implementation of the callback 
$
```
<br>
So now, let we use the `nm` utility of gcc to check the different of the function between versions, with and without user application.
```sh
$ gcc main.c&& nm a.out | grep callback_function_Y
0000000000001158 W callback_function_Y
$ gcc main.c user_application.c && nm a.out | grep callback_function_Y
000000000000117e T callback_function_Y
$
```
We can clearly see that when we build the application without the user application code, the callback function has the W symbol type, meaning it is **weak** symbol. With this, we can even pack the software platform in to static libray that contain the default implementation of a hook, so if the application don't define this hook, the default implemenattion will be used. Let we check it.
```sh
# The below command compile the main.c without linking in to the main.c, then using the `ar` utility to pack into static library libmain.a
$ gcc -c main.c && ar rcs libmain.a main.o 
$ nm libmain.a
main.o:
000000000000001f W callback_function_Y
                 U _GLOBAL_OFFSET_TABLE_
0000000000000000 T main
                 U puts

# We can see the output of NM indicate the the callback function in library is **weak** symbol
# Now we link with the appliaction, -l main is the convention of gcc to link with the libmain.a, -L. mean that the current directoly is also in the library search path.
$ gcc user_application.c -l main -L.
# then run it 
$ ./a.out
Hello world
**Note**: This is the user implementation of the callback%
```
That's it, the weak symbol is very helpful when we want to define the default implementation that is normally defined by the user, but we also want to provide a default implementation to make the deliver software to be compilable out of the box or provide an example implementation. Please be noted that the weak symbol can be applied to variable as well.

--- End ---
