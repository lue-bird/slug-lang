const std = @import("std");
const slug = @import("slug.zig");

const IoState = struct { io: std.Io, writer: *std.Io.File.Writer };

pub fn main(init: std.process.Init) !void {
    var writer_buffer: [128]u8 = undefined;
    var writer = std.Io.File.stdout().writer(
        init.io,
        &writer_buffer,
    );
    _ = try slug.print_random_quote(
        .{ .Ty = IoState, .name = "Io-state" },
        .{ .allocator = init.arena.allocator() },
        .{
            .io = IoState{ .io = init.io, .writer = &writer },
            .stdout_flush = stdout_flush,
            .stdout_write_char_slice = stdout_write_char_slice,
            .http_get = http_get,
        },
    );
}
fn stdout_flush(_: slug.Ctx(&.{}), io: IoState) slug.Error!void {
    io.writer.*.flush() catch {};
}
fn stdout_write_char_slice(
    ctx: slug.Ctx(&.{slug.CtxBuf(slug.SlugType.empty, slug.SlugType.char)}),
    io: IoState,
    span: slug.Opt(slug.Span(void, slug.Char)),
) slug.Error!void {
    var chars = slug.ctxBuf(ctx, slug.SlugType.empty, slug.SlugType.char);
    var writer = io.writer;
    for (chars.optSpanSlice(span)) |char| {
        writer.interface.print("{u}", .{char}) catch {};
    }
    // functions passed to slug code must still clone and drop when necessary
    chars.dropOptSpan(ctx, span, slug.SlugType.char.drop);
}
fn http_get(
    ctx: slug.Ctx(&.{slug.CtxBuf(slug.SlugType.empty, slug.SlugType.char)}),
    io: IoState,
    url: slug.Str,
) slug.Error!slug.Opt(slug.Span(void, slug.Char)) {
    var chars = slug.ctxBuf(ctx, slug.SlugType.empty, slug.SlugType.char);
    if (http_get_request(io.io, ctx.allocator, url.utf8.bytes)) |http_response_body| {
        const span = try chars.addIterator(
            ctx.allocator,
            http_response_body.iterator(),
            std.unicode.Utf8Iterator.nextCodepoint,
        );
        return span;
    } else |err| {
        std.debug.print("{}", .{err});
        return .{ .no = {} };
    }
}
fn http_get_request(
    io: std.Io,
    allocator: std.mem.Allocator,
    url: []const u8,
) !std.unicode.Utf8View {
    var http_client = std.http.Client{ .io = io, .allocator = allocator };
    var response_writer = std.Io.Writer.Allocating.init(allocator);
    _ = try http_client.fetch(.{
        .location = .{ .url = url },
        .response_writer = &response_writer.writer,
    });
    const response_body = try response_writer.toOwnedSlice();
    http_client.deinit();
    return std.unicode.Utf8View.init(response_body);
}
