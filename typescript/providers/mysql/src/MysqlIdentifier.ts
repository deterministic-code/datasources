import {
  CheckConstraint,
  ForeignKey,
  PrimaryKey,
  UniqueConstraint,
  type ITableConstraint,
} from "@deterministic-code/datasources";

/** MySQL identifiers are at most 64 characters. */
const LIMIT = 64;

const ROLES = [
  "default_constraint",
  "unique_constraint",
  "foreign_key",
  "primary_key",
  "updated_at",
  "index",
] as const;

const pascal = (role: string): string =>
  role
    .split("_")
    .map((word) => word[0]!.toUpperCase() + word.slice(1))
    .join("");

const preservedSuffix = (name: string): string => {
  const lower = name.toLowerCase();
  for (const role of ROLES) {
    const snake = `_${role}`;
    const kebab = `-${role.replace(/_/g, "-")}`;
    if (lower.endsWith(snake)) return name.slice(-snake.length);
    if (lower.endsWith(kebab)) return name.slice(-kebab.length);
  }
  for (const role of ROLES) {
    const shaped = pascal(role);
    if (name.endsWith(shaped)) return name.slice(-shaped.length);
  }
  return "";
};

/** First 6 hex digits of FNV-1a 32-bit. The same function in C# and Rust. */
const hash = (name: string): string => {
  let value = 0x811c9dc5;
  for (const byte of new TextEncoder().encode(name)) {
    value ^= byte;
    value = Math.imul(value, 0x01000193) >>> 0;
  }
  return (value >>> 0).toString(16).padStart(8, "0").slice(0, 6);
};

/**
 * Leave a name that fits. A longer one keeps its role suffix and a hash of the
 * full name, so create and drop statements spell it the same way.
 */
export const fitMysqlIdentifier = (name: string): string => {
  if (name.length <= LIMIT) return name;
  const suffix = preservedSuffix(name);
  const tail = `${suffix.startsWith("-") ? "-" : "_"}${hash(name)}${suffix}`;
  return `${name.slice(0, LIMIT - tail.length).replace(/[_-]+$/, "")}${tail}`;
};

const fitted: Record<ITableConstraint["kind"], (constraint: ITableConstraint) => ITableConstraint> = {
  primaryKey: (constraint) => {
    const key = constraint as PrimaryKey;
    return new PrimaryKey(fitMysqlIdentifier(key.name), key.columns);
  },
  foreignKey: (constraint) => {
    const key = constraint as ForeignKey;
    return new ForeignKey(
      fitMysqlIdentifier(key.name),
      key.columns,
      key.referencesTable,
      key.referencesColumns,
      key.onDelete,
      key.onUpdate,
    );
  },
  unique: (constraint) => {
    const key = constraint as UniqueConstraint;
    return new UniqueConstraint(fitMysqlIdentifier(key.name), key.columns);
  },
  check: (constraint) => {
    const key = constraint as CheckConstraint;
    return new CheckConstraint(fitMysqlIdentifier(key.name), key.expression);
  },
};

export const fitMysqlConstraint = (constraint: ITableConstraint): ITableConstraint => {
  const handler = fitted[constraint.kind];
  if (handler === undefined) throw new Error("empty constraint");
  return handler(constraint);
};
