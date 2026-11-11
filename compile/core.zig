const std = @import("std");

fn RecordWithFieldNames(comptime @"%field_names": []const []const u8) type {
    return struct {
        fn WithFieldValueTypesOf(@"%type_tuple": type) type {
            return @Struct(
                std.lang.Type.ContainerLayout.auto,
                null,
                @"%field_names",
                @ptrCast(@typeInfo(@"%type_tuple").@"struct".field_types),
                &@splat(.{}),
            );
        }
        fn withFieldValuesOf(@"%field_values": anytype) @This().WithFieldValueTypesOf(@TypeOf(@"%field_values")) {
            var @"%record": @This().WithFieldValueTypesOf(@TypeOf(@"%field_values")) = undefined;
            inline for (@"%field_names") |@"%field_name"| {
                @field(@"%record", @"%field_name") = @field(@"%field_values", @"%field_name");
            }
            return @"%record";
        }
    };
}
/// be aware: different field order means different type
pub fn Record(@"%struct_type": type) type {
    // uncomment to verify field names are sorted
    // if (!std.sort.isSorted([]const u8, @typeInfo(@"%struct_type").@"struct".field_names, {}, struct {
    //     fn f(_: void, @"%a": []const u8, @"%b": []const u8) bool {
    //         return std.mem.order(u8, @"%a", @"%b") == .lt;
    //     }
    // }.f)) {
    //     @compileError("fields must be sorted");
    // }
    return RecordWithFieldNames(@typeInfo(@"%struct_type").@"struct".field_names).WithFieldValueTypesOf(@"%struct_type");
}
/// be aware: different field order means different type
pub fn record(@"%struct_value": anytype) Record(@TypeOf(@"%struct_value")) {
    return RecordWithFieldNames(@typeInfo(@TypeOf(@"%struct_value")).@"struct".field_names).withFieldValuesOf(@"%struct_value");
}
fn StructMerge(@"%A": type, @"%B": type) type {
    switch (@typeInfo(@"%A")) {
        .void => {
            return @"%B";
        },
        .@"struct" => |@"%A struct"| {
            switch (@typeInfo(@"%B")) {
                .void => {
                    return @"%A";
                },
                .@"struct" => |@"%B struct"| {
                    return @Struct(
                        .auto,
                        null,
                        @"%A struct".field_names ++ @"%B struct".field_names,
                        @"%A struct".field_types ++ @"%B struct".field_types,
                        @"%A struct".field_attrs ++ @"%B struct".field_attrs,
                    );
                },
                else => {
                    @compileError("expected void or a struct");
                },
            }
        },
        else => {
            @compileError("expected void or a struct");
        },
    }
}
fn structMerge(@"%a": anytype, @"%b": anytype) StructMerge(@TypeOf(@"%a"), @TypeOf(@"%b")) {
    var @"%combined": StructMerge(@TypeOf(@"%a"), @TypeOf(@"%b")) = undefined;
    switch (@typeInfo(@TypeOf(@"%a"))) {
        .void => {
            return @"%b";
        },
        .@"struct" => |@"%A struct"| {
            switch (@typeInfo(@TypeOf(@"%b"))) {
                .void => {
                    return @"%a";
                },
                .@"struct" => |@"%B struct"| {
                    inline for (@"%A struct".field_names) |@"%a_field_name"| {
                        @field(@"%combined", @"%a_field_name") = @field(@"%a", @"%a_field_name");
                    }
                    inline for (@"%B struct".field_names) |@"%b_field_name"| {
                        @field(@"%combined", @"%b_field_name") = @field(@"%b", @"%b_field_name");
                    }
                    return @"%combined";
                },
                else => {
                    @compileError("expected void or a struct");
                },
            }
        },
        else => {
            @compileError("expected void or a struct");
        },
    }
}
fn StructMergeAll(@"%structs": []const type) type {
    var @"%field_names": []const [:0]const u8 = &.{};
    var @"%field_types": []const type = &.{};
    var @"%field_attrs": []const std.lang.Type.Struct.FieldAttributes = &.{};
    for (@"%structs") |@"%maybe_struct"| {
        switch (@typeInfo(@"%maybe_struct")) {
            .void => {},
            .@"struct" => |@"%struct"| {
                @"%field_names" = @"%field_names" ++ @"%struct".field_names;
                @"%field_types" = @"%field_types" ++ @"%struct".field_types;
                @"%field_attrs" = @"%field_attrs" ++ @"%struct".field_attrs;
            },
            else => {
                @compileError("expected void or a struct");
            },
        }
    }
    return @Struct(.auto, null, @"%field_names", @"%field_types", @"%field_attrs");
}
fn structSubset(
    @"%value": anytype,
    @"%Fields": type,
) @"%Fields" {
    switch (@typeInfo(@"%Fields")) {
        .void => {
            return {};
        },
        .@"struct" => |@"%struct"| {
            var @"%result": @"%Fields" = undefined;
            inline for (@"%struct".field_names) |@"%field_name"| {
                @field(@"%result", @"%field_name") = @field(@"%value", @"%field_name");
            }
            return @"%result";
        },
        else => {
            @compileError("expected void or a struct");
        },
    }
}
fn StructFromFields(
    @"%names": []const []const u8,
    @"%Values": *const [@"%names".len]type,
) type {
    return @Struct(
        .auto,
        null,
        @"%names",
        @"%Values",
        &@splat(std.lang.Type.Struct.FieldAttributes{}),
    );
}
fn structFromFields(
    comptime @"%names": []const []const u8,
    @"%values": anytype, // Tuple(@"%names".len, anytype)
) StructFromFields(
    @"%names",
    @typeInfo(@TypeOf(@"%values")).@"struct".field_types,
) {
    var @"%result": StructFromFields(
        @"%names",
        @typeInfo(@TypeOf(@"%values")).@"struct".field_types,
    ) = undefined;
    inline for (@"%names", @typeInfo(@TypeOf(@"%values")).@"struct".field_names) |@"%name", @"%i"| {
        @field(@"%result", @"%name") = @field(@"%values", @"%i");
    }
    return @"%result";
}
fn Ctx(@"%AdditionalFields": []const type) type {
    return StructMerge(
        struct { allocator: std.mem.Allocator },
        StructMergeAll(@"%AdditionalFields"),
    );
}
/// pass either SlugType or .{ name = "..slug..", .Ty = ZigType }
pub fn CtxBuf(@"%Origin": anytype, @"%Item": anytype) type {
    return @Struct(
        .auto,
        null,
        &.{"buf(" ++ @"%Origin".name ++ " " ++ @"%Item".name ++ ")"},
        &.{*BufCore(@"%Origin".Ty, @"%Item".Ty)},
        &.{std.lang.Type.Struct.FieldAttributes{}},
    );
}
pub fn ctxBufName(
    comptime @"%origin_name": []const u8,
    comptime @"%item_name": []const u8,
) []const u8 {
    return "buf(" ++ @"%origin_name" ++ " " ++ @"%item_name" ++ ")";
}
/// pass either SlugType or .{ name = "..slug..", .Ty = ZigType }
pub fn ctxBuf(
    ctx: anytype,
    @"%Origin": anytype,
    @"%Item": anytype,
) *BufCore(@"%Origin".Ty, @"%Item".Ty) {
    return @field(ctx, ctxBufName(@"%Origin".name, @"%Item".name)).*;
}
pub const SlugType = struct {
    Ty: type,
    Bufs: type = void,
    drop: fn (
        anytype, // Ctx(.{@This().Bufs})
        anytype, // @This().Ty
    ) void = struct {
        fn f(_: anytype, _: anytype) void {}
    }.f,
    clone: fn (
        anytype, // @This().Bufs
        anytype, // @This().Ty
    ) void = struct {
        fn f(_: anytype, _: anytype) void {}
    }.f,
    name: []const u8,
};

// If you're wondering about the strange names:
// - @"'variant_a'variant_b": since zig removed
//   support for proper anonymous union(enum)s,
//   this workaround is necessary to make zig believe they all belong to the same type
// - @"%Type" for type variables to not overlap with existing type names
// - @"%variable" for expression variables to not overlap with existing file-scope const/fn names.
// For the last 2, alternative naming schemes like `var_name` are much harder to
// properly disambiguate. E.g. what if a slug name actually also starts with var-?
// Prefixing with % is inspired by LLVM IR. Could have just as well used $ or others.
//
// When writing core declarations, this is a little error prone. It is what it is

// TODO do the same for structural tagged unions as for records
pub fn @"[No Yes]"(@"%No": type, @"%Yes": type) type {
    return union(enum) { no: @"%No", yes: @"%Yes" };
}
pub fn @"[Down Up]"(@"%Down": type, @"%Up": type) type {
    return union(enum) { down: @"%Down", up: @"%Up" };
}
pub fn @"[Equal Greater Less]"(@"%Equal": type, @"%Greater": type, @"%Less": type) type {
    return union(enum) { equal: @"%Equal", greater: @"%Greater", less: @"%Less" };
}
pub fn @"[Done Going]"(@"%Done": type, @"%Going": type) type {
    return union(enum) { done: @"%Done", going: @"%Going" };
}
pub fn @"[Out-of-memory Success]"(@"%Out_of_memory": type, @"%Success": type) type {
    return union(enum) { out_of_memory: @"%Out_of_memory", Success: @"%Success" };
}
/// would preferably be noreturn but it isn't allowed in parameters for some reason
pub const @"[]" = enum {};

