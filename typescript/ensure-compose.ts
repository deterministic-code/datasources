import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(fileURLToPath(new URL(".", import.meta.url)), "..");

const compose = (...args: string[]) =>
  execFileSync("docker", ["compose", ...args], { cwd: root, stdio: "inherit" });

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
  compose("up", "-d", "--wait", service);
  if (service === "mysql") grantMysql();
};
