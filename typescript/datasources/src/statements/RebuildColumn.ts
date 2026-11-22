export class RebuildColumn {
  constructor(
    readonly name: string,
    readonly source?: string,
    readonly cast?: string,
    readonly literal?: string,
  ) {}
}
