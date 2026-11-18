programming language which uses indexes bound to specific array buffers instead of references.
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

Think of how you'd parse infos from emails in a log file.
You might try collecting some views over text and a few extra array buffers.
```rust
struct Email {
    local: []char, // non-empty
    host_labels: [][]char, // non-empty
    top_level: []char, // non-empty
}
struct ParsedFileEmails {
    emails: Vec<Email>,
    host_labels: Vec<[]char>,
    text: Vec<char>, // in practice you might store the text somewhere else
}
````
With this setup you need to be careful: the slices could get invalidated once the underlying array buffers grow.
So you might switch to
```rust
struct Email {
    local: Range, // non-empty
    host_labels: Range, // non-empty
    top_level: Range, // non-empty
}
struct ParsedFileEmails {
    emails: Vec<Email>,
    host_labels: Vec<Range>,
    text: Vec<char>,
}
```
Does it feel good to write code like this?
Wish you could specify which references/indexes point into the which collection and have the compiler verify this for you?
Here's some slug code:
```slug
ty email(Host-labels Text) {
    local span(Text char)
    host-labels span(Host-labels span(Text char))
    top-level span(Text char)
}
ty parsed-file-emails {   
    emails opt(span([Email] email([Host-labels] [Text])))
    
    emails-buffer buf([Email] email([Host-labels] [Text]))
    host-labels buf([Host-labels] span([Text] char))
    text buf([Text] char)
}
```
Now it's clear to humans and the compiler that `local` and `top-level` point into the same array buffer of chars and that `host-labels` points into a separate array buffer of ranges into the array buffer of chars.
(`[`Some-name`]` represents an enum with just that one member, you can think of it as just a tag type for now)

This is more powerful than you may think:
- whenever a function parameter or result uses a slot or span but no buf with the same type parameters is in reach there, these bufs can be passed implicitly. This means inserting into a buf and accessing a buf for example never requires passing a buf. With that slots and spans should feel almost as convenient as plain references
- all types know how to drop and clone themselves (using the fact that the necessary implied bufs are always present at that point). These operations tend to be cheap in slug and are thus inserted automatically

Having these kinds of features is what makes the language fun to use in my opinion. Compare with my earlier attempt at enabling this pattern: [sloe](https://codeberg.org/lue-bird/sloe) which is probably faster on average but felt like it sucked every particle of joy out of the room :(

The rules to make it all safe (checked by the compiler):
  - you can never have multiple collections that have the same item and origin type in reach
  - when exiting a scope where new array buffers were introduced, you cannot return slots and spans with the same origin and item type without also returning the containing array buffer 

For a more complex example, here's how you could represent the syntax of slug's `ty` declarations:
```slug
ty type-alias(Syntax Source) {
    parameters opt(span(Syntax span(Source char)))
    type type(Syntax Source)
}

ty type(Syntax Source) [
    Variable(span(Source char))
    Tuple(opt(span(Syntax type(Syntax Source))))
    Fn({
        parameters opt(span(Syntax type(Syntax Source)))
        result slot(Syntax type(Syntax Source))
    })
    Alias(
        span(Source char)
        opt(span(Syntax type(Syntax Source)))
    )
    Record(
        opt(span(Syntax type-record-field(Syntax Source)))
    )
    Choice( 
        opt(span(Syntax type-choice-variant(Syntax Source)))
    )
]

ty type-record-field(Syntax Source) {
    name span(Source char)
    value type(Syntax Source)
}

ty type-choice-variant(Syntax Source) {
    name span(Source char)
    values opt(span(Syntax type(Syntax Source)))
}
```
Notice here how we use the same `Syntax` parameter with different item types.
This is fine because we know these bufs are all created and destroyed at the same time.
Rule of thumb: One purpose = one unique type variable

Here's how we could store each file independently so that e.g. removing/replacing one file
doesn't need to crawl through the syntax tree to free each slot and span:
```slug
ty file {
    type-aliases opt(span([Syntax] type-alias([Syntax] [Source])))
    
    source buf([Source] char)
    names buf([Syntax] span([Source] char))
    types buf([Syntax] type([Syntax] [Source]))
    type-variant-values buf([Syntax] opt(span([Syntax] type([Syntax] [Source]))))
    type-aliases-buf buf([Syntax] type-alias([Syntax] [Source]))
}

ty state {
    files opt(span([File] file))
    projects-buf buf([File] file)
}

