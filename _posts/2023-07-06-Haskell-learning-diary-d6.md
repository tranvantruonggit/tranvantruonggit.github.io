---
title: "Learning Haskell Diary (Part #6): Category Theory, Maybe, Just, Nothing, and Monad"
categories:
  - Engineering
tags:
  - Learning
  - Haskell
  - Category theory
  - Functional programming
---

# Recap After Half a Year

Been learning Haskell intensively from last year, starting with some fundamental knowledge about category theory. I have been exploring the concepts of functors and monads, particularly focusing on understanding category theory. To aid in my learning journey, I came across a valuable resource: Dr. Bartosz Milewski's YouTube channel. His series of lecture videos on Category Theory for programmers have been immensely helpful. They have not only piqued my interest but also provided me with new perspectives on the programming work I have been doing for years, making me contemplate the quality of my code and the potential bugs that may arise due to the imperative nature of certain languages.

# Some Ideas from Category Theory

## Another Way of Abstraction

Category theory introduces a novel approach to abstraction. It represents things as dots (objects) and arrows (morphisms). A category should exhibit the following characteristics:

- Identity operation: This morphism maps an object to itself. `id x = x`
- Composition: When applying morphism `f` from object `a` to object `b`, and then morphism `g` from object `b` to object `c`, the resulting morphism `g . f` (called g after f) should satisfy `g . f = a -> c`.

# Example(s) of Using Category Theory to Abstract Common Mathematical Characteristics

These are my own interpretations, which may not perfectly align with the theory, but they demonstrate how I perceive the topic.

### The Adder and Number Zero, and/or the Multiplier and Number One

Although addition and multiplication are distinct operations we learn as children, mathematical abstraction reveals similarities between the two. I use the symbol `^` to represent the operator:

- Composition: `(a^b)^c = a^(b^c)`
- Commutative property: `a^b = b^a`
- Identity element: `I` is the identity element, ensuring that for every number, the operation `x ^ I = I ^ x = x`.
- For every operation `f` and `g` in set M, the composition `f x g` is also in set M.

We observe that these characteristics align with the number zero for addition and the number one for multiplication. Category theory addresses such special characteristics in a more abstract manner, referring to them as monoids. The diagram below (image from Wikipedia) depicts a monoid:
![Monoid diagram](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Monoid_unit_svg.svg/726px-Monoid_unit_svg.svg.png)

