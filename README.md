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

## Relog Syntax

Relog is a type system, not a programming language. It is not Turing Complete, it is Strongly Normalizing.

The basic syntax of a Relog program is defined by unifications, followed by a result.

Unifications have a left-hand-side and a right-hand-side separated by an equal sign with a semicolon after each unification:

```
Pair<a,b> = Pair<Int,Int>;
```

Results are a single term and can reference unification variables bound by previous actions:

```
Pair<a,a>
```

An example program might represent the application of a function `a -> Maybe<a>` with argument `Int` which would look like below:

```
a=Int;Maybe<a>
```

when running this program the unification will bind `Int` to `a` and return the result `Maybe<Int>`.

## Relog Type System

![Reduction](https://github.com/andrew-johnson-4/InPlace/blob/main/unifyreify.png)

$$apply \ \ \frac{unify(f,fx)⊢r}{f⇻r⊢reify(apply(r))}$$
