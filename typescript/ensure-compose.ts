import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(fileURLToPath(new URL(".", import.meta.url)), "..");

const compose = (...args: string[]) =>
  execFileSync("docker", ["compose", ...args], { cwd: root, stdio: "inherit" });

const up = (service: "mysql" | "postgres") => {
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
