export class EnrichmentSelection {
  private constructor(private readonly names: Set<string>) {}

  static readonly none = new EnrichmentSelection(new Set());

  static only = (...names: string[]): EnrichmentSelection => new EnrichmentSelection(new Set(names));

  includes = (destination: string): boolean => this.names.has(destination);
}
