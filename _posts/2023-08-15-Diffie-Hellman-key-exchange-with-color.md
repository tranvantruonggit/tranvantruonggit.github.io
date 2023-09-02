---
title: "Diffie-Hell man key exchange with color blending"
categories:
  - Shower thought
tags:
  - Learning
---
Diffie-Hellman key exchange with ... color.

The first time when I learn about key exchange protocol, I came accross an analogy of the protocol using color mixing ([Wikipedia image](https://commons.wikimedia.org/wiki/File:Diffie-Hellman_Key_Exchange.svg)). This analogy is easy to understand, but does it actually work, ignoring the fact that it may be insecure. Let's we find our now

# Diffie-Hellman problem
Let's we define the Diffie hellman problem with color:
* Give `g` to be the generator of the group, in this case we chose the color `g` in the RGBA in the color space.
* Let's define the `^` to be the one way function, that is give 2 color `x` and `y`, we can easily compute the mixed color `m=x^y`, but given the color `m`, it is "hard" to find the color which `m` is mixed from.
* The the DHP in the color space will be: Give 2 color $X$ and $Y$, where as $X=g^x$ and $Y=g^y$, it is hard to find the color the color $Z=(g^x)^y$

It sounds logical, right? Let's take a closer look.

# Color mixing algorithm
For one who is not familiar with graphic processing, color in image can be represent in several way, in this post, I use the RGBA which use Reg-Green-Blue-Alpha element to represent color, and it is represented by a vector of 4 byte. Here are some example:

<h2 style="background-color:rgba(255, 99, 71,1 );">* rgba(255, 99, 71, 255)</h2>
<h2 style="background-color:rgba(255, 99, 71,0.4 );">* rgba(255, 99, 71, 102)</h2>
<h2 style="background-color:rgba(32, 99, 71, 0.4);">* rgba(99, 99, 71, 102)</h2>
<h2 style="background-color:rgba(99, 99, 250, 0.6);">* rgba(255, 99, 250, 102)</h2>

Now we define the mix color operation, there are several one for this, but for the workable DH scheme, we will need the one that has the commutative and associative characteristic.
* Option1 - Addition: C1(r1,g1,b1) ^ C2(r2,g2,b2) = Z(r1+r2, g1+g2,  b1+b2), it seems to match our imagination about the RGB color space, however, it has the problem about the saturation, our information will be lost. For instance, if one of the color in mixing is white (FFFFFF), the result is always wh/ite due to saturation.
* Option2: average. On the surface, it is commutative, but it doesn't provide associative. So I wanna make a modification for it.
$F$ is the mix funcion which take the list of colors to produce the new color.
$F(color1)$ = $color1$
$F(color1, color2)$ = $(color1+color2)/2$
$F(color1, color2, color3)$ = $(color1+color2+color3)/3$ = $F(color1,color2)*2/3 + F(color3)*1/3$
* Option3 - multiply blend: This is a common blend mode, the special effect of this mode is that the result is alway a color darker than 2 input colors. Here is the blend function for it
$F(a,b) = a.b$ where as the result in RGB-888 notation is new $Z$ color is  $Rz = Ra.Rb/255; Gz = Ga.Gb/255; Bz=Ba.Bb/255$

Let's see the illustration of the color blending in different modes.

| Mode      | Description |
| ----------- | ----------- |
| Addition      | Title       |
| Average   | Text        |
| Multiply | a |

Based on the result of the blending, we can easily see addictive blending doesn't fit with the realtiy of paint mixing, because the more we mix, the lighter it is, but it shall be darker in reality.
# Try out the key exchange with color blending

## Addictive blending
g color is tbd
Alice chooses private color x, and produce public colox g^x
Bob chooses private color y and produce public color g^y
Alice caculate the shared secret color Z = g^y^x = 
Bob calculate share secret color Zb= TBD

## Average blending

## Multiply blending

# Trivia security evaluation
We call it trivial because all above color blending mode are not truely one way function mathematically, because the private key are easyli calculated from g color and the public key:
* addictive blending
* average blending
* Muliply blending

Thus, we only evaluate the security properties of the scheme bases on the color visualization. Let's see the following analysis:

* Addiction blending: we can easily see that this scheme work only if the g is near the dark color to avoid the saturation, at the same time, the private key should be chosen that it shall not cause the saturation. With that said, the result of the Color DH is always brighter color than the g and the public key itself, thus the attacker can just mix gx and gy, and brute force the color from g^x^y untill the g^x

* Average blending: Well, I bieve Average blending is the most accurate interm of color visualization, because it reflect the way we mix paint in real life. The share secret it this case is a bit harder to guess, because it can be brighter and darker than the public colors.

* Multiply blending: This scheme is the oposite of the Addiction blending, where it need the `g` color to be near the while color, and the result of the blending is always the darker color. Though this scheme does not cause much of the saturation problem (unless one of the serecet color is completely black - represent by `0` value), it does not represent the real color blending in reality. For instance, if we mix 2 color of the same color, the result shall be the same color in real life, but with this multiply color bleanding, we get the darker color.

