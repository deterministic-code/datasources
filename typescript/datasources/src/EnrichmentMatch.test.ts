import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import type { Scalar } from "./builders/Scalar.ts";
import { DefaultCreateEnrichmentStrategy } from "./DefaultCreateEnrichmentStrategy.ts";
import { HashsetOptimizedReadonlyEnrichmentStrategy } from "./HashsetOptimizedReadonlyEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IConnection } from "./IConnection.ts";
import { ReadonlyRepository } from "./ReadonlyRepository.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { collectAsync } from "./collectAsync.ts";

class Mark {
  Id = 0;
  PlayId = 0;
  UserId = 0;
  Label = "";
}

class Seat {
  Id = 0;
  PlayId = 0;
  UserId = 0;
  Marks: Mark[] = [];
}

const columns = new ChangeCasingMapper("pascalCase", "snakeCase");

const repo = <T>(type: new () => T, connection: IConnection) =>
  new ReadonlyRepository(connection, { type, propertyMappings: columns });

const setup = async (rows: Record<string, Scalar>[]): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  for (const row of rows) {
    const response = await connection.createCommand().execute(
      "insert",
      Object.entries(row).map(([name, value]) => ({ name, value })),
    );
    if (typeof response.result === "number") row.id = response.result;
  }
  return connection;
};

const marks: Enrichment = {
  destinationProperty: "Marks",
  sourceProperty: "PlayId",
  relatedKey: "PlayId",
  type: "list",
  keys: [
    { sourceProperty: "PlayId", relatedKey: "PlayId" },
    { sourceProperty: "UserId", relatedKey: "UserId" },
  ],
};

describe("composite enrichment", () => {
  it("reads only the rows that match every column", async () => {
    const seats = await setup([
      { play_id: 1, user_id: 7 },
      { play_id: 1, user_id: 8 },
    ]);
    const events = await setup([
      { play_id: 1, user_id: 7, label: "mine" },
      { play_id: 1, user_id: 8, label: "theirs" },
      { play_id: 1, user_id: 7, label: "also" },
    ]);
    const seatsRepo = repo(Seat, seats);
    seatsRepo.addEnriched(marks, repo(Mark, events));
    const rows = await collectAsync(seatsRepo.read());
    const mine = rows.find((row) => row.UserId === 7);
    const theirs = rows.find((row) => row.UserId === 8);
    expect(mine?.Marks.map((mark) => mark.Label).sort()).toEqual(["also", "mine"]);
    expect(theirs?.Marks.map((mark) => mark.Label)).toEqual(["theirs"]);
  });

  it("hashset reads only the rows that match every column", async () => {
    const seats = await setup([
      { play_id: 1, user_id: 7 },
      { play_id: 1, user_id: 8 },
    ]);
    const events = await setup([
      { play_id: 1, user_id: 7, label: "mine" },
      { play_id: 1, user_id: 8, label: "theirs" },
    ]);
    const seatsRepo = repo(Seat, seats);
    seatsRepo.addEnriched(marks, repo(Mark, events), new HashsetOptimizedReadonlyEnrichmentStrategy());
    const rows = await collectAsync(seatsRepo.read());
    expect(rows.find((row) => row.UserId === 7)?.Marks.map((mark) => mark.Label)).toEqual(["mine"]);
    expect(rows.find((row) => row.UserId === 8)?.Marks.map((mark) => mark.Label)).toEqual(["theirs"]);
  });

  it("stamps every parent column onto the child", () => {
    const strategy = new DefaultCreateEnrichmentStrategy<Seat>();
    const parent = new Seat();
    parent.PlayId = 1;
    parent.UserId = 7;
    parent.Marks = [Object.assign(new Mark(), { Label: "mine" })];
    const [child] = strategy.collectChildren({} as never, marks, parent) as Mark[];
    expect(child?.PlayId).toBe(1);
    expect(child?.UserId).toBe(7);
    expect(child?.Label).toBe("mine");
  });
});
