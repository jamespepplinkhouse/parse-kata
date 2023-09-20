const std = @import("std");
const json = std.json;
const Allocator = std.mem.Allocator;
const ThreadPool = std.Thread.Pool;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const args = try std.process.argsAlloc(allocator);

    if (args.len != 3) {
        std.debug.print("Usage: {} <input_file> <output_file>\n", .{args[0]});
        std.os.exit(1);
    }

    const input_file_path = args[1];
    const output_file_path = args[2];
    try processInputFile(allocator, input_file_path, output_file_path);
    std.process.argsFree(allocator, args);
}

fn processInputFile(allocator: Allocator, input_file_path: []const u8, output_file_path: []const u8) !void {
    const in_file = try std.fs.cwd().openFile(input_file_path, .{});
    defer in_file.close();

    const out_file = try std.fs.cwd().createFile(output_file_path, .{});
    defer out_file.close();

    const options = ThreadPool.Options{ .n_jobs = 8, .allocator = allocator };
    const thread_pool = try ThreadPool.init(options);
    defer thread_pool.deinit();

    var buf: [4096]u8 = undefined;
    while (try in_file.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const result = try thread_pool.spawn(allocator, processLine, .{line});
        try out_file.writer().print("{}\n", .{result});
    }
}

fn processLine(allocator: *Allocator, line: []const u8) ![]const u8 {
    const fields = std.mem.tokenize(line, "\t");
    const json_str = std.mem.span(fields[3]);
    const json_parser = json.Parser.init(allocator);
    const json_value = try json_parser.parse(json_str);
    const title_value = json_value.Object.get("title") orelse return error.MissingTitleField;
    return title_value.String;
}
