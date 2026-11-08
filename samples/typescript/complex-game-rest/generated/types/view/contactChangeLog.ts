// schema-version: 1.0
import type { ContactChangeLog as ContactChangeLogBase } from "../datasource/contactChangeLog";
/** View ContactChangeLog. */
export const contactChangeLog = (init: Partial<ContactChangeLog>): ContactChangeLog => ({ ...init } as ContactChangeLog);
export interface ContactChangeLog extends ContactChangeLogBase {}
