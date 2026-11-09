const std = @import("std");
const slug = @import("slug.zig");

pub fn main(init: std.process.Init) !void {
    const sum = try slug.run(.{ .allocator = init.arena.allocator() });
    std.debug.print("The sum is {}.\n", .{sum});
}
