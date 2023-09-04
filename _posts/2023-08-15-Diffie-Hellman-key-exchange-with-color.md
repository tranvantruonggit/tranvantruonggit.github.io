---
title: "Diffie-Hellman key exchange with color blending"
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
* The the DHP in the color space will be: Give 2 color $X$ and $Y$, where as $X=g^x$ and $Y=g^y$, it is hard to find the color the color $$Z=(g^x)^y$$

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
  $$F$$ is the mix funcion which take the list of colors to produce the new color.

  $$F(color1)$$ = $$color1$$
  
  $$F(color1, color2)$$ = $$(color1+color2)/2$$
  
  $$F(color1, color2, color3)$$ = $$(color1+color2+color3)/3$$ = $$F(color1,color2)*2/3 + F(color3)*1/3$$
  
* Option3 - multiply blend: This is a common blend mode, the special effect of this mode is that the result is alway a color darker than 2 input colors. Here is the blend function for it
  $F(a,b) = a.b$ where as the result in RGB-888 notation is new $Z$ color is  $Rz = Ra.Rb/255; Gz = Ga.Gb/255; Bz=Ba.Bb/255$

Let's see the illustration of the color blending in different modes.

| Mode     | Example                                                                                                         |
| -------- | --------------------------------------------------------------------------------------------------------------- |
| Addition | ![](https://link.storjshare.io/s/jwrwdsivuxf7px5e66djn4vcrxcq/blogimage/DHwithColor/AddictiveBLend.png?wrap=0)  |
| Average  | ![](https://link.storjshare.io/s/jxinqesm5lkudhcmknoortkggbsa/blogimage/DHwithColor/AverageBlending.png?wrap=0) |
| Multiply | ![](https://link.storjshare.io/s/jvjfpe6we2yr2zdntnlhywkhb4la/blogimage/DHwithColor/Multiply.png?wrap=0)        |

Based on the result of the blending, we can easily see addictive blending doesn't fit with the realtiy of paint mixing, because the more we mix, the lighter it is, but it shall be darker in reality. However, we will accept it for the DH key exchange, just for reference.

# Try out the key exchange with color blending

Let's make the key exchange between Alice and Bob, as the following table.
<style type="text/css">
.tg  {border-collapse:collapse;border-spacing:0;}
.tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
  overflow:hidden;padding:10px 5px;word-break:normal;}
.tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
  font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
.tg .tg-3v66{background-color:#0f0f0f;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-bt2x{background-color:#10f0c1;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-gg2y{background-color:#0fb002;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-u75l{background-color:#90ffff;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-nlez{background-color:#aae291;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-7eyh{background-color:#80f0d9;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-pl4m{background-color:#ffff82;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-lkdj{background-color:#0f0c00;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-w9rw{background-color:#876b09;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-8hba{background-color:#48b8a1;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-vs2s{background-color:#fffff2;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-a5yt{background-color:#609846;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-pgyx{background-color:#abe391;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-zop7{background-color:#0fb001;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-tyl5{background-color:#0fe2b6;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-3ote{background-color:#ffffd2;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-ad4u{background-color:#ffd611;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-fixl{background-color:#806401;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-0lax{text-align:left;vertical-align:top}
.tg .tg-ymsu{background-color:#f0f0f0;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-7fqw{background-color:#808080;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-bweo{background-color:#ffffff;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-03ip{background-color:#1fffd0;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-io95{background-color:#108068;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-r1pw{background-color:#087861;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-9kc8{background-color:#010e0b;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-t5fq{background-color:#ffc702;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-ay8z{background-color:#f8dc79;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-3dfx{background-color:#c0a441;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-wpuk{background-color:#f0bb02;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-juaj{background-color:#85bc6c;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-qag1{background-color:#5f9746;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-4d6x{background-color:#085e01;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
.tg .tg-srmr{background-color:#010b00;font-family:"Courier New", Courier, monospace !important;text-align:left;vertical-align:top}
</style>
<table class="tg">
<thead>
  <tr>
    <th class="tg-0lax"> </th>
    <th class="tg-0lax" colspan="3">Addictive</th>
    <th class="tg-0lax" colspan="3">Average</th>
    <th class="tg-0lax" colspan="3">Multiply</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td class="tg-0lax"> </td>
    <td class="tg-0lax">Near white</td>
    <td class="tg-0lax">Mid point</td>
    <td class="tg-0lax">Near black</td>
    <td class="tg-0lax">Near white</td>
    <td class="tg-0lax">Mid point</td>
    <td class="tg-0lax">Near black</td>
    <td class="tg-0lax">Near white</td>
    <td class="tg-0lax">Mid point</td>
    <td class="tg-0lax">Near black</td>
  </tr>
  <tr>
    <td class="tg-0lax">Common color G</td>
    <td class="tg-ymsu">F0F0F0</td>
    <td class="tg-7fqw">808080</td>
    <td class="tg-3v66">0F0F0F</td>
    <td class="tg-ymsu">F0F0F0</td>
    <td class="tg-7fqw">808080</td>
    <td class="tg-3v66">0F0F0F</td>
    <td class="tg-ymsu">F0F0F0</td>
    <td class="tg-7fqw">808080</td>
    <td class="tg-3v66">0F0F0F</td>
  </tr>
  <tr>
    <td class="tg-0lax">Alice's private color</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
    <td class="tg-bt2x">10F0C1</td>
  </tr>
  <tr>
    <td class="tg-0lax">Alice's public color</td>
    <td class="tg-bweo">FFFFFF</td>
    <td class="tg-u75l">90FFFF</td>
    <td class="tg-03ip">1FFFD0</td>
    <td class="tg-7eyh">80F0D9</td>
    <td class="tg-8hba">48B8A1</td>
    <td class="tg-io95">108068</td>
    <td class="tg-tyl5">0FE2B6</td>
    <td class="tg-r1pw">087861</td>
    <td class="tg-9kc8">010E0B</td>
  </tr>
  <tr>
    <td class="tg-0lax">Bob's private color</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
    <td class="tg-t5fq">FFC702</td>
  </tr>
  <tr>
    <td class="tg-0lax">Bob's public color</td>
    <td class="tg-vs2s">FFFFF2</td>
    <td class="tg-pl4m">FFFF82</td>
    <td class="tg-ad4u">FFD611</td>
    <td class="tg-ay8z">F8DC79</td>
    <td class="tg-3dfx">C0A441</td>
    <td class="tg-w9rw">876B09</td>
    <td class="tg-wpuk">F0BB02</td>
    <td class="tg-fixl">806401</td>
    <td class="tg-lkdj">0F0C00</td>
  </tr>
  <tr>
    <td class="tg-0lax">Alice's share secret</td>
    <td class="tg-bweo">FFFFFF</td>
    <td class="tg-bweo">FFFFFF</td>
    <td class="tg-3ote">FFFFD2</td>
    <td class="tg-pgyx">ABE391</td>
    <td class="tg-juaj">85BD6C</td>
    <td class="tg-qag1">5F9746</td>
    <td class="tg-gg2y">0FB002</td>
    <td class="tg-4d6x">085E01</td>
    <td class="tg-srmr">010B00</td>
  </tr>
  <tr>
    <td class="tg-0lax">Bob's share secret</td>
    <td class="tg-bweo">FFFFFF</td>
    <td class="tg-bweo">FFFFFF</td>
    <td class="tg-3ote">FFFFD2</td>
    <td class="tg-nlez">AAE291</td>
    <td class="tg-juaj">85BD6C</td>
    <td class="tg-a5yt">609846</td>
    <td class="tg-zop7">0FB001</td>
    <td class="tg-4d6x">085E01</td>
    <td class="tg-srmr">010B00</td>
  </tr>
</tbody>
</table>


# Trivial security evaluation

We call it trivial because all above color blending mode are not truely one way function mathematically, because the private key are easyli calculated from g color and the public key:

* addictive blending
* average blending
* Muliply blending

Thus, we only evaluate the security properties of the scheme bases on the color visualization. Let's see the following analysis:

* Addiction blending: we can easily see that this scheme work only if the g is near the dark color to avoid the saturation, at the same time, the private key should be chosen that it shall not cause the saturation. With that said, the result of the Color DH is always brighter color than the g and the public key itself, thus the attacker can just mix gx and gy, and brute force the color from g^x^y untill the g^x

* Average blending: Well, I bieve Average blending is the most accurate interm of color visualization, because it reflect the way we mix paint in real life. The share secret it this case is a bit harder to guess, because it can be brighter and darker than the public colors.

* Multiply blending: This scheme is the oposite of the Addiction blending, where it need the `g` color to be near the while color, and the result of the blending is always the darker color. Though this scheme does not cause much of the saturation problem (unless one of the serecet color is completely black - represent by `0` value), it does not represent the real color blending in reality. For instance, if we mix 2 color of the same color, the result shall be the same color in real life, but with this multiply color bleanding, we get the darker color.