fn initial-state() state {
    projects No<opt(span([File] file))>
    projects-buf buf-empty<[File] file>()
}
```
[Additional examples, explanations, a repl and core library documentation](https://lue-bird.github.io/slug-lang/)

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
# Placing a comment immediately before a declaration is interpreted as documentation
fn function-name<Type-arguments That-are-only-used-in-the-result>(
    parameter-patterns-with-types
    another-parameter
)
    result-type
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
- more familiar syntax

## TODO
- implement span_sort in zig (and don't forget to clone the items to order)
- allow piping into Variant constructor. Then remove fn opt-yes in examples
- rename unset-slice-cast-or-rid-and-allocate to unset-slice-cast-or-drop-and-allocate
- rename buf-pre-allocation-rid to buf-pre-allocation-drop

## known design weaknesses
- The most important: implicitly cloning and dropping spans is O(span-length). This is quite the performance footgun and easily makes operations like recursive traversals quadratic.
  My hope is that compile optimizations can eliminate many of those. If this assumptions turns out to be false, look into e.g.
    - not doing anything on drop for slot/span if the containing buf only has 1 owner and is dropped in the sme breath
    - "automatic input-only parameters = don't drop" and or lobster/...-adjacent stuff
    - a more efficient representation for slot counts, like an enum(u2){0,1,2,look-in-side-list} for every item that looks in a side-list for higher counts
- building spans of a nested structure is quadratic because there is no concept of "claimed unset slots/spans"
- while implicit passing of bufs is one "killer convenience feature" of slug, it comes with a bunch of drawbacks.
  For example, if bufs were explicitly passed,
    - `slot` and `span` would not need a second type parameter (huge) and type aliases would never be recursive
    - it's way more obvious
    - `buf` could also be specialized to e.g. `arena` or `index-map`, which themselves could also have a specialized API that may for example allow iterating all items or not
- it's not uncommon to keep one span around which covers all elements as a "central access point" and to keep a range of items alive even if there are no referents.
  This means that individual counter increments and decrements are entirely pointless but are almost impossible to optimize out :(
  
  The same applies to spans that are kept around to enable bulk deletion. All slots inside must be checked individually which is slow. Maybe there is a buf representation which encodes all this more clearly?
- the syntax is fairly ugly. It feels dense. It feels disorganized. It feels unintuitive (somehow).
  destruct syntax indents weirdly when put directly after e.g. a single-line case pattern
  Any help appreciated!

## potential improvements in the future
- zig codegen: implement leak-free error handling via "errdefer { for all reachable bufs: drop(allocator) }"
- consider introducing operation that can set an index after new unset items. Something like `add-unset-then`.
  A better option may be changing counts to `bitpack struct { is_unset: bool, count: u15 }` and introducing
  unset slots and spans. This would also maybe solve the current "buf-pre-allocate needs to take in a buf which defeats the whole point of slug"
- consider allowing `buf` to be configured to never free slots (acting like an arena). This means no counts are ever recorded (massive for temporary bufs) but also any `item-at-index` stuff outside of `span` is likely to hit dead items and is thus invalid.
  Question: How to represent in zig?
- (will probably reject) what about slots and spans that are valid across multiple (!) bufs? This would make things like struct-of-arrays easier and require less duplicate info in memory.
- automatically figure out optimizations for "consume-only" parameters not needing to be cloned
- I would **LOVE** if there was a way to allow for "unspecialized function types". If e.g. rust gains support for this, we should add this (and maybe backport it to sloe)
- allow comments before field names and choice type variant names
- could this approach be used to compile existing, high-level languages (e.g. elm/gren)?
- should FFI be more ergonomic? Maybe by providing `extern-fn` signatures (and maybe `extern-ty`?) that must get populated by the platform
- add a crash primitive which in zig "throws" an error in the same way as OutOfMemory, in js just throws a generic new Error(). This would enable building "memory safe but not validity-safe" abstractions in slug itself
- add more examples

## rejected
(some points may be outdated)

- consider requiring `:` for parameter pattern variables between the name and type. This is probably more obvious and is a little more robust. Downside: more typing and still not super clear. Format: name: type
- consider changing `f(0 1)` syntax to `(f 0 1)` including types. This takes up 1 more space. It is however more consistent with record and variant syntax and can potentially start suggesting completions earlier.
  By far the biggest disadvantage is familiarity. People will hate the lispiness :(
- closures/ any/dyn-types
- (soft reject) local type aliases
- (soft reject) the option to declare nominal types
- consider indenting dot-calls. Tried but somehow looks even more disorganized
- should the underlying bufs be per-(size+align / bucket) instead of per-type? Per-type feels more reasonable but per-(size+align / bucket) could be more compact, avoid frequent smaller allocations and be leaner on the stack when passing the bufs.
  On the other hand it may (?) be worse for caches (semantically related objects are not necessarily grouped together), debugging, ffi, and memory usage (short-lived item bufs temporarily "pollute" a bigger buf and aren't freed as easily)
- change tuple syntax from `*()` to `<>` (more consistent and easier to type)
- should there be separate types for shared and exclusive access? Exclusive access would be easier to reason about and allow for connecting spans into one span.
  Exclusive slots would need to be deeply (I think) cloned while shared slots (`slot-counted`?) are shallow-copied + count increased.
  They should live in separate bufs to save space and be faster.
  
  Also similarly, there could be a distinction for shared mutable and shared immutable slots. I wonder if there is a point?
- dot-piping into tuples
- add go as an output target. Rejected because there's nice way to locally disable array bounds checks

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


## ramblings - not valuable. skip
Allocating individual recursive references and slices (e.g. rust's `Box`/`String`/`Vec`/`Box<str>`/`Box<[]>`)
  - memory is usually more , meaning it's harder to predict 
  - usually no simple way to pre-allocate space for different purposes
  - to drop, this usually means traversing the whole structure, wandering from pointer to pointer when all we would want to do is just delete all memory that was used for one purpose in one fell swoop
  - pointers are usually 64 bit wide which can be wasteful

Arenas and especially storing an arena for each "group of memory allocations for a specific purpose" does solve most of these,
especially when combined with some kind of tagging system so that no arenas are accidentally confused.
but
  - let's say you grow a few buffers stored in an arena multiple times. Now you have a good bit of stale ("leaked") memory around that you can't give back. I think this can be fine but it's iffy
  - 64 bit wide
  - no way to free and reuse parts of it
  - no way to traverse it

We could then consider creating an arena for every item type.
I will explore this in [fuff](https://codeberg.org/lue-bird/fuff)

Plain integers as index and range types come with some drawbacks:
  - collections need to be passed to every function that needs access. Adding/removing just one collection means changing a bunch of signatures and callsites. I find this tiring
  - every access comes with a bounds check and potential crash or error handling
  - you can accidentally use them to access the wrong collection
  - If the underlying collections are alive for a while, items that have become useless may need to be removed and replaced with new items (think e.g. game enemies that have been eliminated).
    Keeping indexes in sync is a common source of bugs
