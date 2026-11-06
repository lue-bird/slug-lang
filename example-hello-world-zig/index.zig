const std = @import("std");
const slug = @import("slug.zig");

pub fn main(init: std.process.Init) !void {
    var buf = slug.Buf(enum {}, slug.Char).empty;
    const greeting = try slug.greet(
        @TypeOf(buf).origin,
        .{
            .allocator = init.arena.allocator(),
            .@"buf(Origin char)" = &buf,
        },
        slug.Str.fromComptime("world"),
    );
    for (buf.spanSlice(greeting)) |greeting_char| {
        std.debug.print("{u}", .{greeting_char});
    }
    buf.drop(init.arena.allocator());
}
