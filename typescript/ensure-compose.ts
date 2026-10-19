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

const compose = (...args: string[]) =>
  execFileSync("docker", ["compose", "-p", "datasources", "-f", "docker/compose.yml", ...args], {
    cwd: root,
    stdio: "inherit",
  });

const up = (service: "mysql" | "postgres") => {
  withLock(service, () => {
    let last: unknown;
    for (let attempt = 0; attempt < 3; attempt++) {
      try {
        compose("up", "-d", "--wait", service);
        return;
      } catch (error) {
        last = error;
        remove(service);
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
