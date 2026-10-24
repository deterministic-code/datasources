const key = "datasource.use_stored_procedures";

/** Flag on: set the key. Flag off: leave yaml as loaded. */
export const withStoredProcedures = (
  settings: Record<string, string>,
  storedProcedures: boolean,
): Record<string, string> =>
  storedProcedures ? { ...settings, [key]: "true" } : settings;
