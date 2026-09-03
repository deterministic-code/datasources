export const logMapped = (value: object): void => {
  console.log(
    `[${new Date().toISOString()}]-[datasource][Mapped]-[${JSON.stringify(value)}]`,
  );
};
