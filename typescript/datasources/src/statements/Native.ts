export class Native {
  constructor(readonly text: string) {}
}

export const n = (text: string): Native => new Native(text);
