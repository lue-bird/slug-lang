Fast, ergonomic programming language which secretly uses indexes instead of pointers. inspired by
- [`Arrrlang`](https://verdagon.dev/blog/myth-zero-overhead-memory-safety)
- my earlier attempt at enabling this pattern using linear types: [sloe](http://codeberg.org/lue-bird/sloe)

```slug
fn greet(name str) _char
    str-chars("Hello, " str)
    .span-add-str-chars(name)
    .span-add("!" char)
```
`_char` is a span into a heap array of characters.

Important: The compiler and tooling is not implemented, yet! Once available,
install with (requires having [rust installed](https://rust-lang.org/tools/install/))
```bash
cargo install --git https://codeberg.org/lue-bird/slug-lang slug
```

## syntax
```slug
# line comment

# number (available types: p32, u32, i32, f32).
# Specifying a type is required
3.2 f32

# text of type str
"hello" str

# unicode scalar of type char
"a" char

# identifiers. All except type variables start lowercase
variable-or-function-or-field-or-variant-or-type-2012

# function call. arguments are separated by space
some-function(first-argument second-argument)

# syntax sugar for some-function(first-argument second)
first-argument.some-function(second)

# very rarely functions require type arguments <in angles>.
# Function declarations will explicitly list those <Parameters> (see later)
some-function<type arguments>(argument)

# record. You may know it as "struct".
# If field values themselves end in records they need to be parenthesized.
# The last field value can end in a record without needing to be parenthesized
{ first-field first-value second-field second-value }

# local function of type fn.
# the pattern must add a type to all variables
# can **not** use variables from the outer scope.
fn(first-parameter-pattern second) result

# parameter pattern variable
# appending a type is only necessary and allowed in function parameters
some-variable some-type

# parmeter pattern ignore
# appending a type is only necessary and allowed in function parameters
_ some-type

# decompose into a pattern.
# Usually used to assign the result of a function to a variable or to ignore a result
= case-pattern value
resulting-expression

# pattern match, checked for exhaustiveness
if value { first-case-pattern first-result second-case-pattern second-result }

# project function declaration.
# For type variables in the result that aren't used in the input,
# functions require appended type parameters: <...>
fn function-name<Type-arguments That-are-only-used-in-the-result>
    (parameter-patterns-with-types another-parameter)
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

# slot type (index in a heap array)
*item-type

# span type (range in a heap array)
_item-type

# declare a shorthand for an existing type
ty point { x i32 y i32 }

# can also accept parameters
ty pair(First Second-parameter) type-using-the-type-variables

# a "choice type" that can come in different shapes ("variants")
# which each have a unique name and one associated value.
[ first-option {}
  second-option some(u32)
  third-option { some u32 values f32 }
]

# creating a variant.
# The type in angles can be a type alias or a choice type directly <[... ...]>.
# The type does not have to include a variant with the currently constructed name
[some-variant-name<a-choice-type> its-value]

# variant pattern
[some-variant its-value]
```
Goal: coherent, practical and compact, avoiding parens and indentation especially for trailing syntax.
Sloe is a wordy and explicit language, so any extra verbosity is not tolerable.

## editor setups

### zed/gram-like
1. clone this repo
2. open the editor command panel
3. "zed: install dev extension" or "gram: install extension from folder" and select the cloned-path-slug-lang/zed directory

Optionally for a file icon in the project panel, open the editor command panel, select `Icon theme selector: toggle` and choose "slug icon dark"/"slug icon light".

### vscode-like
#### pre-built
1. download https://codeberg.org/lue-bird/sloe/src/branch/main/vscode/sloe-0.1.0.vsix
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


## ergonomic improvements over sloe

- eliminating the need for any type variables and `Origin-erased`
- every function is analyzed for the set of accessed slot/span origins. The bufs for those origins are passed in implicitly
- new API (which still e.g. only requires one u32 per Slot because the compile-time origin uniquely identifies the Buf to access):
  `*Item` (slot), `_Item` (span),
  `fn slot(Item)`
  `fn slot-item(*Item) Item`,
  `fn slot-set(slot(Item) Item) Item`,
  similar for span.
  (most likely also `slot-set` for `Buf-counted`-like behavior)
  These APIs replace existing Buf functions.
- drop is implicit and happens after the last use. This is possible leak-free because a `Slot` knows how to mark itself as unset
- clone is implicit. This is possible leak-free because a `Slot` knows how to modify its associated `Buf` accordingly
- positional arguments and otherwise familiar syntax
- `.` syntax for piping the first argument
- shadowing is allowed and encouraged
- some crazy things this allows: `slot-to-index`, `fn slot-at-index(u32) opt(slot(Item))`, `fn span-starting-at-with-length`, (maybe `fn all-items-step(fn({... Item} {... Item}))`, `fn all-items-alter`), `fn span-map`, anything that modifies the underlying buf without moving existing indexes around

## implementation details
- if `slot` has a non-concrete type, the name the buf parameter should match the variable name
- recursive types that don't eventually go through a slot or span must be reported
- Bufs themselves are dropped once we know no more Slot/Span with a certain item type exists.
  This is exactly once `buf.items.len == 0`.
  (In theory I think this could also be traced by the compiler top down from the call stack.)
- (local) function types do not list which item bufs actually get accessed.
  This means the _function input and output types_ gets consulted to find out item types.
- when a function call asks for a buf which does not exist yet, allocate it.
  If the whole function does not return a value referencing this buf, we can allocate it with an arena.


## TODO
- implement compiler & tooling
- website with online repl

## potential improvements in the future
- I would **LOVE** if there was a way to allow for "unspecialized function types". If we decide to never compile to rust or rust gains support for this, we should instantly add this (and backport it to sloe)
- could this approach be used to compile existing, high-level languages (e.g. elm/gren)?
- (leaning no) should there be separate types for shared and exclusive access? Exclusive access would be easier to reason about and allow for connecting spans into one span.
  Exclusive slots would need to be deeply (I think) cloned while shared slots (`slot-counted`?) are shallow-copied + count increased.
  Question: Should they live in separate bufs to save space?
  Also similarly, there could be a distinction for shared mutable and shared immutable slots. I wonder if there is a point?
- should FFI be more ergonomic? Maybe by providing `extern-fn` signatures that must get populated by the platform

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

### explicit share-able view types
All structural types (e.g. char, str, records of structural types, choice types of structural types) are directly compatible with & and can be recombined into & types (for example `&str` is the same as `str`).
Non-structural types (e.g. buf, span, slot) can only be temporarily "borrowed" with &.
Adding & is thus allowed in front of any type and makes it have the properties of any other &type.

If introduced variables are &-typed, they can be dropped whenever and stay in scope.
If not, they are immediately consumed on next use.
&-typed values are 

```slug
ty buf-span(_origin Item)
    { buf buf(_origin char) span span(_origin) }

ty buf-opt-span(_origin Item)
    { buf buf(_origin char) span opt(span(_origin)) }

fn greet
    string buf-opt-span(_origin char)
    name str
: buf-span(_origin Item)
    buf-opt-span-add-str(string "Hello, " str) = string
    buf-span-add-str(string name) = string
    buf-span-add(string "!" char)


fn times-4-square n& i32 : i32
    i32-to-power(n 2 i32)
```

problematic: No way to get a & from a buf item:
```slug
fn buf-item buf &buf(_origin Item) slot &slot(_origin) : &Item
```
as this _requires_ & to be interpreted as "the incoming &s were themselves passed in as a function parameter".
Arguably the easiest resolution is either
- (not flexible enough!) treat all parameter &s as living as long as the outer function
- require specifying on all parameter &s whether they live as long as the inner or outer function, e.g. `&` vs `_&` (or maybe `*`?)
