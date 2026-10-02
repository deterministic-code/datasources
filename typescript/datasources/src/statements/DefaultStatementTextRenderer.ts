import { StatementTextRenderer } from "./StatementTextRenderer.ts";

export class DefaultStatementTextRenderer extends StatementTextRenderer {
  static readonly instance = new DefaultStatementTextRenderer();

  constructor() {
    super();
  }
}
