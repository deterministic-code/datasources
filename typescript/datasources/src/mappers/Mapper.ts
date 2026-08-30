export class Mapper {
  constructor(
    readonly name: string,
    readonly source: string,
    readonly destination: string,
  ) {}

  to = (name: string) => name;
  from = (name: string) => name;
}
