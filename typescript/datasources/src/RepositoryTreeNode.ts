import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Row } from "./Row.ts";

type Props = Record<string, Scalar | object>;

export class RepositoryTreeNode implements IRepositoryTreeNode {
  constructor(
    readonly value: object,
    readonly propertyName: string,
    readonly parent?: IRepositoryTreeNode,
    readonly enrichment?: Enrichment,
  ) {}
}

export const sourceKey = (
  parent: object,
  name: string,
  sources: (Row | undefined)[] | undefined,
  index: number,
): Scalar | object | undefined => {
  const cell = sources?.[index]?.[name];
  if (cell !== undefined && cell !== null) return cell;
  return props(parent)[name];
};

export const props = (item: object): Props => item as Props;

export const keyOf = (key: Scalar | object): string => String(key);

export const scalar = (value: Scalar | object): Scalar =>
  value === null ||
  typeof value === "string" ||
  typeof value === "number" ||
  typeof value === "boolean" ||
  typeof value === "bigint"
    ? value
    : (() => {
        throw new DatasourceError("Enrichment key must be a scalar");
      })();
