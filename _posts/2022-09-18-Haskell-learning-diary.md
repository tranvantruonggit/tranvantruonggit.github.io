---

title:  "Learning haskell diary"
categories:

- Engineering
  tags:
- Learning
- Haskell

---

{:.text-justify}
This post is just my personal note when I learn haskell. It started on 2020 during COVID quarantine period. I have suspended it for a bit then start over again. This diary is based on [Yann's writting](https://yannesposito.com/Scratch/en/blog/Haskell-the-Hard-Way/#navigation).

# Some basic **NEW** concept

- Functional: it means function as in the mathematic.
- Pure: A function in FP is pure if it doesn't change the program state. Which means feeding function the same params will always produce the same result. It is somewhat equivalent to the term **reentrancy** in the imperative programming.
- Lazyness: mean optimization by compiler, in traditional compiler, if the compiler figure out that some code is never called, it may not emits any implementation of that code. In FP, lazyness mean the function is not evaluate untill the compiler see it is needed to evaluate. For instance, calling the function to get the Fibonacy number with the argument of 0 or 1, the result will be 0 or 1 immediately, without futher recursive function invoking.

# Getting started

## Function declaration in Haskell

```haskell
-- Description about the function type
f :: Int -> Int -> Int
-- This is function declaration
f x y = (x*x) + (y*y)
```

It is close with function in algebra. That is function is some transformation from the input to output. That is, no parentheses, no return statement. The first line  descripte the type of the function that `f` is a function return `Int` and take 2 `Int` arguments. Pretty neat! However, Haskell doesn't have function with 2 arguments, the first line of code mean `f` is a function that take `Int` arugment and convert it to a function convert from `Int` to `Int` type. **Mind blowing?** Yes, I do. I dont know either.

The type declaration is just a way to limit what `f` can take. It is not necessary to define the type restriction there since haskell would decide which is the best type to use for `f`, otherwise, we need to defines different function for different argument type.



## Some notation

```haskell
adder a b = a+b -- infix notation
adder_pre a b = (+) a b -- prefix notation


-- Some reduction
-- η-reduction
square x = (^) 2 x
-- equivalent to the following line (reduce x from both side of the equation)
square' = (^2) -- Yes haskell accept "'" symbol in function name
```

At this point, I feeling learning algebra. (?!)

## Example 1: Sum of series

We don't use loop in haskell. The loop is replaced by recursion. **WHAT???** Yeaah, don't bother about stack space in Haskell, it (or FP language) handle recursion pretty efficient (I am not sure haha). Let's take a look of the recursive implementation of the sum function that sum the number of series

```haskell
sum_series list = do
   if list == []
   then 0
   else
      head list +sum_series (tail list)
```

In the above snippet of code, we learn that Haskell is sensitive with indentation, I also learn that `if then else` in haskell is similar to ternary operation in C `<expression>?<True value>:<False value>` .

Futhermore, for list handling, the `head` take the first element of list, `tail` take the rest of the list (remove the head element).

The above function seems right, however, they rarely did that in FP, they will write like this. Like define the initial statement for recursion.

```haskell
sum_series [] = 0
sum_series list = head list + sum_series (tail list)
```

or even simpler

```haskell
sum_series [] = 0
sum_series (x:xs) = x + sum_series xs
```

## Example 1.1 sum of aligned-4 number in series

Let's add some more complexity in the **Example 1**. Now we sum only the number that hat modulo of 4 is zero. Here are my solution.

```haskell
sum_series_aln4_check x = if x`mod`4 == 0 then x else 0
sum_series_aln4 [] = 0
sum_series_aln4 (x:xs) = sum_series_aln4_check x + sum_series_aln4 xs
```

This can be rewrite using `where` or `let` keyword not to emit the  sum_series_aln4_check

```haskell
sum_series_aln4v2 [] = 0
sum_series_aln4v2 (x:xs) = sum_series_aln4_check x + sum_series_aln4 xs
   where sum_series_aln4_check x = 
            if x`mod`4 == 0 then x else 0
```

I have heard something about `let` keyword, but I haven't known how to write this example using `let` yet haha.

However, the above implementation doesn't look too iterative. So I rewrite it for later learning.

```haskell

sum_align4 = accumulate 0   -- here we reduced the list argument
   where accumulate n [] = n   
         accumulate n (x:xs) = 
            if x`mod`4 == 0 
            then accumulate (n+x) xs
            else accumulate (n) xs         
```

The above implementation just mean

## Higher order function

Higher order function is the function that take function as an input. For instance

the `filter` function in haskell which take function that convert type to boolean) and alist and return a list.  **What The Hell** !?

```haskell
filter :: (a -> Bool) -> [a] -> [a]
```

For instance, the following lines filler only number that are aligned-4

```haskell
aln4_check x = if x`mod`4 == 0 then True else False
filter aln4_check [1..100]
```

Now, we move to the most powerful higher order function, the most evil function 

### foldl

**fold left**, it reduce the following snippet in to simpler form.

```haskell
sum_align4 list= accumulate 0  list
isAligned4 x = if x`mod`4 == 0 then True else False
accumulate n [] = n   
accumulate n (x:xs) = 
   if( isAligned4 x)
            then accumulate (n+x) xs
            else accumulate (n) xs
-- Will become

aln4_check x = if x`mod`4 == 0 then True else False

sum_align4_v2 list = foldl mysum 0  (filter  aln4_check list)
   where mysum n x=  n + x
```

And my brain is dumbed now. I wil  try to explain it to myself (like having a duck)

* The `foldl` function work with the series of data (a `list`).

* The initial value of the whole operation is zero (the 2nd param after `foldl`). I above function the number 0 is also the `n` param of the function `mysum`. 

* The 1st param is the function work on the current state of the operation and take the head of the list (the 3rd param after `foldl`).

* It iterates until the end of the list.

At this point Yann have introduce the lambda expression at in finest.the the function can be rewriten by the following.

```haskell
aln4_check x = if x`mod`4 == 0 then True else False

sum_align4_v3 list = foldl (\x y -> (+) x y) 0 (filter  aln4_check list)
-- which then can be eta-reduced 
sum_align4_v4 list = foldl (+) 0 (filter  aln4_check list)

```


