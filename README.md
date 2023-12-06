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
```

![Reduction](https://github.com/andrew-johnson-4/InPlace/blob/main/unifyreify.png)

$$apply \ \ \frac{unify(f,fx)⊢r}{f⇻r⊢reify(r)}$$
