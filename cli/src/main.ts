import { inquirerPrompt, nodeFs, spawnRunner } from "./io.ts";
import { run } from "./run.ts";

const code = await run(process.argv.slice(2), {
  fs: nodeFs,
  run: spawnRunner,
  log: (line) => console.log(line),
  err: (line) => console.error(line),
  prompt: inquirerPrompt,
});
process.exit(code);