pub const P32 = struct {
    // zig does not have non-zero number types, yet.
    // This is quite wasteful.
    // The one tiny performance benefit is that .predecessor() never underflows on valid P32s
    positive: u32,
    pub const one = P32{ .positive = 1 };
    pub const maxValue = P32{ .positive = std.math.maxInt(u32) };
    pub fn fromComptime(comptime @"%u32": u32) @This() {
        return if (@"%u32" == 0) @compileError("given unsigned integer is not positive") else .{ .positive = @"%u32" };
    }
    pub fn fromU32(@"%u32": U32) ?@This() {
        return if (@"%u32" == 0) null else .{ .positive = @"%u32" };
    }
    pub fn predecessor(@"%p": @This()) u32 {
        return @"%p".positive - 1;
    }
    pub fn addAssumeNoOverflow(@"%p": @This(), @"%increase": u32) P32 {
        return .{ .positive = @"%p".positive + @"%increase" };
    }
    pub fn addClamp(@"%p": @This(), @"%increase": u32) P32 {
        return .{ .positive = @"%p".positive +| @"%increase" };
    }
    pub fn mulClamp(@"%p": @This(), @"%increase": P32) P32 {
        return .{ .positive = @"%p".positive *| @"%increase".positive };
    }
    pub fn min(@"%a": @This(), @"%b": P32) P32 {
        return .{ .positive = @min(@"%a".positive, @"%b".positive) };
    }
    pub fn max(@"%a": @This(), @"%b": P32) P32 {
        return .{ .positive = @max(@"%a".positive, @"%b".positive) };
    }
};
pub const U32 = u32;
pub const I32 = i32;
pub const F32 = f32;
/// assumed to be a valid utf8 codepoint. Should probably be a wrapper instead.
/// Would be nice if zig had one in std.unicode
pub const Char = u21;
pub const Str = struct {
    /// assumed to contain at least one codepoint. Using "" can lead to UB.
    /// Splitting this into a first u21 and the rest would be no safer because there are no
    /// checked utf8 char wrappers in zig
    utf8: std.unicode.Utf8View,

    pub fn fromComptime(comptime @"%bytes": []const u8) Str {
        return comptime @"%str": {
            const @"%utf8_view" = std.unicode.Utf8View.initComptime(@"%bytes");
            if (std.math.cast(u32, @"%bytes".len) == null) @compileError(std.fmt.comptimePrint("Str byte length must fit in a u32, is {}", .{@"%bytes".len}));
            break :@"%str" if (Str.fromUtf8View(@"%utf8_view")) |@"%str"| @"%str" else @compileError("Str must contain at least one codepoint");
        };
    }
    pub fn fromUtf8View(@"%utf8_view": std.unicode.Utf8View) ?Str {
        return if (@"%utf8_view".bytes.len >= 1)
            .{ .utf8 = @"%utf8_view" }
        else
            null;
    }
    pub fn utf8_byte_count_p32(@"%str": Str) P32 {
        return P32.fromU32(@intCast(@"%str".utf8.bytes.len)).?;
    }
    pub fn codepoint_count_p32(@"%str": Str) P32 {
        return P32.fromU32(@intCast(
            std.unicode.utf8CountCodepoints(@"%str".utf8.bytes) catch unreachable,
        )).?;
    }
    pub fn splitStart(@"%str": Str) struct { Char, std.unicode.Utf8View } {
        var @"%codepoint_iterator" = @"%str".utf8.iterator();
        const @"%start_codepoint" = @"%codepoint_iterator".nextCodepoint().?;
        return .{
            @"%start_codepoint",
            std.unicode.Utf8View.initUnchecked(@"%str".utf8.bytes[@"%codepoint_iterator".i..]),
        };
    }
    pub fn splitEnd(@"%str": Str) struct { std.unicode.Utf8View, Char } {
        var @"%i" = @"%str".utf8.bytes.len;
        while (@"%i" >= 1) {
            @"%i" -= 1;
            if (std.unicode.utf8ByteSequenceLength(@"%str".utf8.bytes[@"%i"])) |_| {
                var @"%last_codepoint_iterator" = std.unicode.Utf8View.initUnchecked(@"%str".utf8.bytes[@"%i"..]).iterator();
                const @"%end_codepoint" = @"%last_codepoint_iterator".nextCodepoint().?;
                return .{
                    std.unicode.Utf8View.initUnchecked(@"%str".utf8.bytes[0..@"%i"]),
                    @"%end_codepoint",
                };
            } else |@"%not_start_byte"| {
                switch (@"%not_start_byte") {
                    error.Utf8InvalidStartByte => {},
                }
            }
        }
        unreachable;
    }
};
pub const Error = error{OutOfMemory};

pub const Order =
    @"[Equal Greater Less]"(void, void, void);
pub fn Opt(@"%Yes": type) type {
    return @"[No Yes]"(void, @"%Yes");
}

fn strideOf(@"%Item": type) comptime_int {
    // at the time of writing, this is the same as
    // @sizeOf(@"%Item")
    // is there a nicer way? And is this always correct in the first place?
    return @sizeOf(@"%Item");
}
fn mathOrderToOrder(@"%order": std.math.Order) Order {
    return switch (@"%order") {
        .lt => .{ .less = {} },
        .eq => .{ .equal = {} },
        .gt => .{ .greater = {} },
    };
}

/// sadly origin and item type tags will not make different Slots different unique types.
/// This is because zig otherwise gets in trouble trying unroll "infinite" type aliases :(
pub fn Slot(_: type, _: type) type {
    return SlotCore;
}
pub const SlotCore = struct {
    index: u32,
    pub fn toSpan(@"%slot": @This()) SpanCore {
        return .{ .start = @"%slot".index, .length = P32.one };
    }
};
/// sadly origin and item type tags will not make different Spans different unique types.
/// This is because zig otherwise gets in trouble trying unroll "infinite" type aliases :(
/// invariant: start + length <= intMax(u32)
pub fn Span(_: type, _: type) type {
    return SpanCore;
}
pub const SpanCore =
    struct {
        start: u32,
        length: P32,
        pub fn endIndex(@"%span": @This()) u32 {
            return @"%span".start + @"%span".length.predecessor();
        }
        pub fn splitStart(@"%span": @This()) struct {
            after: Opt(SpanCore),
            start: SlotCore,
        } {
            return .{
                .start = .{ .index = @"%span".start },
                .after = if (P32.fromU32(@"%span".length.predecessor())) |@"%end_length"|
                    .{ .yes = .{
                        .start = @"%span".start + 1,
                        .length = @"%end_length",
                    } }
                else
                    .{ .no = {} },
            };
        }
        pub fn splitEnd(@"%span": @This()) struct {
            Opt(SpanCore),
            SlotCore,
        } {
            return .{
                if (P32.fromU32(@"%span".length.predecessor())) |@"%start_length"|
                    .{ .yes = .{
                        .start = @"%span".start,
                        .length = @"%start_length",
                    } }
                else
                    .{ .no = {} },
                .{ .index = @"%span".endIndex() },
            };
        }
        pub fn splitAfterLengthPositive(
            @"%span": @This(),
            @"%start_length_or_greater": P32,
        ) struct {
            SpanCore,
            Opt(SpanCore),
        } {
            const @"%start_length" = P32.min(@"%start_length_or_greater", @"%span".length);
            return .{
                .{ .start = @"%span".start, .length = @"%start_length" },
                if (P32.fromU32(@"%span".length.positive - @"%start_length".positive)) |@"%after_length_positive"| .{
                    .yes = .{
                        .start = @"%span".start + @"%start_length".positive,
                        .length = @"%after_length_positive",
                    },
                } else .{ .no = {} },
            };
        }
        pub fn splitBeforeEndLengthPositive(
            @"%span": @This(),
            @"%end_length_or_greater": P32,
        ) struct {
            Opt(SpanCore),
            SpanCore,
        } {
            const @"%end_length" = P32.min(@"%end_length_or_greater", @"%span".length);
            const @"%before_length" = @"%span".length.positive - @"%end_length".positive;
            return .{
                if (P32.fromU32(@"%before_length")) |@"%before_length_positive"| .{
                    .yes = .{
                        .start = @"%span".start,
                        .length = @"%before_length_positive",
                    },
                } else .{ .no = {} },
                .{
                    .start = @"%span".start + @"%before_length",
                    .length = @"%end_length",
                },
            };
        }
        pub fn step(
            @"%span": @This(),
            ctx: anytype,
            @"%direction": @"[Down Up]"(void, void),
            @"%initial_state": anytype,
            @"%step": *const fn (
                @TypeOf(ctx),
                @TypeOf(@"%initial_state"),
                SlotCore,
            ) Error!@TypeOf(@"%initial_state"),
        ) Error!@TypeOf(@"%initial_state") {
            var @"%state" = @"%initial_state";
            switch (@"%direction") {
                .up => {
                    for (@"%span".start..(@"%span".start + @"%span".length.positive)) |@"%index"| {
                        @"%state" = try @"%step"(
                            ctx,
                            @"%state",
                            .{ .index = @intCast(@"%index") },
                        );
                    }
                },
                .down => {
                    // dear zig, add for (range) in reverse
                    var @"%index": u32 = @"%span".start + @"%span".length.positive;
                    while (@"%index" > @"%span".start) {
                        @"%index" -= 1;
                        @"%state" = try @"%step"(
                            ctx,
                            @"%state",
                            .{ .index = @"%index" },
                        );
                    }
                },
            }
            return @"%state";
        }
        pub fn step_while(
            @"%span": @This(),
            @"%Done": type,
            ctx: anytype,
            @"%direction": @"[Down Up]"(void, void),
            @"%initial_state": anytype,
            @"%step": *const fn (
                std.mem.Allocator,
                @TypeOf(@"%initial_state"),
                SlotCore,
            ) Error!@"[Done Going]"(@"%Done", @TypeOf(@"%initial_state")),
        ) Error!@"[Done Going]"(
            struct { @"%Done", Opt(SpanCore) },
            @TypeOf(@"%initial_state"),
        ) {
            var @"%state" = @"%initial_state";
            switch (@"%direction") {
                .up => {
                    for (@"%span".start..(@"%span".start + @"%span".length.positive)) |@"%index_usize"| {
                        const @"%index": u32 = @intCast(@"%index_usize");
                        switch (try @"%step"(
                            ctx,
                            @"%state",
                            .{ .index = @"%index" },
                        )) {
                            .going => |@"%new_state"| {
                                @"%state" = @"%new_state";
                            },
                            .done => |@"%done"| {
                                return .{ .done = .{
                                    @"%done",
                                    if (P32.fromU32(
                                        @"%span".start + @"%span".length.positive - 1 - @"%index",
                                    )) |@"%rest_length"| .{
                                        .yes = .{ .start = @"%index" + 1, .length = @"%rest_length" },
                                    } else .{ .no = {} },
                                } };
                            },
                        }
                    }
                },
                .down => {
                    // dear zig, add for (range) in reverse
                    var @"%index": u32 = @"%span".start + @"%span".length.positive;
                    while (@"%index" > @"%span".start) {
                        @"%index" -= 1;
                        switch (try @"%step"(ctx, .{
                            .state = @"%state",
                            .slot = .{ .index = @"%index" },
                        })) {
                            .going => |@"%going"| {
                                @"%state" = @"%going";
                            },
                            .done => |@"%done"| {
                                return .{ .done = .{
                                    @"%done",
                                    if (P32.fromU32(@"%index" - @"%span".start)) |@"%rest_length"| .{
                                        .yes = .{ .start = @"%span".start, .length = @"%rest_length" },
                                    } else .{ .no = {} },
                                } };
                            },
                        }
                    }
                },
            }
            return .{ .going = @"%state" };
        }
    };
