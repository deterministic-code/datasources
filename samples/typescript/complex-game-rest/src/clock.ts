export const clock = {
  now: (): Date => new Date(),
  uuid: (): string => crypto.randomUUID(),
};
