---
title:  "Learning haskell diary (part #6): category theory, Maybe, Just, Nothing and Monad"
categories:
   - Engineering
tags:
   - Learning
   - Haskell
   - Category theory
   - Functional programming
---
# Recap after half year
Been learing haskell intensively from last year, starting with some fundamental knowledge about caterogy theory. I am stuck with the understanding of the functor, monad. So I started to investigating about the **Category Theory**. Thankfully, I have found a youtube channel of the Dr. Bartosz Milewski which has series of lecture videos about Category Theory for programmer, I have been interested in some concept of it. Moreover, learing about category theory provides me some new perspective about programming I have been doing for years. It makes me think about my quality of works, about the potential bug I may have been make in my code (because the nature of imperative language).
# Some idea of the Category theory
## Another way of the abstraction
Being introduce to category theory as a mean of abstracting things. In the category theory, things are represented by dots (objects) and arrows (morphism). The category shall fulfill the following characteristic:
* Identity operation: is the morphism from one object to itself. `id x = x`
* composition: applying morphism `f` from `a` to be `b`, then morphism `g` from `b` to `c`. We have the new morphism `g . f` (called g after f) that `g . f = a -> c`.

# Example(s) of using category theory to abstract common mathernmatic characteristic.
These are just my understanding, it is not necessarily perfectly correct to the theory, but it is how I view the topic.
### The adder and number zero, and/or the multiplier and number 1.
Those the adder and multiplier is different operations that we have learnt when we was a kind, the abstraction of the mathermatic find the similarity of these 2 operation, here I used the sumbol `^` to denote the operator:
* The composition `(a^b)^c = a^(b^c)`
* Commutative `a^b = b^a`
* It has one identity element `I` such tha for every number, the operation `x ^ I = I ^ x = x` .
* For every operation f, g in set M, the composition `f x g` is in set M as well.
Then we will see that for adder, all above characteristic fulfill with the number zero. Similarly, the multiplication with number 1 also fulfill. The category theory address these kind of special characteristic in the much more abstracted way. This is called monoid in category theory and it can be described as a diagram like below (image from wikipedia):
![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Monoid_unit_svg.svg/726px-Monoid_unit_svg.svg.png)
Looking at the diagram, ... well at the moment I feel as a dump person. However, after watching [Dr. Bartozs's lecture on the monoid](https://www.youtube.com/watch?v=GmgoPd7VQ9Q), I think I can understand now.
In general, the monoid is defined in the category with the following morphism:
* $\mu$ : M ⊗ M -> M . This is called the multiplication or binary operator, which maps a pair of object of the same type to the object of the same type
* $\eta$: I -> M . I is unit. and ... honestly I don't quite understand this, but I can just accepts it for the time being (I am still finding the answer)
Then, for this caterogy, we have the following characteristic:
* Multiplication: morphisim of a pair of object to an object. For instance, the multiplication of 2 real numbers is a real number.
* Associative: let's say, we have $\mu$ ($\mu$(x,y),z) = $\mu$ (x,$\mu$(y,z)). For instance, the associative of real number multiplication.
* Identitty: we will have $\mu$ ($\eta$ (I), x) = $\mu$ (x,$\eta$ (I)) = x, it is called left identity and right identity. Donoted by $\lambda$ and $\rho$ in above diagram. For eg: it is the number **1** in the multiplication.
## And then the Monad
WHAT, MONAD? another new term. I have been struggling to grasp the most **fancy** concept of the Haskell and Category theory, in fact, I have been stopping practicing the haskell code to read and watch numberous of article and video to get the idea, and I need to throw out the category from my head to understand the monad in the imperative aspect.
Let's say Haskell is a pure functional language, which means all of it function is pure, they don't change the state variable, they don't cause side effect. Then what is the point of making the function that doesn't cause any side effect such as writing to file, sending the data over internet... and of course, modify an register (haha, I am an embedded software engineer). Now, we will need to understanding that why pure function is important in functional programming? You know, in haskell, we don't have something like `count = count+1` because it violates prinicple of the math universe.
![https://link.storjshare.io/s/jug25lvteqn3jo4f6ns7kbyocx7a/imagehosting/photo_2023-07-08_14-48-48.jpg?wrap=0](https://link.storjshare.io/s/jug25lvteqn3jo4f6ns7kbyocx7a/imagehosting/photo_2023-07-08_14-48-48.jpg?wrap=0)
If the function is impure, the lazy evaluation collapse, thus it will be very inefficient to do the function composition and also the eta reduction.

Let me take an example how we can use monad, I don't explain what the monad is for the moment. This explaination is inspired by [Ertugrul Söylemez blog post](https://web.archive.org/web/20120114225257/http://ertes.de/articles/monads.html).

Say we implement the integer sqrt function which take an interger and return the square root of the input. This function will be represent like the following figure.

As you know the limitation of this function is that it can return only numeric value, we say number 17 doesn't have the integer root. Then how can we solve this problem? Well, in C we solve this problem by providing the function that return the validity of the result via another *pass by reference* variable. Let's see in the below figure.

The obviously problem with this function is that it is impure, as global variable point by pRet being modified! Then what is the point? Why do even need the pureness of the Haskell if it cannot make my life easier? Well, let's say, if we want to make a function that find the possitive fourth root, it is easyly to know that :
$$\sqrt[4]{x} = \sqrt{\sqrt{x}} $$

then we can implement the fourth root of an integer by the function in the following block diagram.

As we discussed above, we have several problem with the square root function it self, and when using it to make the fourth root, it lifts the problem to another question. How can we compose sqrt function correctly. Haskell, or more general mathermatics, has a way to do it.
Let's say if we can some how define a type that can represent the whether the the return value of the function is integer or Nothing, we can call it `Maybe Int`. The function sigature of this function in haskell is depicted in the following diagam.

Well, looking in the diagram, we can mimic the same thing in C by returning the structure that represent the Maybe Integer type, so what's the point. One more point is that we still have the input value as Integer, because we know the function sqrt has the domain (the set of the input) being the integer. Another point is that, since we don't intruduce the global variable, we keep that function to be pure, and it still keep the lazyness of the function. And finally, it have a special way to compose it to make the fourth root, it is the bind (`>>=`) operation. Let's see how we compose the i_sqrt into fourth_sqrt in the following figure.

Looking at above diagram, we will see that the fourtth root function is implemented by composing 2 `i_sqrts` functions, and the way it is composed is kind of special because it does not just feeding the output of this function to the input of the another function (like the dot `(.)`operation in Haskell), but it also process the case where the first i_sqrt function being `Nothing`, the Nothing will be forward directly to the output (not thought the second `i_sqrt` box). That is exactly how the bind (`>>=`) function of the Maybe Monad is defined. Let's see below:
```(haskell)
 (>>=) :: Maybe a -> (a -> Maybe b) -> Maybe b
 (>>=) m g = case m of
                   Nothing -> Nothing
                   Just x  -> g
```

We can see, the monad is exactly what it is defined, no less, no more, but I hope this example gives you one example of how Monad is useful, I can list a few here:
* It helps us to compose the function in special (and may be clever) way.
* If we looking on the `i_fourth_sqrt` function above, we will see that the immediate calculation after the first `i_sqrt` is wrapped within the bind function, we don't need the global variable to store the state of the first `i_sqrt` result, nor need any temporary variables. We also ignore the second `i_sqrt` block incase the first the immediate result is `Nothing`. This is because of the lazy evaluation.
* It helps to abstract our calculation, it hides the details calculation insides, but we will focus on the high level calucalation.
* Last but not least, handling the error case is not why the Monad is invented, it is just an use case where Monad can be used.
To finish this blog post, I would like to come back to the original definition of the Monad in some text book:
>Monad is the monoid in the category of the endo functors.

Well, looking at what define the monoid, we will have:
* the binary operator `(>>=)`: Which compose 2 functors. It worths nothing that function can be seen as an instance of the functor. In the square roots example above, the endofunctor is the mapping from category of the integer to the category of the integer (the mapping from category to itself).
* The unit element, in haskell it is the `return` function. In the `i_sqrt` above, it is the return function of the Maybe monad, and is `Just` function.
* They have associativity of the bind operator.
I think that is enough (for me) for this part so I will stop here not blow my own mind.