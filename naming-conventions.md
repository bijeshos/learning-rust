# Naming Conventions
## Basic

| Item | Convention |
| ---- | ---------- |
| Crates | `snake_case` (but prefer single word) |
| Modules | `snake_case` |
| Types | `UpperCamelCase` |
| Traits | `UpperCamelCase` |
| Enum variants | `UpperCamelCase` |
| Functions | `snake_case` |
| Methods | `snake_case` |
| General constructors | `new` or `with_more_details` |
| Conversion constructors | `from_some_other_type` |
| Local variables | `snake_case` |
| Static variables | `SCREAMING_SNAKE_CASE` |
| Constant variables | `SCREAMING_SNAKE_CASE` |
| Type parameters | concise `UpperCamelCase`, usually single uppercase letter: `T` |
| Lifetimes | short, lowercase: `'a` |

## Conversions

Conversions should be provided as methods, with names prefixed as follows:

|Prefix|	Cost	|Ownership|
|--|--|--|
|as_	|Free	|borrowed -> borrowed|
|to_	|Expensive	|<ul><li> borrowed -> borrowed</li><li> borrowed -> owned (non-Copy types)</li><li>owned -> owned (Copy types) </li> </ul>|
|into_	|Variable	|owned -> owned (non-Copy types)|

- Examples
    - str::as_bytes()
    - Path::to_str
    - String::into_bytes() 
## Getter names
- With a few exceptions, the `get_` prefix is not used for getters in Rust code.
- The `get` naming is used only when there is a single and obvious thing that could reasonably be gotten by a getter

## Others
- Methods on collections that produce iterators follow iter, iter_mut, into_iter
- Iterator type names match the methods that produce them
  - e.g.:
    - Vec::iter
    - Vec::iter_mut
    - Vec::into_iter
- Iterator type names match the methods that produce them
    - e.g.:
        - `Vec::iter` returns `Iter`
        - `Vec::iter_mut` returns `IterMut`
        - `BTreeMap::keys` returns `Keys`
        - `BTreeMap::values` returns `Values`
- Error names
    - use `verb-object-error` word order
        - e.g.: 
            - `JoinPathsError`
            - `ParseBoolError`
            - `ParseCharError`

## Reference

Rust API Guidelines: https://rust-lang.github.io/api-guidelines/naming.html#c-case