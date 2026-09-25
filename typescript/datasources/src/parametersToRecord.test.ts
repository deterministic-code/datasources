import { describe, expect, it } from "vitest";
import { parametersToRecord } from "./index.ts";

describe("parametersToRecord", () => {
  it("returns an empty record when parameters are omitted", () => {
    expect(parametersToRecord()).toEqual({});
    expect(parametersToRecord(null)).toEqual({});
    expect(parametersToRecord(undefined)).toEqual({});
  });

  it("maps names to values", () => {
    expect(
      parametersToRecord([
        { name: "id", value: 1 },
        { name: "label", value: "a" },
      ]),
    ).toEqual({ id: 1, label: "a" });
  });
});
