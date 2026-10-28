import { closeSync, openSync, unlinkSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(fileURLToPath(new URL(".", import.meta.url)), "..");

const sleep = (ms: number) => {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
};

const withLock = (service: string, run: () => void) => {
  const path = join(tmpdir(), `datasources-compose-${service}.lock`);
  const started = Date.now();
  for (;;) {
    try {
      const fd = openSync(path, "wx");
      try {
        run();
        return;
      } finally {
        closeSync(fd);
        try {
          unlinkSync(path);
        } catch {
          /* gone */
        }
      }
    } catch {
      if (Date.now() - started > 120_000) throw new Error(`compose lock timed out for ${service}`);
      sleep(150);
    }
  }
};

const services = {
  mysql: { env: "MYSQL_URL", publish: "MYSQL_PUBLISH_PORT", port: 3306 },
  postgres: { env: "POSTGRES_URL", publish: "POSTGRES_PUBLISH_PORT", port: 5432 },
} as const;

const compose = (...args: string[]) =>
  execFileSync("docker", ["compose", "-p", "datasources", "-f", "docker/compose.yml", ...args], {
    cwd: root,
    stdio: "inherit",
  });

const portFree = (port: number) => {
  try {
    execFileSync(
      process.execPath,
      ["-e", `const s=require("net").createServer();s.on("error",()=>process.exit(1));s.listen(${port},"0.0.0.0",()=>s.close(()=>process.exit(0)));`],
      { stdio: "ignore", timeout: 2000 },
    );
    return true;
  } catch {
    return false;
  }
};

const runningPort = (service: "mysql" | "postgres") => {
  try {
    const out = execFileSync(
      "docker",
      ["port", `datasources-${service}-1`, `${services[service].port}/tcp`],
      { encoding: "utf8" },
    ).trim();
    const match = out.split("\n").map((line) => line.match(/:(\d+)$/)).find((found) => found);
    return match ? Number(match[1]) : undefined;
  } catch {
    return undefined;
  }
};

const configuredPort = (service: "mysql" | "postgres") => {
  const raw = process.env[services[service].env];
  if (!raw) return services[service].port;
  try {
    const port = Number(new URL(raw).port);
    return port > 0 ? port : services[service].port;
  } catch {
    return services[service].port;
  }
};

const localUrl = (service: "mysql" | "postgres") => {
  const raw = process.env[services[service].env];
  if (!raw) return true;
  try {
    const host = new URL(raw).hostname;
    return host === "127.0.0.1" || host === "localhost" || host === "::1";
  } catch {
    return true;
  }
};

const choosePort = (service: "mysql" | "postgres", skip: Set<number>) => {
  const running = runningPort(service);
  if (running && !skip.has(running)) return running;
  const start = configuredPort(service);
  for (let port = start; port < start + 100; port++) {
    if (!skip.has(port) && portFree(port)) return port;
  }
  throw new Error(`${service}: no free host port from ${start} to ${start + 99}`);
};

const publish = (service: "mysql" | "postgres", port: number) => {
  const spec = services[service];
  const previous = configuredPort(service);
  process.env[spec.publish] = String(port);
  if (port !== previous) console.log(`${service}: ${previous} is taken, using ${port}`);
  const raw = process.env[spec.env];
  if (!raw || !localUrl(service)) return;
  try {
    const url = new URL(raw);
    if (Number(url.port || spec.port) === port) return;
    url.port = String(port);
    process.env[spec.env] = url.toString();
  } catch {
    /* leave the caller's url */
  }
};

const up = (service: "mysql" | "postgres") => {
  withLock(service, () => {
    const skip = new Set<number>();
    let last: unknown;
    for (let attempt = 0; attempt < 5; attempt++) {
      const port = choosePort(service, skip);
      publish(service, port);
      try {
        compose("up", "-d", "--wait", service);
        return;
      } catch (error) {
        last = error;
        remove(service);
        if (!portFree(port)) skip.add(port);
      }
    }
    throw last;
  });
};

const remove = (service: "mysql" | "postgres") => {
  try {
    compose("rm", "-sf", "-v", service);
  } catch {
    /* already gone */
  }
  try {
    execFileSync("docker", ["rm", "-f", `datasources-${service}-1`], { stdio: "inherit" });
  } catch {
    /* already gone */
  }
};

const grantMysql = () =>
  compose(
    "exec",
    "-T",
    "mysql",
    "mysql",
    "-uroot",
    "-proot",
    "-e",
    "GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;",
  );

export const ensureCompose = (service: "mysql" | "postgres") => {
  if (process.env.SKIP_DOCKER) return;
  up(service);
  if (service === "mysql") grantMysql();
};

export const freshCompose = (service: "mysql" | "postgres") => {
  if (process.env.SKIP_DOCKER) return;
  remove(service);
  up(service);
  if (service === "mysql") grantMysql();
};

export const stopCompose = (service: "mysql" | "postgres") => {
  if (process.env.SKIP_DOCKER) return;
  remove(service);
};