pub const ReferenceCount: type =
    enum(u32) {
        invalid = 0,
        leaked = std.math.maxInt(u32),
        _,

        pub const one: @This() = @fromBackingInt(1);

        pub fn increment(@"%reference_count": @This()) @This() {
            switch (@"%reference_count") {
                .invalid => unreachable,
                .leaked => return .leaked,
                _ => {
                    return @fromBackingInt(@backingInt(@"%reference_count") + 1);
                },
            }
        }
        /// assumes the given ReferenceCount is > 1
        pub fn decrement(@"%reference_count": @This()) @This() {
            switch (@"%reference_count") {
                .invalid => unreachable,
                .leaked => return .leaked,
                _ => {
                    return @fromBackingInt(@backingInt(@"%reference_count") - 1);
                },
            }
        }
    };

/// slice whose actual items are undefined
pub fn Unset_slice(@"%Item": type) type {
    return *Unset_sliceCore(@"%Item");
}
pub fn Unset_sliceCore(@"%Item": type) type {
    return struct {
        undefined_items: []@"%Item",
        reference_count: ReferenceCount,

        pub fn allocateLength(
            @"%allocator": std.mem.Allocator,
            @"%length": u32,
        ) Error!@This() {
            return .{ .undefined_items = try @"%allocator".alloc(@"%Item", @"%length") };
        }
        pub fn capacity(@"%unset_slice": @This()) u32 {
            return @intCast(@"%unset_slice".undefined_items.len);
        }
        /// the given unset slice may be invalid after
        pub fn castOrRidAndAllocate(
            @"%unset_slice": @This(),
            @"%NewItem": type,
            @"%allocator": std.mem.Allocator,
        ) Error!Unset_slice(@"%NewItem") {
            // alignment must match exactly sadly, required by Allocator.free.
            // SmpAllocator for example uses alignment for size classes.
            // The alternative would be carrying original allocation alignment
            // through all uses (Buf, Unset_slice, future collections)
            // which is a price I'm not willing to pay for a niche feature
            if (strideOf(@"%NewItem") == strideOf(@"%Item") and @alignOf(@"%NewItem") == @alignOf(
                @"%Item",
            ) and @"%unset_slice".reference_count == ReferenceCount.one) {
                return .{ .undefined_items = @as(
                    []@"%NewItem",
                    @ptrCast(@"%unset_slice".undefined_items),
                ) };
            } else {
                @"%unset_slice".dropAndDestroy(@"%allocator");
                return Unset_slice(@"%NewItem").allocateLength(@"%allocator", @"%unset_slice".capacity());
            }
        }
        pub fn clone(@"%unset_slice": *@This()) void {
            @"%unset_slice".reference_count = @"%unset_slice".reference_count.increment();
        }
        /// the given unset slice will be invalid after if reference_count == 1
        pub fn dropAndDestroy(
            @"%unset_slice": *@This(),
            @"%allocator": std.mem.Allocator,
        ) void {
            if (@"%unset_slice".reference_count == ReferenceCount.one) {
                @"%allocator".free(@"%unset_slice".undefined_items);
                @"%allocator".destroy(@"%unset_slice");
            } else {
                @"%unset_slice".reference_count = @"%unset_slice".reference_count.decrement();
            }
        }
        /// the given unset slice will be invalid after if reference_count == 1
        pub fn drop(
            @"%unset_slice": @This(),
            @"%allocator": std.mem.Allocator,
        ) void {
            if (@"%unset_slice".reference_count == ReferenceCount.one) {
                @"%allocator".free(@"%unset_slice".undefined_items);
            } else {
                @"%unset_slice".reference_count = @"%unset_slice".reference_count.decrement();
            }
        }
    };
}
/// Not thread-safe.
pub fn Buf(@"%Origin": type, @"%Item": type) type {
    return *BufCore(@"%Origin", @"%Item");
}
/// Not thread-safe.
pub fn BufCore(@"%Origin": type, @"%Item": type) type {
    return struct {
        /// Assumed to have an .items.len that fits into a u32.
        /// .items.capacity has no such constraint.
        /// if you want to directly access .items, be extra aware of
        /// - considering .counts
        /// - the ABA problem (e.g. an index to an item could point to a wrong, new item
        ///   instead of invalid memory when its index was unset and re-occupied in between)
        items: std.ArrayList(@"%Item"),
        /// has the same length and capacity as items.items.
        /// Prefer accessing through countsSlice or countsArrayList
        counts: [*]SlotCount,
        /// invariants:
        /// - .none_unset if countsSlice() is all 0
        /// - _ points to the first 0 in counts
        /// - _ points to a valid index in items
        first_unset_index: UnsetIndexOrNone,
        reference_count: ReferenceCount,

        pub const SlotCount = enum(u16) {
            leaked = std.math.maxInt(u16),
            _,

            pub const one: @This() = @fromBackingInt(1);

            pub fn increment(@"%count": @This()) @This() {
                switch (@"%count") {
                    .leaked => return .leaked,
                    _ => {
                        return @fromBackingInt(@backingInt(@"%count") + 1);
                    },
                }
            }
            /// assumes the given SlotCount is > 1
            pub fn decrement(@"%count": @This()) @This() {
                switch (@"%count") {
                    .leaked => return .leaked,
                    _ => {
                        return @fromBackingInt(@backingInt(@"%count") - 1);
                    },
                }
            }
        };
        pub const UnsetIndexOrNone = enum(u32) { none_unset = std.math.maxInt(u32), _ };

        /// usually used with std.mem.Allocator.create()
        pub const empty: @This() = .{
            .items = .empty,
            .counts = undefined,
            .first_unset_index = UnsetIndexOrNone.none_unset,
            .reference_count = .one,
        };

        pub fn lenIncludingUnset(@"%buf": @This()) u32 {
            return @intCast(@"%buf".items.items.len);
        }

        pub fn countsSlice(@"%buf": @This()) []SlotCount {
            return @"%buf".counts[0..@"%buf".items.items.len];
        }
        /// important:
        /// - updating the returned ArrayList will not update the Buf's .counts pointer!
        /// - this function assumes items and counts have the same len anf capacity _currently_
        ///   which might not be the case during an internal function call
        pub fn countsArrayList(@"%buf": @This()) std.ArrayList(SlotCount) {
            return std.ArrayList(SlotCount){
                .items = countsSlice(@"%buf"),
                .pointer_stability = @"%buf".items.pointer_stability,
                .capacity = @"%buf".items.capacity,
            };
        }

        pub fn preAllocateAtLeast(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%min_pre_allocated_length": u32,
        ) Error!void {
            var @"%counts" = @"%buf".countsArrayList();
            try @"%counts".ensureUnusedCapacity(@"%allocator", @"%min_pre_allocated_length");
            @"%buf".counts = @"%counts".items.ptr;
            try @"%buf".items.ensureUnusedCapacity(@"%allocator", @"%min_pre_allocated_length");
        }
        pub fn preAllocationRid(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
        ) Error!void {
            var @"%counts" = @"%buf".countsArrayList();
            try @"%counts".shrinkToLen(@"%allocator");
            @"%buf".counts = @"%counts".items.ptr;
            try @"%buf".items.shrinkToLen(@"%allocator");
        }
        pub fn unsetCount(@"%buf": @This()) u32 {
            return @intCast(std.mem.countScalar(SlotCount, @"%buf".countsSlice(), @fromBackingInt(0)));
        }
        pub fn setCount(@"%buf": @This()) u32 {
            return @"%buf".lenIncludingUnset() - @"%buf".unsetCount();
        }
        pub fn add(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_item": @"%Item",
        ) Error!Slot(@"%Origin", @"%Item") {
            const @"%new_index" = @"%buf".lenIncludingUnset();
            var @"%counts" = @"%buf".countsArrayList();
            const @"%new_count_ptr" = try @"%counts".addOne(@"%allocator");
            const @"%new_item_ptr" = try @"%buf".items.addOne(@"%allocator");
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            @"%buf".counts = @"%counts".items.ptr;
            @"%new_item_ptr".* = @"%new_item";
            @"%new_count_ptr".* = SlotCount.one;
            return Slot(@"%Origin", @"%Item"){ .index = @"%new_index" };
        }
        pub fn insert(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_item": @"%Item",
        ) Error!Slot(@"%Origin", @"%Item") {
            switch (@"%buf".first_unset_index) {
                .none_unset => {
                    return @"%buf".add(@"%allocator", @"%new_item");
                },
                _ => |@"%first_unset_index_enum"| {
                    const @"%first_unset_index" = @backingInt(@"%first_unset_index_enum");
                    @"%buf".items.items[@"%first_unset_index"] = @"%new_item";
                    @"%buf".countsSlice()[@"%first_unset_index"] = SlotCount.one;
                    const @"%set_slot" = Slot(@"%Origin", @"%Item"){ .index = @"%first_unset_index" };
                    @"%buf".first_unset_index = @"%buf".firstUnsetIndexStartSearchFrom(@"%first_unset_index" + 1);
                    return @"%set_slot";
                },
            }
        }
        pub fn setOrInsert(
            @"%buf": *@This(),
            ctx: anytype,
            @"%index": U32,
            @"%new_item": @"%Item",
        ) Error!Slot(@"%Origin", @"%Item") {
            if ((@"%index" >= @"%buf".items.items.len) or (@"%buf".countsSlice()[@"%index"] != @as(
                SlotCount,
                @fromBackingInt(0),
            ))) {
                return @"%buf".insert(ctx.allocator, @"%new_item");
            }
            @"%buf".items.items[@"%index"] = @"%new_item";
            @"%buf".countsSlice()[@"%index"] = SlotCount.one;
            if (@"%buf".first_unset_index == @as(UnsetIndexOrNone, @fromBackingInt(@"%index"))) {
                @"%buf".first_unset_index = @"%buf".firstUnsetIndexStartSearchFrom(@"%index" + 1);
            }
            return Slot(@"%Origin", @"%Item"){ .index = @"%index" };
        }
        fn firstUnsetIndexStartSearchFrom(@"%buf": @This(), @"%search_start_index": u32) UnsetIndexOrNone {
            return if (std.mem.findScalarPos(
                SlotCount,
                @"%buf".countsSlice(),
                @"%search_start_index",
                @fromBackingInt(0),
            )) |@"%new_first_unset_index"|
                @fromBackingInt(@as(u32, @intCast(@"%new_first_unset_index")))
            else
                .none_unset;
        }
        /// slot is invalid while resulting ptr is live
        pub fn item_ptr(@"%buf": @This(), @"%slot": Slot(@"%Origin", @"%Item")) *@"%Item" {
            return &@"%buf".items.items[@"%slot".index];
        }
        pub fn item(@"%buf": @This(), @"%slot": Slot(@"%Origin", @"%Item")) @"%Item" {
            return @"%buf".items.items[@"%slot".index];
        }
        pub fn take(
            @"%buf": *@This(),
            ctx: anytype,
            @"%slot": Slot(@"%Origin", @"%Item"),
            @"clone Item": fn (
                anytype, // @TypeOf(ctx)
                anytype, // @"%Item"
            ) void,
        ) @"%Item" {
            const @"%item" = @"%buf".item(@"%slot");
            const @"%slot_count" = @"%buf".countsSlice()[@"%slot".index];
            if (@"%slot_count" == SlotCount.one) {
                // can be optimized
                @"%buf".unsetSpanRid(.{ .start = @"%slot".index, .length = P32.one });
            } else {
                @"clone Item"(ctx, @"%item");
                @"%buf".countsSlice()[@"%slot".index] = @"%slot_count".decrement();
            }
            return @"%item";
        }
        /// returns the old item
        pub fn replace(
            @"%buf": @This(),
            @"%slot": Slot(@"%Origin", @"%Item"),
            @"%new": @"%Item",
        ) @"%Item" {
            const @"%item_ptr" = @"%buf".item_ptr(@"%slot");
            const @"%old_item" = @"%item_ptr".*;
            @"%item_ptr".* = @"%new";
            return @"%old_item";
        }
        pub fn swap(
            @"%buf": @This(),
            @"%slot_a": Slot(@"%Origin", @"%Item"),
            @"%slot_b": Slot(@"%Origin", @"%Item"),
        ) void {
            std.mem.swap(@"%Item", @"%buf".item_ptr(@"%slot_a"), @"%buf".item_ptr(@"%slot_b"));
        }
        // The given span is invalid while the returned slice is live
        pub fn spanSlice(@"%buf": @This(), @"%span": Span(@"%Origin", @"%Item")) []@"%Item" {
            return @"%buf".items.items[@"%span".start..][0..@"%span".length.positive];
        }
        // The given span is invalid while the returned slice is live
        pub fn optSpanSlice(@"%buf": @This(), @"%opt_span": Opt(Span(@"%Origin", @"%Item"))) []@"%Item" {
            return switch (@"%opt_span") {
                .no => &.{},
                .yes => |@"%span"| @"%buf".spanSlice(@"%span"),
            };
        }
        /// The returned slice is only valid while buf.items.items is live
        pub fn takeSpan(
            @"%buf": *@This(),
            @"%span": Span(@"%Origin", @"%Item"),
        ) []@"%Item" {
            // TODO
            const @"%slice" = @"%buf".spanSlice(@"%span");
            @"%buf".unsetSpanRid(.{ .start = @"%span".start, .length = @"%span".length });
            return @"%slice";
        }
        /// The returned slice is only valid while buf.items.items is live
        pub fn takeOptSpan(
            @"%buf": *@This(),
            @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
        ) []@"%Item" {
            switch (@"%opt_span") {
                .no => return []@"%Item",
                .yes => |@"%span"| {
                    return @"%buf".takeSpan(@"%span");
                },
            }
        }
        fn unsetSpanRid(@"%buf": *@This(), @"%span": SpanCore) void {
            const @"%counts_in_span" = @"%buf".countsSlice()[@"%span".start..(@"%span".start + @"%span".length.positive)];
            for (@"%counts_in_span") |*@"%count"| {
                @"%count".* = @"%count".decrement();
            }
            if (@"%span".start + @"%span".length.positive < @"%buf".lenIncludingUnset()) {
                @"%buf".first_unset_index = @fromBackingInt(@min(@backingInt(@"%buf".first_unset_index), @"%span".start));
            } else {
                // span is at the end
                @"%buf".trimUnsetEnd();
            }
        }
        fn trimUnsetEnd(@"%buf": *@This()) void {
            const @"%counts_without_trailing_0s" = std.mem.trimEnd(
                SlotCount,
                @"%buf".countsSlice(),
                &.{@fromBackingInt(0)},
            );
            if (@"%counts_without_trailing_0s".len < @"%buf".items.items.len) {
                if (@"%buf".first_unset_index == @as(UnsetIndexOrNone, @fromBackingInt(@intCast(@"%counts_without_trailing_0s".len)))) {
                    @"%buf".first_unset_index = .none_unset;
                }
                var @"%counts" = @"%buf".countsArrayList();
                @"%counts".shrinkRetainingCapacity(@"%counts_without_trailing_0s".len);
                @"%buf".counts = @"%counts".items.ptr;
                @"%buf".items.shrinkRetainingCapacity(@"%counts_without_trailing_0s".len);
            }
        }
        /// invalidates all Slots and Spans into this Buf
        pub fn clearRetainingCapacity(@"%buf": *@This()) void {
            @"%buf".items.clearRetainingCapacity();
            @"%buf".first_unset_index = .none_unset;
        }
        pub fn spanMoveToEnd(
            @"%buf": *@This(),
            ctx: anytype,
            @"%span": Span(@"%Origin", @"%Item"),
        ) Error!Span(@"%Origin", @"%Item") {
            if (@"%span".start + @"%span".length.positive == @"%buf".lenIncludingUnset()) {
                return @"%span";
            }
            // span is not at the end already
            @"%buf".unsetSpanRid(.{
                .start = @"%span".start,
                .length = @"%span".length,
            });
            try @"%buf".preAllocateAtLeast(ctx, @"%span".length.positive);
            @"%buf".items.appendSliceAssumeCapacity(@"%buf".spanSlice(@"%span"));
            if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
            return .{
                .start = @"%buf".lenIncludingUnset() - @"%span".length.positive,
                .length = @"%span".length,
            };
        }
        pub fn spanMoveToUnset(@"%buf": *@This(), @"%span": Span(@"%Origin", @"%Item")) Span(@"%Origin", @"%Item") {
            if (@"%span".start + @"%span".length.positive < @"%buf".lenIncludingUnset()) {
                return @"%span";
            }
            // span is at the end of items
            if (@"%buf".markLengthPositiveAsSet(@"%span".length)) |@"%earlier_start_to_set_from"| {
                @"%buf".items.replaceRangeAssumeCapacity(
                    @"%earlier_start_to_set_from",
                    @"%span".length.positive,
                    @"%buf".spanSlice(@"%span"),
                );
                @"%buf".unsetSpanRid(.{ .start = @"%span".start, .length = @"%span".length });
                return Span(@"%Origin", @"%Item"){
                    .start = @"%earlier_start_to_set_from",
                    .length = @"%span".length,
                };
            } else {
                return @"%span";
            }
        }
        pub fn spanAddOwnSpan(
            @"%buf": *@This(),
            ctx: anytype,
            @"%start": Span(@"%Origin", @"%Item"),
            @"%end": Span(@"%Origin", @"%Item"),
        ) Error!Span(@"%Origin", @"%Item") {
            const @"%combined_length" = @"%start".length.addAssumeNoOverflow(@"%end".length.positive);
            if (@"%start".start + @"%start".length.positive == @"%end".start) {
                return Span(@"%Origin", @"%Item"){ .start = @"%start".start, .length = @"%combined_length" };
            } else {
                const @"%moved_start" = try @"%buf".spanMoveToEnd(ctx, @"%start");
                _ = try @"%buf".spanMoveToEnd(ctx, @"%end");
                return Span(@"%Origin", @"%Item"){
                    .start = @"%moved_start".start,
                    .length = @"%combined_length",
                };
            }
        }
        fn markLengthPositiveAsSet(@"%buf": *@This(), @"%length_to_set": P32) ?u32 {
            switch (@"%buf".first_unset_index) {
                .none_unset => {
                    return null;
                },
                _ => |@"%first_unset_index_enum"| {
                    const @"%first_unset_index" = @backingInt(@"%first_unset_index_enum");
                    // can be optimized
                    // unset_iterator is now at after first_unset_index
                    // because we know there is no unset index before first_unset_index
                    var @"%unset_length_so_far": u32 = 1;
                    for (@"%buf".countsSlice()[@"%first_unset_index"..], @"%first_unset_index"..) |@"%count", @"%index"| {
                        if (@"%count" == @as(SlotCount, @fromBackingInt(0))) {
                            @"%unset_length_so_far" = 0;
                        } else {
                            @"%unset_length_so_far" += 1;
                            if (@"%unset_length_so_far" == @"%length_to_set".positive) {
                                const @"%found_start" = @"%index" + 1 - @"%length_to_set".positive;
                                @memset(@"%buf".countsSlice()[@"%found_start"..(@"%index" + 1)], SlotCount.one);
                                if (@"%first_unset_index" == @"%found_start") {
                                    @"%buf".first_unset_index = @"%buf".firstUnsetIndexStartSearchFrom(@"%first_unset_index" + 1);
                                }
                                return @"%found_start";
                            }
                        }
                    }
                    return null;
                },
            }
        }
        // add insertSlice?
        /// Assumes a slice length <= intMax(u32)
        pub fn addSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_items": []const @"%Item",
        ) Error!Opt(Span(@"%Origin", @"%Item")) {
            if (@"%new_items".len >= 1) {
                try @"%buf".preAllocateAtLeast(@"%allocator", @as(u32, @intCast(@"%new_items".len)));
                @"%buf".items.appendSliceAssumeCapacity(@"%new_items");
                if (std.math.cast(u32, @"%buf".items.items.len) == null) return error.OutOfMemory;
                return .{ .yes = .{
                    .start = @"%buf".lenIncludingUnset() - @as(u32, @intCast(@"%new_items".len)),
                    .length = P32.fromU32(@intCast(@"%new_items".len)).?,
                } };
            } else return .{ .no = {} };
        }
        // add insertIterator?
        pub fn addIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%new_items": anytype,
            @"%next_item": fn (*@TypeOf(@"%new_items")) ?@"%Item",
        ) Error!Opt(Span(@"%Origin", @"%Item")) {
            const @"%length_before_add" = @"%buf".lenIncludingUnset();
            var @"%new_items_iterator" = @"%new_items";
            while (@"%next_item"(&@"%new_items_iterator")) |@"%new_item"| {
                _ = try @"%buf".add(@"%allocator", @"%new_item");
            }
            return if (P32.fromU32(
                @"%buf".lenIncludingUnset() - @"%length_before_add",
            )) |@"%new_length"|
                .{ .yes = .{ .start = @"%length_before_add", .length = @"%new_length" } }
            else
                .{ .no = {} };
        }
        pub fn optSpanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
            @"%new_item": @"%Item",
        ) Error!Span(@"%Origin", @"%Item") {
            return switch (@"%opt_span") {
                .no => (try @"%buf".add(@"%allocator", @"%new_item")).toSpan(),
                .yes => |@"%span"| @"%buf".spanAdd(@"%allocator", @"%span", @"%new_item"),
            };
        }
        pub fn spanAdd(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin", @"%Item"),
            @"%new_item": @"%Item",
        ) Error!Span(@"%Origin", @"%Item") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            _ = try @"%buf".add(@"%allocator", @"%new_item");
            return Span(@"%Origin", @"%Item"){
                .start = @"%moved_span".start,
                .length = @"%moved_span".length.addAssumeNoOverflow(1),
            };
        }
        pub fn optSpanAddSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
            @"%new_items": []const @"%Item",
        ) Error!Opt(Span(@"%Origin", @"%Item")) {
            return switch (@"%opt_span") {
                .no => @"%buf".addSlice(@"%allocator", @"%new_items"),
                .yes => |@"%span"| .{ .yes = try @"%buf".spanAddSlice(@"%allocator", @"%span", @"%new_items") },
            };
        }
        pub fn spanAddSlice(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin", @"%Item"),
            @"%new_items": []const @"%Item",
        ) Error!Span(@"%Origin", @"%Item") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            _ = try @"%buf".addSlice(@"%allocator", @"%new_items");
            return Span(@"%Origin", @"%Item"){
                .start = @"%moved_span".start,
                .length = @"%moved_span".length.addAssumeNoOverflow(
                    @intCast(@"%new_items".len),
                ),
            };
        }
        pub fn optSpanAddIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
            @"%new_items": anytype,
            @"%next_item": fn (*@TypeOf(@"%new_items")) ?@"%Item",
        ) Error!Opt(Span(@"%Origin", @"%Item")) {
            return switch (@"%opt_span") {
                .no => @"%buf".addIterator(@"%allocator", @"%new_items", @"%next_item"),
                .yes => |@"%span"| .{
                    .yes = try @"%buf".spanAddIterator(@"%allocator", @"%span", @"%new_items", @"%next_item"),
                },
            };
        }
        pub fn spanAddIterator(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
            @"%span": Span(@"%Origin", @"%Item"),
            @"%new_items": anytype,
            @"%next_item": fn (*@TypeOf(@"%new_items")) ?@"%Item",
        ) Error!Span(@"%Origin", @"%Item") {
            const @"%moved_span" = try @"%buf".spanMoveToEnd(@"%allocator", @"%span");
            const @"%length_before_add" = @"%buf".lenIncludingUnset();
            var @"%new_items_iterator" = @"%new_items";
            while (@"%next_item"(&@"%new_items_iterator")) |@"%new_item"| {
                _ = try @"%buf".add(@"%allocator", @"%new_item");
            }
            const @"%iterated_length" = @"%buf".lenIncludingUnset() - @"%length_before_add";
            return Span(@"%Origin", @"%Item"){
                .start = @"%moved_span".start,
                .length = @"%moved_span".length.addAssumeNoOverflow(@"%iterated_length"),
            };
        }
        pub fn spanReverse(
            @"%buf": @This(),
            @"%span": Span(@"%Origin", @"%Item"),
        ) Span(@"%Origin", @"%Item") {
            std.mem.reverse(@"%Item", @"%buf".spanSlice(@"%span"));
            return @"%span";
        }
        pub fn optSpanReverse(
            @"%buf": @This(),
            @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
        ) Opt(Span(@"%Origin", @"%Item")) {
            std.mem.reverse(@"%Item", @"%buf".optSpanSlice(@"%opt_span"));
            return @"%opt_span";
        }
        /// buf is invalid after if .reference_count is 1
        pub fn intoUnsetSlice(
            @"%buf": @This(),
            @"%allocator": std.mem.Allocator,
        ) Unset_slice(@"%Item") {
            if (@"%buf".reference_count == ReferenceCount.one) {
                var @"%counts" = @"%buf".countsArrayList();
                @"%counts".deinit(@"%allocator");
                @"%buf".counts = undefined;
                return .{ .undefined_items = @"%buf".items.allocatedSlice() };
            } else {
                return .{ .undefined_items = &.{} };
            }
        }
        /// buf is invalid after if .reference_count is 1
        pub fn intoUnsetSliceAndDestroy(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
        ) Unset_slice(@"%Item") {
            if (@"%buf".reference_count == ReferenceCount.one) {
                var @"%counts" = @"%buf".countsArrayList();
                @"%counts".deinit(@"%allocator");
                @"%buf".counts = undefined;
                @"%allocator".destroy(@"%buf");
                return .{ .undefined_items = @"%buf".items.allocatedSlice() };
            } else {
                return .{ .undefined_items = &.{} };
            }
        }
        /// buf is invalid after if .reference_count is 1
        pub fn drop(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
        ) void {
            if (@"%buf".reference_count == ReferenceCount.one) {
                var @"%counts" = @"%buf".countsArrayList();
                @"%counts".deinit(@"%allocator");
                @"%buf".counts = undefined;
                @"%buf".items.deinit(@"%allocator");
            } else {
                @"%buf".reference_count = @"%buf".reference_count.decrement();
            }
        }
        /// buf is invalid after if .reference_count is 1
        pub fn dropAndDestroy(
            @"%buf": *@This(),
            @"%allocator": std.mem.Allocator,
        ) void {
            if (@"%buf".reference_count == ReferenceCount.one) {
                var @"%counts" = @"%buf".countsArrayList();
                @"%counts".deinit(@"%allocator");
                @"%buf".counts = undefined;
                @"%buf".items.deinit(@"%allocator");
                @"%allocator".destroy(@"%buf");
            } else {
                @"%buf".reference_count = @"%buf".reference_count.decrement();
            }
        }
        pub fn dropSlot(
            @"%buf": *@This(),
            ctx: anytype, // Ctx(&.{@"%Item".Bufs})
            @"%slot": Slot(@"%Origin", @"%Item"),
            @"drop Item": fn (
                anytype, // @TypeOf(ctx)
                anytype, // @"%Item"
            ) void,
        ) void {
            const @"%slot_count" = @"%buf".countsSlice()[@"%slot".index];
            if (@"%slot_count" == SlotCount.one) {
                @"drop Item"(ctx, @"%buf".item(@"%slot"));
                // can be optimized
                @"%buf".unsetSpanRid(.{ .start = @"%slot".index, .length = P32.one });
            } else {
                @"%buf".countsSlice()[@"%slot".index] = @"%slot_count".decrement();
            }
        }
        pub fn dropSpan(
            @"%buf": *@This(),
            ctx: anytype, // Ctx(&.{@"%Item".Bufs})
            @"%span": Span(@"%Origin", @"%Item"),
            @"drop Item": fn (@TypeOf(ctx), @"%Item") void,
        ) void {
            // TODO optimize
            for (@"%span".start..(@"%span".start + @"%span".length.positive)) |@"%index"| {
                @"%buf".dropSlot(ctx, .{ .index = @"%index" }, @"drop Item");
            }
        }
        pub fn clone(@"%buf": *@This()) void {
            @"%buf".reference_count = @"%buf".reference_count.increment();
        }
        pub fn cloneSlot(
            @"%buf": *@This(),
            @"%slot": Slot(@"%Origin", @"%Item"),
        ) void {
            @"%buf".counts()[@"%slot".index] = @"%buf".counts()[@"%slot".index].increment();
        }
        pub fn cloneSpan(
            @"%buf": *@This(),
            @"%span": Span(@"%Origin", @"%Item"),
        ) void {
            for (@"%span".start..(@"%span".start + @"%span".length.positive)) |@"%index"| {
                @"%buf".cloneSlot(.{ .index = @"%index" });
            }
        }
    };
}

