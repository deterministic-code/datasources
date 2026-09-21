export const ParameterDirection = {
  Input: "input",
  Output: "output",
  InputOutput: "inputOutput",
  ReturnValue: "returnValue",
} as const;

export type ParameterDirection =
  (typeof ParameterDirection)[keyof typeof ParameterDirection];
