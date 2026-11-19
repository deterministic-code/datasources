import { run } from "./run.ts";

const lines = await run();
for (const line of lines) console.log(line);
