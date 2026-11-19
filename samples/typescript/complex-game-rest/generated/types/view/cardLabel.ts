// schema-version: 1.0
/** View CardLabel. */
export const cardLabel = (init: Partial<CardLabel>): CardLabel => Object.assign(new CardLabel(), init);
export class CardLabel {
  key: string = "";
  value: string = "";
}
