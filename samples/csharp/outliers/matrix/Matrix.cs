var log = Environment.GetEnvironmentVariable("E2E_MATRIX_LOG") ?? throw new InvalidOperationException("E2E_MATRIX_LOG");
var lines = await Program.Run();
Directory.CreateDirectory(Path.GetDirectoryName(log)!);
await File.WriteAllTextAsync(log, lines.Length == 0 ? "" : string.Join('\n', lines) + "\n");
