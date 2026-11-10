const std = @import("std");
const slug = @import("slug.zig");

pub fn main(init: std.process.Init) !void {
    const Origin = enum {};
    var buf: slug.BufCore(Origin, slug.Char) = .empty;
    const greeting = try slug.greet(
        .{ .Ty = Origin, .name = "Origin" },
        .{ .allocator = init.arena.allocator(), .@"buf(Origin char)" = &buf },
        slug.Str.fromComptime("world"),
    );
    for (buf.spanSlice(greeting)) |greeting_char| {
        std.debug.print("{u}", .{greeting_char});
    }
    buf.drop(init.arena.allocator());
}