// You may notice that even infallible functions that can be called by slug-generated code
// return `Error!...`.
// This is to avoid deeper analysis of e.g. when to `try`, when to annotate and convert errors etc.
// It also makes for a consistent `Fn` type alias.
// This might seem wasteful but I'd be surprised if optimizers couldn't reduce un-raisd errors away.
// The biggest actual issue I can forsee is that zig code calling into slug-generated zig code
// may lead to unnecessary error handling (which IMO is fine because slug code could always change to a fallible operation).
//
// If you're running the slug code in a loop, there honestly isn't much you can do on the zig side
// once you're running out of memory as you won't receive a new state value
// (and any array pointers you may have stored independently are likely stale and/or corrupted).
// For typical applications, just exiting is fine, though hopefully a little more gracefully than
// if every slug-generated zig function used `catch @panic` instead of returning an error.
// If this is not an option, try to e.g. keep deep-cloned state value "backups" or similar.
// (or just don't use slug)

pub fn p32_to_u32(@"%n": P32) U32 {
    return @"%n".positive;
}
pub fn p32_add_clamp(@"%p": P32, @"%u": U32) P32 {
    return @"%p".addClamp(@"%u");
}
pub fn p32_mul_clamp(@"%a": P32, @"%b": P32) P32 {
    return @"%a".mulClamp(@"%b");
}
pub fn p32_order(@"%left": P32, @"%right": P32) Order {
    return mathOrderToOrder(std.math.order(@"%left".positive, @"%right".positive));
}

