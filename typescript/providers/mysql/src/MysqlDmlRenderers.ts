import {
  BulkDeleteStatementTextRenderer,
  BulkInsertStatementTextRenderer,
  DeleteStatementTextRenderer,
  ExpressionRenderer,
  InsertStatement,
  InsertStatementTextRenderer,
  SelectStatementTextRenderer,
  UpdateStatementTextRenderer,
} from "@deterministic-code/datasources";
import { isKeyword, isVerbatim } from "./applyKeywords.ts";

const quote = (name: string): string =>
  isKeyword(name) || isVerbatim(name) ? `\`${name}\`` : name;
const parameter = (name: string): string => `:${name}`;

export class MysqlInsertStatementTextRenderer extends InsertStatementTextRenderer {
  override render(statement: InsertStatement): string {
    return `INSERT INTO ${quote(statement.table)} (${statement.fields.map((field) => quote(field.name)).join(", ")}) VALUES (${statement.fields.map((field) => parameter(field.name)).join(", ")})`;
  }
}

export class MysqlUpdateStatementTextRenderer extends UpdateStatementTextRenderer {
  protected override quote(name: string): string {
    return quote(name);
  }
  protected override parameter(name: string): string {
    return parameter(name);
  }
}

export class MysqlDeleteStatementTextRenderer extends DeleteStatementTextRenderer {
  protected override quote(name: string): string {
    return quote(name);
  }
}

export class MysqlSelectStatementTextRenderer extends SelectStatementTextRenderer {
  protected override quote(name: string): string {
    return quote(name);
  }
}

export class MysqlBulkInsertStatementTextRenderer extends BulkInsertStatementTextRenderer {
  protected override quote(name: string): string {
    return quote(name);
  }
  protected override parameter(name: string): string {
    return parameter(name);
  }
}

export class MysqlBulkDeleteStatementTextRenderer extends BulkDeleteStatementTextRenderer {
  protected override quote(name: string): string {
    return quote(name);
  }
  protected override parameter(name: string): string {
    return parameter(name);
  }
}

export class MysqlExpressionRenderer extends ExpressionRenderer {
  protected override quote(name: string): string {
    return quote(name);
  }
  protected override parameter(name: string): string {
    return parameter(name);
  }
}
