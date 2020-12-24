# macro


## using

***just only***
-   Patterns
-   Statements
-   Expressions
-   Items
-   `impl`  Items

***It is ban for list***
-   Identifiers
-   Match arms
-   Struct fields
-   Types  


### First

macro_rules!

```
macro_rules! $name{
...
}
```

```
macro_rules! for {
  ()=>{1+3};
}

//four!()、four![] 或者 four!{}
```

## variable

-   `item`: an item, like a function, struct, module, etc.
-   `block`: a block (i.e. a block of statements and/or an expression, surrounded by braces)
-   `stmt`: a statement
-   `pat`: a pattern
-   `expr`: an expression
-   `ty`: a type
-   `ident`: an identifier
-   `path`: a path (e.g.  `foo`,  `::std::mem::replace`,  `transmute::<_, int>`, …)
-   `meta`: a meta item; the things that go inside  `#[...]`  and  `#![...]`  attributes
-   `tt`: a single token tree

examples:

```
macro_rules! one_expression {
    ($e:expr) => {...};
}

macro_rules! times_five {
    ($e:expr) => {5 * $e};
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
}

```
-   `$`  is a literal dollar token.
-   `( ... )`  is the paren-grouped pattern being repeated.
-   `sep`  is an  _optional_  separator token. Common examples are  `,`, and  `;`.
-   `rep`  is the  _required_  repeat control. Currently, this can be  _either_  
	`*`  (indicating zero or more repeats) or  
	`+`  (indicating one or more repeats). 
	You cannot write "zero or one" or any other more specific counts or ranges.

```
macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}
```

## more

- [a book](https://danielkeep.github.io/tlborm/book/README.html)

- [practical-intro-to-macros](https://danielkeep.github.io/practical-intro-to-macros.html)