pub fn u32_to_i32_clamp(@"%n": U32) I32 {
    return std.math.lossyCast(i32, @"%n");
}
pub fn u32_round_to_nearest_f32_else_even(@"%n": U32) F32 {
    // see i32_round_to_nearest_f32_else_even for details
    return @floatFromInt(@"%n");
}
pub fn u32_successor_clamp(@"%n": U32) P32 {
    return .{ .positive = @"%n" +| 1 };
}
pub fn u32_add_clamp(@"%a": U32, @"%b": U32) U32 {
    return @"%a" +| @"%b";
}
pub fn u32_add_i32_clamp(@"%i": I32, @"%u": U32) U32 {
    return std.math.lossyCast(u32, @as(i33, @"%u") +| @as(i33, @"%i"));
}
pub fn u32_mul_clamp(@"%a": U32, @"%b": U32) U32 {
    return @"%a" *| @"%b";
}
pub fn u32_mul_wrap(@"%a": U32, @"%b": U32) U32 {
    return @"%a" *% @"%b";
}
pub fn u32_pow_clamp(@"%base": U32, @"%exponent": P32) U32 {
    return std.math.powi(
        u32,
        @"%base",
        @"%exponent".positive,
    ) catch std.math.maxInt(u32);
}
pub fn u32_to_p32(@"%n": U32) Opt(P32) {
    return if (P32.fromU32(@"%n")) |@"%p32"| .{ .yes = @"%p32" } else .{ .no = {} };
}
pub fn u32_order(@"%left": U32, @"%right": U32) Order {
    return mathOrderToOrder(std.math.order(@"%left", @"%right"));
}

