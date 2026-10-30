import type { ICreateTriggerStatement } from "./ICreateTriggerStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { StatementNaming } from "./StatementNaming.ts";

export class CreateTriggerStatement implements ICreateTriggerStatement {
  constructor(
    readonly table: string,
    readonly name: string,
    readonly updatedColumn: string,
    readonly keyColumn: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  static nameOf = (entity: string, naming: StatementNaming): string => {
    if (naming.isVerbatim(entity)) return `trg_${entity}_updated_at`;
    const stem = naming.physicalStem(entity);
    return naming.letter(naming.convert(`trg_${stem}_updated_at`, naming.triggerNameCasing));
  };
}