Initially, this diagram might appear overwhelming, but after watching [Dr. Bartosz's lecture on monoids](https://www.youtube.com/watch?v=GmgoPd7VQ9Q), I gained a better understanding. In general, a monoid is defined within a category with the following morphisms:
- $$\mu$$: M ⊗ M -> M. This represents multiplication or the binary operator, mapping a pair of objects of the same type to an object of the same type.
- $$\eta$$: I -> M. Here, I denotes the unit. Although I haven't fully grasped this concept yet, I accept it for the time being and continue my search for answers.

For this particular category, the following characteristics hold:
- Multiplication: Morphism of a pair of objects to an object. For example, the multiplication of two real numbers yields a real number.
- Associativity: Suppose we have $$\mu$$ ($$\mu$$(x,y),z) = $$\mu$$ (x,$$\mu$$(y,z)). This equation represents the associativity of real number multiplication.
- Identity: We have $$\mu$$ ($$\eta$$ (I), x) = $$\mu$$ (x,$$\eta$$ (I)) = x, denoted as left identity and right identity ($$\lambda$$ and $$\rho$$) in the diagram. For instance, in multiplication, the number **1** serves as the identity element.

## And Then the Monad

Monad? Another new term. I have been struggling to grasp the most fascinating concept of Haskell and Category theory. In fact, I even paused my Haskell coding practice to read and watch numerous articles and videos to gain a better understanding. However, I needed to set aside the concept of category theory temporarily to comprehend monads from an imperative perspective.

Let's consider Haskell, a pure functional language where functions are pure and do not modify state variables or cause side effects such as writing to files, sending data over the internet, or altering registers (I happen to be an embedded software engineer). At this point, a question arises: Why are pure functions essential in functional programming? In Haskell, we do not have expressions like `count = count+1` because they violate the principles of the mathematical universe. Impure functions lead to the collapse of lazy evaluation, thus rendering function composition and eta reduction are inefficient.
![Lazy evaluation collapse](https://link.storjshare.io/s/jug25lvteqn3jo4f6ns7kbyocx7a/imagehosting/photo_2023-07-08_14-48-48.jpg?wrap=0)


Allow me to provide an example illustrating the use of monads. I will not delve into the definition of a monad for now. This explanation draws inspiration from [Ertugrul Söylemez's blog post](https://web.archive.org/web/20120114225257/http://ertes.de/articles/monads.html).

Let's assume we implement an integer square root function that takes an integer and returns its square root. We can represent this function as shown in the following figure:
![Integer square root](https://link.storjshare.io/s/jvlinhdvoxzdbzeatmgph6gsidrq/blogimage/monad/i_sqrt.png?wrap=0)

However, as you know, this function has its limitations. It can only return numeric values, and for certain inputs like 17, an integer root does not exist. How can we address this problem? In C, we solve it by providing a function that returns the validity of the result via another *pass by reference* variable, as shown below:
![Safe square root](https://link.storjshare.io/s/jxikev6oeihdtbjhlokyensc4erq/blogimage/monad/safesqrt.png?wrap=0)

The problem with this approach is evident: the function is impure due to the modification of the global variable `pRet`. So, what's the point? Why do we emphasize the purity of Haskell if it does not simplify our lives? Suppose we want to compute the positive fourth root, which can be expressed as:
$$\sqrt[4]{x} = \sqrt{\sqrt{x}}$$

We can implement the fourth root of an integer using the function depicted in the following block diagram:
![Pure fourth root](https://link.storjshare.io/s/jwap57d37uzedipvsqepfe7o2yka/blogimage/monad/pure_fourthroot.png?wrap=0)

As previously discussed, the square root function presents several problems, which are carried forward when computing the fourth root. So, how can we correctly compose the `sqrt` function? Haskell, and mathematics in general, provide a solution.

Suppose we can define a type that represents whether the function's return value is an integer or `Nothing`. In Haskell, we call this type `Maybe Int`. The function signature for this approach is depicted in the following diagram:
![Maybe square root](https://link.storjshare.io/s/jxlruhm23j34rzyludmzjgde7mza/blogimage/monad/maybe_sqrt.png?wrap=0)

Looking at the diagram, we can simulate a similar approach in C by returning a structure that represents the `Maybe Integer` type. However, what is the advantage? One crucial aspect is that we still pass an integer as the input value, as the square root function operates on integers. Another significant advantage is that by avoiding the use of global variables, the function remains pure and preserves laziness. Moreover, we can use a special composition technique known as bind (`>>=`) operation to compose functions. Let's examine how we compose `i_sqrt` into `fourth_sqrt` in the following figure:
![Monad fourth root](https://link.storjshare.io/s/juk56ofv26cewcdo7bagdbriu5qq/blogimage/monad/monad_fourthroot.png?wrap=0)

Analyzing the diagram, we observe that the fourth root function is implemented by composing two `i_sqrt` functions. The composition is special because it not only passes the output of the first function as the input of the second function (like the dot `(.)` operation in Haskell), but it also handles cases where the first `i_sqrt` function yields `Nothing`. In such cases, `Nothing` is directly forwarded to the output without passing through the second `i_sqrt` box. This behavior aligns with the definition of the bind (`>>=`) function in the Maybe Monad. The Haskell definition is as follows:

```haskell
(>>=) :: Maybe a -> (a -> Maybe b) -> Maybe b
(>>=) m g = case m of
                   Nothing -> Nothing
                   Just x  -> g
```
As we can see, the Monad is precisely defined, serving its purpose. This example highlights a few key benefits of using monads:

- It facilitates function composition in a special and potentially clever way.
- In the i_fourth_sqrt function mentioned above, the intermediate calculation after the first i_sqrt is encapsulated within the bind function. We no longer require global variables to store the state of the first i_sqrt result or any temporary variables. Additionally, we can skip the second i_sqrt block if the immediate result is Nothing, thanks to lazy evaluation.
- Monads aid in abstracting calculations, hiding intricate details and allowing us to focus on high-level computations.
Last- ly, while error handling is not the primary purpose of monads, they provide a useful tool for handling error cases.

To conclude this blog post, let's revisit the original definition of a Monad found in some textbooks:
> A Monad is a monoid in the category of endofunctors.

When we examine what defines a monoid, we find:

- The binary operator (>>=): Composing two functors. It's worth noting that functions can be seen as instances of functors. In the square root example mentioned earlier, the endofunctor maps from the category of integers to the category of integers (the mapping from the category to itself).
- The unit element, which in Haskell is the return function. In the i_sqrt function, the return function of the Maybe monad is Just.
- Associativity of the bind operator.

I hope this example sheds some light on the usefulness of monads. While I've only listed a few benefits here, the true power of monads extends beyond error handling. It allows for clever function composition and abstracts computations, enabling us to focus on higher-level concepts.

That concludes this part of my blog post, as I fear my explanations might become convoluted if I continue.