pub fn i32_to_u32(@"%i": I32) Opt(U32) {
    return if (std.math.cast(U32, @"%i")) |@"%u"| .{ .yes = @"%u" } else .{ .no = {} };
}
pub fn i32_round_to_nearest_f32_else_even(@"%n": I32) F32 {
    // - custom backend: explicit round ties to even
    //   https://codeberg.org/ziglang/zig/src/branch/master/lib/compiler_rt/float_from_int.zig#L508-L509
    // - in llvm, this compiles to sitofp which uses the default rounding mode
    //   @setRoundMode is set to strict by default which claims IEEE compliance
    //   IEEE specifies round ties to even (see §4.3.3, §7.4 in IEEE 754-2008)
    return @floatFromInt(@"%n");
}
pub fn i32_add_clamp(@"%a": I32, @"%b": I32) I32 {
    return @"%a" +| @"%b";
}
pub fn i32_mul_clamp(@"%a": I32, @"%b": I32) I32 {
    return @"%a" *| @"%b";
}
pub fn i32_pow_clamp(@"%base": I32, @"%exponent": P32) I32 {
    return std.math.powi(
        i32,
        @"%base",
        std.math.lossyCast(i32, @"%exponent".positive),
    ) catch std.math.maxInt(i32);
}
pub fn i32_negate_clamp(@"%n": I32) I32 {
    return 0 -| @"%n";
}
pub fn i32_abs_to_u32(@"%n": I32) U32 {
    return @abs(@"%n");
}
pub fn i32_order(@"%left": I32, @"%right": I32) Order {
    return mathOrderToOrder(std.math.order(@"%left", @"%right"));
}

pub fn f32_pi() F32 {
    return std.math.pi;
}
pub fn f32_negate(@"%n": F32) F32 {
    return -@"%n";
}
pub fn f32_abs(@"%n": F32) F32 {
    return @abs(@"%n");
}
pub fn f32_ln(@"%n": F32) Opt(F32) {
    if (@"%n" <= 0) {
        return .{ .no = {} };
    } else {
        const @"%ln_result" = @log(@"%n");
        return if (std.math.isFinite(@"%ln_result")) .{ .yes = @"%ln_result" } else .{ .no = {} };
    }
}
pub fn f32_exp(@"%n": F32) F32 {
    return @min(@exp(@"%n"), std.math.floatMax(f32));
}
pub fn f32_sin(@"%n": F32) F32 {
    return @sin(@"%n");
}
pub fn f32_cos(@"%n": F32) F32 {
    return @cos(@"%n");
}
pub fn f32_tan(@"%n": F32) F32 {
    return @tan(@"%n");
}
pub fn f32_atan(@"%n": F32) F32 {
    return std.math.atan(@"%n");
}
pub fn f32_round_up(@"%n": F32) F32 {
    return @ceil(@"%n");
}
pub fn f32_round_down(@"%n": F32) F32 {
    return @floor(@"%n");
}
pub fn f32_round_toward_0(@"%n": F32) F32 {
    return @trunc(@"%n");
}
pub fn f32_round_away_from_0(@"%n": F32) F32 {
    return @ceil(@abs(@"%n")) * std.math.sign(@"%n");
}
pub fn f32_round_nearest_else_away_from_0(@"%n": F32) F32 {
    return @round(@"%n");
}
pub fn f32_round_nearest_else_even(@"%n": F32) F32 {
    // your move zig. Please add an intrinsic
    const @"%mod" = std.math.modf(@"%n");
    return if (@"%mod".fpart == 0.0) @"%n" else if (@abs(@"%mod".fpart) == 0.5)
        (
            // @"%n" is on the midpoint
            if (@mod(@"%mod".ipart, 2) == 1)
                // is odd
                //  11.5 ->  12
                // -11.5 -> -12
                @round(@"%n")
            else
                // @"%n" is even
                //  10.5 ->  10, not  11
                // -10.5 -> -10, not -11
                (@round(@"%n") - std.math.sign(@"%n")))
    else
        @round(@"%n");
}
pub fn f32_round_up_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_up(@"%n"));
}
pub fn f32_round_down_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_down(@"%n"));
}
pub fn f32_round_toward_0_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_toward_0(@"%n"));
}
pub fn f32_round_away_from_0_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_away_from_0(@"%n"));
}
pub fn f32_round_nearest_else_away_from_0_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_nearest_else_away_from_0(@"%n"));
}
pub fn f32_round_nearest_else_even_to_i32_clamp(@"%n": F32) I32 {
    return std.math.lossyCast(i32, f32_round_nearest_else_even(@"%n"));
}
pub fn f32_add_clamp(@"%a": F32, @"%b": F32) F32 {
    const @"%sum" = @"%a" + @"%b";
    return if (std.math.isNegativeInf(@"%sum")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%sum")) std.math.floatMax(f32) else @"%sum";
}
pub fn f32_mul_clamp(@"%a": F32, @"%b": F32) F32 {
    const @"%product" = @"%a" * @"%b";
    return if (std.math.isNegativeInf(@"%product")) std.math.floatMin(f32) else if (std.math.isPositiveInf(@"%product")) std.math.floatMax(f32) else @"%product";
}
pub fn f32_div_clamp(@"%n": F32, @"%by": F32) F32 {
    return if (@"%by" == 0.0) 0.0 else {
        const @"%div_result" = @"%n" / @"%by";
        return if (std.math.isNegativeInf(@"%div_result")) std.math.floatMin(
            f32,
        ) else if (std.math.isPositiveInf(@"%div_result")) std.math.floatMax(
            f32,
        ) else @"%div_result";
    };
}
pub fn f32_square_root(@"%n": F32) Opt(F32) {
    return if (@"%n" < 0.0) .{ .no = {} } else .{ .yes = @sqrt(@"%n") };
}
pub fn f32_pow_i32(@"%base": F32, @"%exponent": I32) Opt(F32) {
    const @"%power" = std.math.pow(f32, @"%base", @floatFromInt(@"%exponent"));
    return if (std.math.isFinite(@"%power")) .{ .yes = @"%power" } else .{ .no = {} };
}
pub fn f32_pow(@"%base": F32, @"%exponent": F32) Opt(F32) {
    const @"%power" = std.math.pow(f32, @"%base", @"%exponent");
    return if (std.math.isFinite(@"%power")) .{ .yes = @"%power" } else .{ .no = {} };
}
pub fn f32_order(@"%left": F32, @"%right": F32) Order {
    return mathOrderToOrder(std.math.order(@"%left", @"%right"));
}

pub fn char_to_u32(@"%char": Char) U32 {
    return @"%char";
}

pub fn str_utf8_length(@"%str": Str) P32 {
    return @"%str".utf8_byte_count_p32();
}
pub fn str_char_count(@"%str": Str) P32 {
    return @"%str".codepoint_count_p32();
}
pub fn str_start(@"%str": Str) struct { Char, Opt(Str) } {
    const @"%split" = @"%str".splitStart();
    return .{
        .start = @"%split".start,
        .after = if (Str.fromUtf8View(@"%split".after)) |@"%after"| .{ .yes = @"%after" } else .{ .no = {} },
    };
}
pub fn str_end(@"%str": Str) struct { Opt(Str), Char } {
    const @"%split" = @"%str".splitEnd();
    return .{
        if (Str.fromUtf8View(@"%split".@"0")) |@"%before"| .{ .yes = @"%before" } else .{ .no = {} },
        @"%split".@"1",
    };
}

pub fn choice_empty_to(
    @"%Result": type,
    @"%impossible": @"[]",
) @"%Result" {
    return switch (@"%impossible") {};
}

// A note about ctx: anytype:
// - it's a struct
// - it contains `allocator: std.mem.Allocator`
// - it contains necessary bufs in the style
//   @"buf(slug-origin-type slug-item-type)": *BufCore(Origin, Item) (noalias)
//   **Important**: Don't forget to &reference the passed buf values
// - it contains necessary drop operations in the style
//   @"drop Type_variable_name": fn(@TypeOf(ctx), @"%Type_variable_name") void
// - it contains necessary clone operations in the style
//   @"clone Type_variable_name": fn(@TypeOf(ctx), @"%Type_variable_name") void
//
// why aren't these specified as precise types? Major reasons are
// - passing a struct with extra fields should be allowed
// - passing the same type but to different functions should be allowed (zig structs are not structural)
// - (a slightly annoying amount of typing)
//
// I think it would be vaguely possible to represent every ctx type as something like
// `Ctx(.{@"%All_type_variables"}, .{ struct{ ..all implicit origin-item-pair types.. } })`
// but I don't see much of a benefit.

pub fn slot_index(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%slot": Slot(@"%Origin".Ty, @"%Item".Ty),
) U32 {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    @"%buf".dropSlot(ctx, @"%slot", @"%Item".drop);
    return @"%slot".index;
}
pub fn slot_to_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    @"%slot": Slot(@"%Origin".Ty, @"%Item".Ty),
) Span(@"%Origin".Ty, @"%Item".Ty) {
    return @"%slot".toSpan();
}

