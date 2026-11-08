import { describe, expect, it } from "vitest";
import { clock } from "./clock.ts";

describe("clock", () => {
  it("defaults cover now", () => {
    expect(clock.now()).toBeInstanceOf(Date);
    expect(clock.uuid().length).toBeGreaterThan(0);
    const now = new Date("2020-01-02T03:04:05.000Z");
    clock.now = () => now;
    clock.uuid = () => "00000000-0000-0000-0000-000000000000";
    expect(clock.uuid()).toBe("00000000-0000-0000-0000-000000000000");
    expect(clock.now()).toEqual(now);
  });
});
