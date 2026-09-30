import { describe, expect, it } from "vitest";
import { ChangeCasingMapper } from "./ChangeCasingMapper.ts";

describe("ChangeCasingMapper", () => {
  it("maps pascal to snake and back", () => {
    const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
    expect(mapper.map("UserType")).toBe("user_type");
    expect(mapper.map("Id")).toBe("id");
    expect(mapper.reverseMap("user_type")).toBe("UserType");
    expect(mapper.reverseMap("id")).toBe("Id");
  });

  it("maps contact names", () => {
    const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
    const contact = mapper.map("Contact");
    const firstName = mapper.map("FirstName");
    const emailAddress = mapper.map("EmailAddress");
    expect(contact).toBe("contact");
    expect(firstName).toBe("first_name");
    expect(emailAddress).toBe("email_address");
  });

  it("reverse maps contact names", () => {
    const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
    const contact = mapper.reverseMap("contact");
    const firstName = mapper.reverseMap("first_name");
    const emailAddress = mapper.reverseMap("email_address");
    expect(contact).toBe("Contact");
    expect(firstName).toBe("FirstName");
    expect(emailAddress).toBe("EmailAddress");
  });
});
