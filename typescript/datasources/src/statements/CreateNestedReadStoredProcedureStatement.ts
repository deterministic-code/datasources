import type { NestedSelectStatement } from "./NestedSelectStatement.ts";
import { Operator } from "./Operator.ts";
import { Identifier } from "./Identifier.ts";
import type { ICreateNestedReadStoredProcedureStatement } from "./ICreateNestedReadStoredProcedureStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { NestedWhereArgument } from "./NestedWhereArgument.ts";
import { SelectStatement } from "./SelectStatement.ts";
import { Where } from "./Where.ts";

export class CreateNestedReadStoredProcedureStatement implements ICreateNestedReadStoredProcedureStatement {
  readonly name: string;
  readonly nested: NestedSelectStatement;
  readonly arguments: readonly NestedWhereArgument[];

  constructor(
    name: string,
    nested: NestedSelectStatement,
    args: readonly NestedWhereArgument[],
  ) {
    this.name = name;
    this.nested = nested;
    this.arguments = args;
  }

  get select(): SelectStatement {
    return CreateNestedReadStoredProcedureStatement.withArguments(this.nested.select, this.arguments);
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  static withArguments(select: SelectStatement, arguments_: readonly NestedWhereArgument[]): SelectStatement {
    const where = arguments_.reduce(
      (clause, argument) => {
        const equals = new Operator.Equals(new Identifier(argument.path), new Identifier(argument.parameter.name));
        return clause instanceof Where ? new Where(new Operator.And(clause.condition, equals)) : new Where(equals);
      },
      select.where,
    );
    return new SelectStatement({
      distinct: select.distinct,
      table: select.table,
      fields: select.fields,
      aliases: select.aliases,
      joins: select.joins,
      where,
      groupBy: select.groupBy,
      having: select.having,
      orderBy: select.orderBy,
      limit: select.limit,
      offset: select.offset,
    });
  }
}