pub fn span_start_index(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) U32 {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    @"%buf".dropSpan(ctx, @"%span", @"%Item".drop);
    return @"%span".start;
}
pub fn span_length(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) P32 {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    @"%buf".dropSpan(ctx, @"%span", @"%Item".drop);
    return @"%span".length;
}
pub fn opt_span_length(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
) U32 {
    return switch (@"%opt_span") {
        .no => 0,
        .yes => |@"%span"| span_length(@"%Item", @"%Origin", ctx, @"%span").positive,
    };
}
pub fn span_start(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    _: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) struct {
    Slot(@"%Origin".Ty, @"%Item".Ty),
    Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
} {
    return @"%span".splitStart();
}
pub fn span_end(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    _: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) struct {
    Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    Slot(@"%Origin".Ty, @"%Item".Ty),
} {
    return @"%span".splitEnd();
}
pub fn span_start_of_length_positive(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    _: anytype,
    @"%length": P32,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) struct {
    Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    Span(@"%Origin".Ty, @"%Item".Ty),
} {
    return @"%span".splitAfterLengthPositive(@"%length");
}
pub fn span_end_of_length_positive(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    _: anytype,
    @"%length": P32,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) struct {
    Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    Span(@"%Origin".Ty, @"%Item".Ty),
} {
    return @"%span".splitBeforeEndLengthPositive(@"%length");
}
pub fn opt_span_step(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%State": SlugType,
    ctx: anytype,
    @"%direction": @"[Down Up]"(void, void),
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%state": @"%State".Ty,
    @"%step": *const fn (
        Ctx(&.{ @"%Item".Bufs, @"%Origin".Bufs, @"%State".Bufs, CtxBuf(@"%Origin", @"%Item") }),
        @"%State",
        Slot(@"%Origin".Ty, @"%Item".Ty),
    ) Error!@"%State".Ty,
) Error!@"%State".Ty {
    return switch (@"%opt_span") {
        .no => @"%state",
        .yes => |@"%span"| @"%span".step(ctx, @"%direction", @"%state", @"%step"),
    };
}
pub fn span_step(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%State": SlugType,
    ctx: anytype,
    @"%direction": @"[Down Up]"(void, void),
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%state": @"%State".Ty,
    @"%step": *const fn (
        Ctx(&.{ @"%Item".Bufs, @"%Origin".Bufs, @"%State".Bufs, CtxBuf(@"%Origin", @"%Item") }),
        @"%State",
        Slot(@"%Origin".Ty, @"%Item".Ty),
    ) Error!@"%State".Ty,
) Error!@"%State".Ty {
    return @"%span".step(ctx, @"%direction", @"%state", @"%step");
}
pub fn opt_span_step_while(
    comptime @"%Done": SlugType,
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%State": SlugType,
    ctx: anytype,
    @"%direction": @"[Down Up]"(void, void),
    @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
    @"%state": @"%State",
    @"%step": *const fn (
        Ctx(&.{ @"%Item".Bufs, @"%Origin".Bufs, @"%State".Bufs, CtxBuf(@"%Origin", @"%Item") }),
        @"%State",
        Slot(@"%Origin".Ty, @"%Item".Ty),
    ) Error!@"[Done Going]"(@"%Done", @"%State"),
) Error!@"[Done Going]"(
    struct { @"%Done".Ty, Opt(Span(@"%Origin".Ty, @"%Item".Ty)) },
    @"%State".Ty,
) {
    return switch (@"%opt_span") {
        .no => .{ .going = @"%state" },
        .yes => |@"%span"| @"%span".step_while(@"%Done", ctx, @"%direction", @"%state", @"%step"),
    };
}
pub fn span_step_while(
    comptime @"%Done": SlugType,
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%State": SlugType,
    ctx: anytype,
    @"%direction": @"[Down Up]"(void, void),
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%state": @"%State",
    @"%step": *const fn (
        Ctx(&.{ @"%Done".Bufs, @"%Item".Bufs, @"%Origin".Bufs, @"%State".Bufs, CtxBuf(@"%Origin", @"%Item") }),
        @"%State",
        Slot(@"%Origin", @"%Item"),
    ) Error!@"[Done Going]"(@"%Done", @"%State"),
) Error!@"[Done Going]"(
    struct { @"%Done".Ty, Opt(Span(@"%Origin".Ty, @"%Item".Ty)) },
    @"%State".Ty,
) {
    return @"%span".step_while(@"%Done", ctx, @"%direction", @"%state", @"%step");
}

pub fn buf_empty(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
) Error!Buf(@"%Origin".Ty, @"%Item".Ty) {
    const @"%buf" = try ctx.allocator.create(BufCore(@"%Origin".Ty, @"%Item".Ty));
    @"%buf".* = .empty;
    return @"%buf";
}
pub fn buf_reuse(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%": Unset_slice(@"%Item".Ty),
) Buf(@"%Origin".Ty, @"%Item".Ty) {
    var @"%items" = std.ArrayList(@"%Item").fromOwnedSlice(@"%".slice.undefined_items);
    @"%items".clearRetainingCapacity();
    const @"%buf" = try ctx.allocator.create(BufCore(@"%Origin".Ty, @"%Item".Ty));
    @"%buf".* = .{
        .items = @"%items",
        .counts = undefined,
        .first_unset_index = .none_unset,
    };
    return @"%buf";
}
pub fn buf_pre_allocate_at_least(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%buf": Buf(@"%Origin".Ty, @"%Item".Ty),
    @"%length": u32,
) Error!void {
    try @"%buf".preAllocateAtLeast(ctx.allocator, @"%length");
}
pub fn buf_try_pre_allocate_at_least(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%buf": Buf(@"%Origin".Ty, @"%Item".Ty),
    @"%length": u32,
) @"[Out-of-memory Success]"(void, void) {
    @"%buf".preAllocateAtLeast(ctx.allocator, @"%length") catch {
        return .{ .out_of_memory = {} };
    };
    return .{ .success = {} };
}
pub fn buf_pre_allocation_rid(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%buf": Buf(@"%Origin".Ty, @"%Item".Ty),
) Error!void {
    try @"%buf".preAllocationRid(ctx.allocator);
    return @"%buf";
}
pub fn insert(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%new": @"%Item".Ty,
) Error!Slot(@"%Origin".Ty, @"%Item".Ty) {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    return @"%buf".insert(ctx.allocator, @"%new");
}
pub fn set_or_insert(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%index": U32,
    @"%new": @"%Item".Ty,
) Error!Slot(@"%Origin".Ty, @"%Item".Ty) {
    var @"%bufmut" = ctxBuf(ctx, @"%Origin", @"%Item");
    return @"%bufmut".setOrInsert(ctx, @"%index", @"%new");
}
pub fn add(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%new": @"%Item".Ty,
) Error!Slot(@"%Origin".Ty, @"%Item".Ty) {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    return @"%buf".add(ctx.allocator, @"%new");
}
pub fn add_str_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%new": Str,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buf" = @field(ctx, ctxBufName(@"%Origin".name, "char"));
    const @"%new_span" = try @"%buf".addIterator(
        ctx.allocator,
        @"%new".utf8.iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
    return @"%new_span".yes;
}
pub fn item(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%slot": Slot(@"%Origin".Ty, @"%Item".Ty),
) @"%Item".Ty {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    const @"%item" = @"%buf".take(ctx, @"%slot", @"%Item".drop);
    return @"%item";
}
pub fn replace(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%slot": Slot(@"%Origin".Ty, @"%Item".Ty),
    @"%new": @"%Item".Ty,
) @"%Item".Ty {
    const @"%old_item" = ctxBuf(ctx, @"%Origin", @"%Item").replace(@"%slot", @"%new");
    return @"%old_item";
}
pub fn swap(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%slot_a": Slot(@"%Origin".Ty, @"%Item".Ty),
    @"%slot_b": Slot(@"%Origin".Ty, @"%Item".Ty),
) void {
    ctxBuf(ctx, @"%Origin", @"%Item").swap(@"%slot_a", @"%slot_b");
}

pub fn opt_span_add(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%new": @"%Item".Ty,
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    return @"%buf".optSpanAdd(ctx.allocator, @"%span", @"%new");
}
pub fn span_add(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%new": @"%Item".Ty,
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    return @"%buf".spanAdd(ctx.allocator, @"%span", @"%new");
}
pub fn span_add_str_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, Char),
    @"%new": Str,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buf" = @field(ctx, ctxBufName(@"%Origin".name, "char"));
    return @"%buf".spanAddIterator(
        ctx.allocator,
        @"%span",
        @"%new".utf8.iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
}
pub fn opt_span_add_str_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Opt(Span(@"%Origin".Ty, Char)),
    @"%new": Str,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buf" = @field(ctx, ctxBufName(@"%Origin".name, "char"));
    const @"%combined_span" = try @"%buf".optSpanAddIterator(
        ctx.allocator,
        @"%span",
        @"%new".utf8.iterator(),
        std.unicode.Utf8Iterator.nextCodepoint,
    );
    return @"%combined_span".yes;
}
// is there a more correct way?
const u32_max_print_len = std.fmt.count("{}", .{std.math.maxInt(U32)});
pub fn span_add_u32_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, Char),
    @"%new": U32,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buffer": [u32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%new", 10, std.fmt.Case.lower, .{});
    return span_add_str_chars(
        @"%Origin",
        ctx,
        @"%span",
        Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    );
}
pub fn opt_span_add_u32_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Opt(Span(@"%Origin".Ty, Char)),
    @"%new": U32,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buffer": [u32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%new", 10, std.fmt.Case.lower, .{});
    const @"%combined" = try opt_span_add_str_chars(
        @"%Origin",
        ctx,
        @"%span",
        Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    );
    return .{ .buf = @"%combined".buf, .span = @"%combined".span };
}
// is there a more correct way?
const i32_max_print_len = @max(
    std.fmt.count("{}", .{std.math.minInt(I32)}),
    std.fmt.count("{}", .{std.math.maxInt(I32)}),
);
pub fn span_add_i32_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, Char),
    @"%new": I32,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buffer": [i32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%new", 10, std.fmt.Case.lower, .{});
    return span_add_str_chars(
        @"%Origin",
        ctx,
        @"%span",
        Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    );
}
pub fn opt_span_add_i32_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Opt(Span(@"%Origin".Ty, Char)),
    @"%new": I32,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buffer": [i32_max_print_len]u8 = undefined;
    const @"%buffer_exclusive_end" = std.fmt.printInt(&@"%buffer", @"%new", 10, std.fmt.Case.lower, .{});
    const @"%combined" = try opt_span_add_str_chars(
        Char,
        @"%Origin",
        ctx,
        @"%span",
        Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%buffer"[0..@"%buffer_exclusive_end"])).?,
    );
    return .{ .buf = @"%combined".buf, .span = @"%combined".span.yes };
}
const f32_max_decimal_print_len =
    std.fmt.float.bufferSize(std.fmt.float.Mode.decimal, F32);
