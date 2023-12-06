# InPlace
An implementation of several strongly-normalizing string rewriting systems

# Relog Flavor

```relog
a=Int;List<a>
List<Int>

A<b,C<d>>=A<Int,C<Bool>>;R<b>
R<Int>
```

$$unify \ atom \ \ \frac{⊢unify(A,A)}{⊢A}$$

$$unify \ left \ var \ \ \frac{⊢unify(a,B)}{a↦B⊢B}$$

$$unify \ right \ var \ \ \frac{⊢unify(A,b)}{b↦A⊢A}$$

$$unify \ compound \ \ \frac{⊢unify(A \langle B.. \rangle ,A \langle C.. \rangle)}{⊢A \langle b∈B,c∈C..unify(b,c) \rangle}$$
