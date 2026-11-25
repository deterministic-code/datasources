import {
  CheckConstraint,
  ForeignKey,
  PrimaryKey,
  UniqueConstraint,
  type ITableConstraint,
} from "@deterministic-code/datasources";
import { describe, expect, it } from "vitest";
import { fitMysqlConstraint, fitMysqlIdentifier } from "./MysqlIdentifier.ts";

describe("fitMysqlIdentifier", () => {
  it("leaves a name that already fits", () => {
    const name = "a".repeat(64);
    expect(fitMysqlIdentifier(name)).toBe(name);
    expect(fitMysqlIdentifier("users_email_index")).toBe("users_email_index");
  });

  it("truncates a long name and keeps a hash of the full name", () => {
    expect(fitMysqlIdentifier("a".repeat(65))).toBe(`${"a".repeat(57)}_2dd603`);
    expect(fitMysqlIdentifier("game_journey_settings_game_journey_setting_category_type_id_index")).toBe(
      "game_journey_settings_game_journey_setting_category_d88931_index",
    );
    expect(fitMysqlIdentifier("trg_game_journey_setting_category_preference_option_values_updated_at")).toBe(
      "trg_game_journey_setting_category_preference_o_8e15ab_updated_at",
    );
    expect(fitMysqlIdentifier("notification-types-channel-name-and-something-very-long-foreign-key")).toBe(
      "notification-types-channel-name-and-something-61b8c2-foreign-key",
    );
    expect(fitMysqlIdentifier(`${"a".repeat(56)}_${"b".repeat(9)}`)).toBe(`${"a".repeat(56)}_a955d9`);
    expect(fitMysqlIdentifier("GameJourneySettingsGameJourneySettingCategoryTypeIdentifierForeignKey")).toBe(
      "GameJourneySettingsGameJourneySettingCategoryTy_aa3be2ForeignKey",
    );
    expect(fitMysqlIdentifier("GAME_JOURNEY_SETTINGS_GAME_JOURNEY_SETTING_CATEGORY_TYPE_ID_INDEX")).toBe(
      "GAME_JOURNEY_SETTINGS_GAME_JOURNEY_SETTING_CATEGORY_3bfc58_INDEX",
    );
  });

  it("gives two different long names different identifiers", () => {
    const left = fitMysqlIdentifier(`${"a".repeat(70)}_index`);
    const right = fitMysqlIdentifier(`${"a".repeat(69)}b_index`);
    expect(left).not.toBe(right);
    expect(left.length).toBeLessThanOrEqual(64);
    expect(right.endsWith("_index")).toBe(true);
  });

  it("fits each constraint kind and rejects an unknown kind", () => {
    const long = "n".repeat(70);
    const primary = fitMysqlConstraint(new PrimaryKey(long, ["id"])) as PrimaryKey;
    const foreign = fitMysqlConstraint(
      new ForeignKey(long, ["user_id"], "users", ["id"], "cascade", "restrict"),
    ) as ForeignKey;
    const unique = fitMysqlConstraint(new UniqueConstraint(long, ["email"])) as UniqueConstraint;
    const check = fitMysqlConstraint(new CheckConstraint(long, "1 = 1")) as CheckConstraint;
    expect(primary.name).toBe(fitMysqlIdentifier(long));
    expect(primary.columns).toEqual(["id"]);
    expect(foreign.name).toBe(fitMysqlIdentifier(long));
    expect(foreign.onDelete).toBe("cascade");
    expect(foreign.onUpdate).toBe("restrict");
    expect(foreign.referencesTable).toBe("users");
    expect(unique.name).toBe(fitMysqlIdentifier(long));
    expect(unique.columns).toEqual(["email"]);
    expect(check.name).toBe(fitMysqlIdentifier(long));
    expect(check.expression).toBe("1 = 1");
    expect(() => fitMysqlConstraint({ kind: "missing", name: long } as unknown as ITableConstraint)).toThrow(
      /empty constraint/,
    );
  });
});
