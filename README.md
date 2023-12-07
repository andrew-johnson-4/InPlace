# InPlace
An implementation of several strongly-normalizing string rewriting systems

# Relog Flavor

```relog
a=Int;List<a>
List<Int>

A<b,C<d>>=A<Int,C<Bool>>;R<b>
R<Int>

A<b,c>:=R<c>;A<B,C>
R<C>

Print<"hello world">
[stdout]hello world
0

For<n,99,0,
   Print<"{n} bottles of beer on the wall
        \r{n} bottles of beer
        \rTake one down, pass it around
        \r{n} bottles of beer on the wall">
>
[stdout]99 bottles of beer on the wall
...
0
```

![Reduction](https://github.com/andrew-johnson-4/InPlace/blob/main/unifyreify.png)

$$apply \ \ \frac{unify(f,fx)⊢r}{f⇻r⊢reify(apply(r))}$$