pub fn span_add_f32_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, Char),
    @"%new": F32,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buffer": [f32_max_decimal_print_len]u8 = undefined;
    const @"%used_buffer_slice" = std.fmt.float.render(
        &@"%buffer",
        @"%new",
        .{ .mode = .decimal, .precision = null },
    ) catch unreachable;
    return span_add_str_chars(
        @"%Origin",
        ctx,
        @"%span",
        Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%used_buffer_slice")).?,
    );
}
pub fn opt_span_add_f32_chars(
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Opt(Span(@"%Origin".Ty, Char)),
    @"%new": F32,
) Error!Span(@"%Origin".Ty, Char) {
    var @"%buffer": [f32_max_decimal_print_len]u8 = undefined;
    const @"%used_buffer_slice" = std.fmt.float.render(
        &@"%buffer",
        @"%new",
        .{ .mode = .decimal, .precision = null },
    ) catch unreachable;
    const @"%combined" = try opt_span_add_str_chars(
        @"%Origin",
        ctx,
        @"%span",
        Str.fromUtf8View(std.unicode.Utf8View.initUnchecked(@"%used_buffer_slice")).?,
    );
    return @"%combined".yes;
}
pub fn span_add_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%Source_origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%source": Span(@"%Source_origin".Ty),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    var @"%source_buf" = ctxBuf(ctx, @"%Source_origin", @"%Item");
    const @"%source_slice" = try @"%source_buf".takeSpan(ctx, @"%source");
    var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
    return @"%buf".spanAddSlice(ctx, @"%span", @"%source_slice");
}
pub fn span_add_opt_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%Source_origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%opt_source": Opt(Span(@"%Source_origin".Ty)),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    switch (@"%opt_source") {
        .no => {
            return .{ .no = {} };
        },
        .yes => |@"%source"| {
            return span_add_span(@"%Item", @"%Origin", @"%Source_origin", ctx, @"%span", @"%source");
        },
    }
}
pub fn opt_span_add_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%Source_origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%source": Span(@"%Source_origin".Ty, @"%Item".Ty),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    switch (@"%opt_span") {
        .no => {
            var @"%source_buf" = ctxBuf(ctx, @"%Source_origin", @"%Item");
            const @"%source_slice" = try @"%source_buf".takeSpan(ctx, @"%source");
            var @"%buf" = ctxBuf(ctx, @"%Origin", @"%Item");
            const @"%combined_span" = try @"%buf".addSlice(ctx, @"%source_slice");
            return .{
                .source = @"%source",
                .span = @"%combined_span".yes,
                .buf = ctxBuf(ctx, @"%Origin", @"%Item"),
            };
        },
        .yes => |@"%span"| {
            return span_add_span(@"%Item", @"%Origin", @"%Source_origin", ctx, @"%span", @"%source");
        },
    }
}
pub fn opt_span_add_opt_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    comptime @"%Source_origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%opt_source": Opt(Span(@"%Source_origin".Ty, @"%Item".Ty)),
) Error!Opt(Span(@"%Origin".Ty, @"%Item".Ty)) {
    switch (@"%opt_source") {
        .no => {
            return .{ .no = {} };
        },
        .yes => |@"%source"| {
            return opt_span_add_span(@"%Item", @"%Origin", @"%Source_origin", ctx, @"%opt_span", @"%source");
        },
    }
}
pub fn span_add_own_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%start": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%end": Span(@"%Origin".Ty, @"%Item".Ty),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    return ctxBuf(ctx, @"%Origin", @"%Item").spanAddOwnSpan(ctx, @"%start", @"%end");
}
pub fn opt_span_add_own_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_start": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%end": Span(@"%Origin".Ty, @"%Item".Ty),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    switch (@"%opt_start") {
        .no => return @"%end",
        .yes => |@"%start"| {
            return ctxBuf(ctx, @"%Origin", @"%Item").spanAddOwnSpan(ctx, @"%start", @"%end");
        },
    }
}
pub fn span_add_own_opt_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%start": Span(@"%Origin".Ty, @"%Item".Ty),
    @"%opt_end": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    switch (@"%opt_end") {
        .no => return @"%start",
        .yes => |@"%end"| {
            return ctxBuf(ctx, @"%Origin", @"%Item").spanAddOwnSpan(ctx, @"%start", @"%end");
        },
    }
}
pub fn opt_span_add_own_opt_span(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_start": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%opt_end": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
) Error!Span(@"%Origin".Ty, @"%Item".Ty) {
    switch (@"%opt_start") {
        .no => return @"%opt_end",
        .yes => |@"%start"| {
            switch (@"%opt_end") {
                .no => return .{ .yes = @"%start" },
                .yes => |@"%end"| {
                    const @"%combined_span" = try ctxBuf(ctx, @"%Origin", @"%Item").spanAddOwnSpan(ctx, @"%start", @"%end");
                    return .{ .yes = @"%combined_span" };
                },
            }
        },
    }
}
pub fn span_move_to_unset(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) Span(@"%Origin".Ty, @"%Item".Ty) {
    return ctxBuf(ctx, @"%Origin", @"%Item").spanMoveToUnset(@"%span");
}
pub fn opt_span_move_to_unset(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
) Opt(Span(@"%Origin".Ty, @"%Item".Ty)) {
    switch (@"%opt_span") {
        .no => return .{ .no = {} },
        .yes => |@"%span"| {
            const @"%moved_span" = ctxBuf(ctx, @"%Origin", @"%Item").spanMoveToUnset(@"%span");
            return .{ .yes = @"%moved_span" };
        },
    }
}
pub fn span_move_to_end(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) Span(@"%Origin".Ty, @"%Item".Ty) {
    const @"%moved_span" = ctxBuf(ctx, @"%Origin", @"%Item").spanMoveToEnd(ctx, @"%span");
    return @"%moved_span";
}
pub fn opt_span_move_to_end(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
) Opt(Span(@"%Origin".Ty, @"%Item".Ty)) {
    switch (@"%opt_span") {
        .no => return .{ .no = {} },
        .yes => |@"%span"| {
            const @"%moved_span" = ctxBuf(ctx, @"%Origin", @"%Item").spanMoveToEnd(ctx, @"%span");
            return .{ .yes = @"%moved_span" };
        },
    }
}
pub fn span_reverse(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) void {
    ctxBuf(ctx, @"%Origin", @"%Item").spanReverse(@"%span");
    ctxBuf(ctx, @"%Origin", @"%Item").dropSpan(ctx.allocator, ctx, @"%span", @"%Origin".drop);
}
pub fn opt_span_reverse(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin", @"%Item")),
) void {
    switch (@"%opt_span") {
        .no => {},
        .yes => |@"%span"| {
            ctxBuf(ctx, @"%Origin", @"%Item").optSpanReverse(@"%span");
        },
    }
}
pub fn span_sort(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%order": *const fn (Ctx(&.{@"%Item".Bufs}), @"%Item".Ty, @"%Item".Ty) Error!Order,
    @"%span": Span(@"%Origin".Ty, @"%Item".Ty),
) Error!void {
    // TODO
    const @"%SortState" = struct {
        buf_items: []@"%Item",
        order: @TypeOf(@"%order"),
        ctx: @TypeOf(ctx),
        is_out_of_memory: *bool,
    };
    const @"%span_slice_offsets" = try ctx.allocator.alloc(usize, @"%span".length.positive);
    for (@"%span_slice_offsets", 0..) |*@"%span_slice_offset_ptr", @"%offset"| {
        @"%span_slice_offset_ptr".* = @"%offset";
    }
    var @"%is_out_of_memory" = false;
    std.sort.block(
        usize,
        @"%span_slice_offsets",
        @"%SortState"{
            .buf_items = ctxBuf(ctx, @"%Origin", @"%Item").items.items,
            .order = @"%order",
            .ctx = ctx,
            .is_out_of_memory = &@"%is_out_of_memory",
        },
        struct {
            fn f(@"%state": @"%SortState", @"%left_offset": usize, @"%right_offset": usize) bool {
                const @"%left" = &@"%state".buf_items[@"%left_offset"];
                const @"%right" = &@"%state".buf_items[@"%right_offset"];
                const @"%ordered" = @"%state".order(
                    @"%state".ctx,
                    .{ .left = @"%left".*, .right = @"%right".* },
                ) catch {
                    @"%state".is_out_of_memory.* = true;
                    // returning false would fail an invariant of std.sort.block
                    return @"%left_offset" < @"%right_offset";
                };
                @"%left".* = @"%ordered".left;
                @"%right".* = @"%ordered".right;
                return switch (@"%ordered".order) {
                    .less => true,
                    else => false,
                };
            }
        }.f,
    );
    if (@"%is_out_of_memory") {
        ctx.allocator.free(@"%span_slice_offsets");
        return error.OutOfMemory;
    }
    const @"%span_slice" = ctxBuf(ctx, @"%Origin", @"%Item").spanSlice(@"%span");
    for (@"%span_slice_offsets", 0..) |@"%sorted_offset", @"%offset"| {
        if (@"%sorted_offset" > @"%offset") {
            std.mem.swap(
                @"%Item",
                &@"%span_slice"[@"%offset"],
                &@"%span_slice"[@"%sorted_offset"],
            );
        }
    }
    ctx.allocator.free(@"%span_slice_offsets");
    // TODO should probably be done earlier
    ctxBuf(ctx, @"%Origin", @"%Item").dropSpan(ctx, @"%span", @"%Item".drop);
}
pub fn opt_span_sort(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%opt_span": Opt(Span(@"%Origin".Ty, @"%Item".Ty)),
    @"%order": *const fn (Ctx(&.{@"%Item".Bufs}), @"%Item".Ty, @"%Item".Ty) Error!Order,
) Error!void {
    switch (@"%opt_span") {
        .no => {
            return .{ .no = {} };
        },
        .yes => |@"%span"| {
            try span_sort(@"%Item", @"%Origin", ctx, @"%span", @"%order");
        },
    }
}
pub fn buf_to_unset(
    comptime @"%Item": SlugType,
    comptime @"%Origin": SlugType,
    ctx: anytype,
    @"%buf": Buf(@"%Origin".Ty, @"%Item".Ty),
) Unset_slice(@"%Item") {
    return @"%buf".intoUnsetSliceAndDestroy(ctx.allocator);
}
pub fn unset_slice_allocate_length(
    comptime @"%Item": SlugType,
    ctx: anytype,
    @"%length": U32,
) Error!Unset_slice(@"%Item".Ty) {
    const @"%unset_slice" = ctx.allocator.create(Unset_sliceCore(@"%Item".Ty));
    @"%unset_slice".* = Unset_slice(@"%Item".Ty).allocateLength(ctx, @"%length");
    return @"%unset_slice";
}
pub fn unset_slice_cast_or_rid_and_allocate(
    comptime @"%Item": SlugType,
    comptime @"%New_item": SlugType,
    ctx: anytype,
    @"%unset_slice": Unset_slice(@"%Item".Ty),
) Error!Unset_slice(@"%New_item") {
    return @"%unset_slice".castOrRidAndAllocate(@"%New_item".Ty, ctx.allocator);
}
