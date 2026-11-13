programming language which uses indexes bound to specific arrays instead of references.
An earlier attempt at enabling this pattern using linear types was [sloe](http://codeberg.org/lue-bird/sloe).

```slug
fn greet<Chars>(name str) span(Chars char)
    add-str-chars<Chars>("Hello, "str)
    .span-add-str-chars(name)
    .span-add("!"char)
```

install with (requires having [rust installed](https://rust-lang.org/tools/install/))
```bash
cargo install --git https://codeberg.org/lue-bird/slug-lang slug
```

[skip to syntax overview](#syntax)

## 🧩 each slot/span has one associated collection
(TODO rewrite to be more approachable)

```slug
ty type-alias(Types Names Source)
    {
    parameters opt(span(Names span(Source char)))
    type type(Types Names Source)
    }

ty type(Types Names Source)
    [
    Variable(span(Source char))
    Tuple(opt(span(Types (type Types))))
    Fn(
        {
        parameters opt(span(Types (type Types)))
        result slot(Types (type Types))
        }
    )
    Alias(
        {
        name span(Source char)
        arguments opt(span(Types (type Types)))
        }
    )
    Record(
        {
        names opt(span(Names (span Source char)))
        values opt(span(Types (type(Types Names Source))))
        }
    )
    ]
```
The rule to make it all safe: you can never have multiple collections that have the same item and origin type in reach.

Use nested bufs instead of one big one to operate on one chunk in a clear way, e.g. pre-allocating, altering, reusing memory, stepping, quick bulk deletion / bulk access / bulk modification without needing to reach into _all_ items of the same kind.
```slug
ty project-state
    {
    type-aliases opt(span([TypeAliases] type-alias([Types] [Names] [Source])))
    
    source buf([Source] char)
    names buf([Names] span([Source] char))
    types buf([Types] type([Types] [Names] [Source]))
    type-aliases-buf buf([TypeAliases] type-alias([Types] [Names] [Source]))
    }

ty state
# multiple open workspaces which have independent state
    {
    projects opt(span([Project]))
    projects-buf buf([Project] project-state)
    }

fn initial-state() state
    {
    projects No<opt(span([Project] project-state))>
    projects-buf buf-empty<[Project] project-state>()
    }
```

## syntax
```slug
# line comment

# number (available types: p32, u32, i32, f32).
# Specifying a type is required
3.2f32

# text of type str
"hello"str

# unicode scalar of type char
"a"char

# identifiers. All except type variable and variant names start lowercase
variable-or-function-or-field-or-variant-or-type-2012

# function call. arguments are separated by space
some-function(first-argument second-argument)

# syntax sugar for some-function(first-argument second)
first-argument.some-function(second)

# some functions require type arguments in angle brackets before the (arguments).
# Function declarations will explicitly list those <Parameters> (see later)
some-function<type arguments>(arguments)

# tuple, positional group of values
*(first-value second-value third-value)

# empty tuple. Used for example for empty state while stepping through a span
*

# record, like a tuple with names for each part. You may know it as "struct"
{first-field first-value second-field second-value}

# local function. The pattern must add a type to all variables
# can **not** use variables from the outer scope.
fn(first-parameter-pattern second) result

# parameter pattern variable
# appending a type is only necessary and allowed in function parameters
some-variable some-type

# parmeter pattern ignore
# appending a type is only necessary and allowed in function parameters
_ some-type

# deconstruct into a pattern.
# Usually used to assign the result of a function to a variable or to ignore a result
= query-pattern value
resulting-expression

# pattern match, checked for exhaustiveness
if value {
first-query-pattern
    first-result
second-query-pattern
    second-result
}

# project function declaration.
# For type variables in the result that aren't used in the input,
# functions require appended type parameters: <...>
fn function-name<Type-arguments That-are-only-used-in-the-result>(
    parameter-patterns-with-types
    another-parameter
)
    result-type
# optional documentation
# comment
    result-expression

# type name without arguments
u32

# type with multiple arguments
point(meters f32)

# type variable/parameter
Type-variable-name

# function type
fn(first-parameter-type second) result-type

# declare a shorthand for an existing type
ty point {x i32 y i32}

# can also accept parameters
ty pair(First Second-parameter) type-using-the-type-variables

# a "choice type" that can come in different shapes ("variants")
# which each have a unique name and one associated value.
[
First-option-no-associated-values
Second-option(some(u32))
Third-option-multiple-arguments(u32 f32)
]

# creating variants.
# The type in angles can be a type alias or a choice type directly <[... ...]>.
# The type does not have to include a variant with the currently constructed name
Third-option-multiple-arguments<a-choice-type>(its values)
First-option-no-associated-values<a-choice-type>

# variant patterns
Third-option-multiple-arguments(its values)
First-option-no-associated-values
```

## editor setups

### zed/gram-like
1. clone this repo
2. open the editor command panel
3. "zed: install dev extension" or "gram: install extension from folder" and select the cloned-path-slug-lang/zed directory

Optionally for a file icon in the project panel, open the editor command panel, select `Icon theme selector: toggle` and choose "slug icon dark"/"slug icon light".

### vscode-like
#### pre-built
1. download https://codeberg.org/lue-bird/slug-lang/src/branch/main/vscode/slug-0.1.0.vsix
2. open the command bar at the top and select: `>Extensions: Install from VSIX`
#### build from source
1. clone this repo
2. open `vscode/`
3. run `npm run package` to create the `.vsix`
4. open the command bar at the top and select: `>Extensions: Install from VSIX`

### helix
write to `~/.config/helix/languages.toml`:
```toml
[language-server.slug]
command = "slug lsp"
[[language]]
name = "slug"
scope = "source.slug"
injection-regex = "slug"
file-types = ["slug"]
indent = { tab-width = 4, unit = "    " }
language-servers = [ "slug" ]
auto-format = true
```

For other editors, there's usually a way to specify `slug` as the language server and or point to the directory `tree-sitter/` in this repository.


> As a user of slug you can stop reading here. The rest is for developers and those interested in language design

## dev setup
to recompile, run
```bash
cargo install --offline --debug --path . slug
```


## ergonomic improvements over sloe
- eliminating the need for any type variables and `Origin-erased`
- bufs can be passed implicitly because the origin already uniquely identifies them
- drop and clone is implicit and happens after the last use. This is possible leak-free because a `slot` knows how to modify the associated `buf` in the scope it's in
- positional arguments and otherwise familiar syntax
- `.` syntax for piping the first argument
- enabling nested origins or even type-identity origins (like Box<> or gc or shared_ptr)
- more conventional syntax

## TODO
- add more examples
- show implicitly passed bufs when hovering a project fn
- don't report "this if case pattern only covers possibilities that earlier case patterns will already catch." if errors were already reported inside (most importantly type mismatch)
- website with online repl
- core add more primitives for strings (e.g. span-strip-str-chars-once(span direction str) for checking starts with or ends with)

## implementation details
- introduce operation that can set an index after new unset items. Something like `add-unset-then`
- shadowing is disallowed (local functions are an entirely separate context so they allow it)
- expressions like `buf-empty<^new-origin-name element-type>()` automagically introduce `^new-origin-name` into the scope.
  The fact that the origin could be literally any type is no problem because uniqueness of the type is still checked and no other properties are relevant
- once a `buf` has been "used" (e.g. explicitly passed in and out of a function) it invalidates all prior uses of the origin in the rest of the current scope. E.g. a buf of bufs + a buf of slots with the same origin means you could mix and match willy nilly
- For functions with type variables, pass in a generic "bundle of bufs" along with drop and clone functions that take and modify that bundle
- implicitly pass a generic "bundle of bufs" whenever a function does not itself take a buf
- If the input and output types never mention an origin created inside, we could allocate the buf with this origin with an arena
- type signatures and type aliases are _not_ checked for origin violations, only actual expression+pattern code is
- passing bufs is allowed but marks it as "unusable" for implicit passing (which includes clone and drop), which is basically identical to invalidating slots and spans with the same origin
- type variables are checked as if they contained any number of buf origins and slot/span origins.
  To illustrate why this could otherwise lead to issues, look at
  ```
  fn (
      first slot(Origin {x X y Y})
      second slot(Origin {x X y Y})
      xy-rid fn({x X y Y}) *
  )
      
      = * xy-rid({x first-x y second-y})
      xy-rid({x second-y y first-y})
  ```  
  So: (I think) this means no introducing a type variable-typed value into a scope where _any_ type-variable-typed value is already reachable. Or more concretely, it must never be used in variable functions calls or function results, but that may be harder to detect.
- allow `buf` to be configured to never free slots (acting like an arena). This means no counts are ever recorded (massive for temporary bufs) but also any `item-at-index` stuff outside of `span` is likely to hit dead items and is thus invalid.
  Question: How to represent in zig?

## known design weaknesses
- The most important: implicitly cloning and dropping spans is O(span-length). This is quite the performance footgun and easily makes traversals quadratic.
  My hope is that compile optimizations can eliminate many of those. If this assumptions turns out to be false, I may need to look into e.g. "automatic input-only parameters = don't drop" and or lobster/...-adjacent stuff. There may also be a more efficient representation for slot counts, like an enum(u2){0,1,2,look-in-side-list} for every item that looks in a side-list for higher counts.
- building spans of a nested structure is quadratic because there is no concept of "claimed unset slots/spans"
- while implicit passing of bufs is one "killer convenience feature" of slug, it comes with a bunch of drawbacks.
  For example, if bufs were explicitly passed,
    - `slot` and `span` would not need a second type parameter (huge) and type aliases would never be recursive
    - it's way more obvious
    - `buf` could also be specialized to e.g. `arena` or `index-map`, which themselves could also have a specialized API that may for example allow iterating all items or not
- it's not uncommon to keep one span around which covers all elements as a "central access point" and to keep a range of items alive even if there are no referents.
  This means that individual counter increments and decrements are entirely pointless but are almost impossible to optimize out :(
  
  The same applies to spans that are kept around to enable bulk deletion. All slots inside must be checked individually which is slow. Maybe there is a buf representation which encodes all this more clearly?
- the syntax is ugly. It feels dense. It feels disorganized. It feels unintuitive (somehow). Any help appreciated!

## potential improvements in the future
- add a crash primitive which in zig "throws" an error in the same way as OutOfMemory, in js just throws a generic new Error(). This would enable building "memory safe but not validity-safe" abstractions in slug itself
- zig codegen: implement leak-free error handling via "errdefer { for all reachable bufs: drop(allocator) }"
- consider requiring `:` for parameter pattern variables between the name and type. This is probably more obvious and is a little more robust. Downside: more typing and still not super clear. Format: name: type
- (will probably reject) what about slots and spans that are valid across multiple (!) bufs? This would make things like struct-of-arrays easier and require less duplicate info in memory.
- should there be separate types for shared and exclusive access? Exclusive access would be easier to reason about and allow for connecting spans into one span.
  Exclusive slots would need to be deeply (I think) cloned while shared slots (`slot-counted`?) are shallow-copied + count increased.
  They should live in separate bufs to save space and be faster.
  
  Also similarly, there could be a distinction for shared mutable and shared immutable slots. I wonder if there is a point?
- automatically figure out optimizations for "consume-only" parameters not needing to be cloned
- I would **LOVE** if there was a way to allow for "unspecialized function types". If e.g. rust gains support for this, we should add this (and maybe backport it to sloe)
- could this approach be used to compile existing, high-level languages (e.g. elm/gren)?
- should FFI be more ergonomic? Maybe by providing `extern-fn` signatures (and maybe `extern-ty`?) that must get populated by the platform
- change `{}` to `*` in the formatter

## rejected
(some points may be outdated)

- should `^origin` be included in each `slot` and `span`?
  This would complicate the language for users and compiler authors, especially around type signatures.
  Reasons for consideration:
    - no proper way to distinguish between e.g. temporary spans and permanent spans, to avoid e.g. possibly quadratic `span-add` operations
    - no proper way to distinguish slots with the same element type but different semantics. E.g. `{ users span(slot(person-info)) creators span(slot(person-info)) }`
- types currently use `*` for slot, `_` for span instead of regular names. Disadvantage: Their name in operations is not obvious. Hover info should help. Advantages: Shorter and it's actually a primitive with special rules. Giving it a normal name doesn't do it justice
- consider changing `f(0 1)` syntax to `(f 0 1)` including types. This takes up 1 more space. It is however more consistent with record and variant syntax and can potentially start suggesting completions earlier.
  By far the biggest disadvantage is familiarity. People will hate the lispiness :(
- closures/ any/dyn-types
- (soft reject) local type aliases
- (soft reject) the option to declare nominal types
- should the underlying bufs be per-(size+align / bucket) instead of per-type? Per-type feels more reasonable but per-(size+align / bucket) could be more compact, avoid frequent smaller allocations and be leaner on the stack when passing the bufs.
  On the other hand it may (?) be worse for caches (semantically related objects are not necessarily grouped together), debugging, ffi, and memory usage (short-lived item bufs temporarily "pollute" a bigger buf and aren't freed as easily)
- change tuple syntax from `*()` to `<>` (more consistent and easier to type)

### explicit share-able view types (outdated syntax)
All structural types (e.g. char, str, records of structural types, choice types of structural types) are directly compatible with & and can be recombined into & types (for example `&str` is the same as `str`).
Non-structural types (e.g. buf, span, slot) can only be temporarily "borrowed" with &.
Adding & is thus allowed in front of any type and makes it have the properties of any other &type.

If introduced variables are &-typed, they can be dropped whenever and stay in scope.
If not, they are immediately consumed on next use.
&-typed values are 

```slug
ty buf-span(Origin Item)
    { buf buf(Origin char) span span(Origin) }

ty buf-opt-span(Origin Item)
    { buf buf(Origin char) span opt(span(Origin)) }

fn greet
    string buf-opt-span(Origin char)
    name str
: buf-span(Origin Item)
    buf-opt-span-add-str(string "Hello, "str) = string
    buf-span-add-str(string name) = string
    buf-span-add(string "!"char)


fn times-4-square n& i32 : i32
    i32-to-power(n 2i32)
```

problematic: No way to get a & from a buf item:
```slug
fn buf-item buf &buf(Origin Item) slot &slot(Origin) : &Item
```
as this _requires_ & to be interpreted as "the incoming &s were themselves passed in as a function parameter".
Arguably the easiest resolution is either
- (not flexible enough!) treat all parameter &s as living as long as the outer function
- require specifying on all parameter &s whether they live as long as the inner or outer function, e.g. `&` vs `_&` (or maybe `*`?)
