import { describe, expect, it } from "vitest";
import type { Cells } from "../converters/Cell.ts";
import type { TypeMap } from "./TypeMap.ts";
import { FlatToTypeMapper } from "./FlatToTypeMapper.ts";

type Address = { id: number; line1: string; city: string; address_type_name: string };
type Phone = { id: number; number: string; phone_type_name: string };
type Contact = {
  id: number;
  first_name: string;
  last_name: string;
  email: string | null;
  contact_source_name: string;
  contact_type_name: string;
  addresses: Address[];
  phones: Phone[];
};

const contactsView: TypeMap<Contact> = {
  key: "id",
  fields: {
    id: "id",
    first_name: "first_name",
    last_name: "last_name",
    email: "email",
    contact_source_name: "contact_source_name",
    contact_type_name: "contact_type_name",
    addresses: {
      key: "address_id",
      fields: {
        id: "address_id",
        line1: "address_line1",
        city: "address_city",
        address_type_name: "address_type_name",
      },
    },
    phones: {
      key: "phone_id",
      fields: {
        id: "phone_id",
        number: "phone_number",
        phone_type_name: "phone_type_name",
      },
    },
  },
};

const adaHome: Cells = {
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  address_id: 101,
  address_line1: "10 Downing St",
  address_city: "London",
  address_type_name: "Home",
  phone_id: 201,
  phone_number: "+44",
  phone_type_name: "Mobile",
};

const adaWork: Cells = {
  ...adaHome,
  address_id: 102,
  address_line1: "1 Parliament Sq",
  address_city: "London",
};

const noAddress: Cells = {
  id: 8,
  first_name: "Charles",
  last_name: "Babbage",
  email: null,
  contact_source_name: "Import",
  contact_type_name: "Person",
  address_id: null,
  address_line1: null,
  address_city: null,
  address_type_name: null,
  phone_id: 202,
  phone_number: "555",
  phone_type_name: "Work",
};

describe("FlatToTypeMapper", () => {
  const mapper = new FlatToTypeMapper<Contact>(contactsView);

  it("maps one denormalized row into a contact with 0-1 children", () => {
    expect(mapper.map(adaHome)).toEqual({
      id: 7,
      first_name: "Ada",
      last_name: "Lovelace",
      email: "ada@ex.com",
      contact_source_name: "Web",
      contact_type_name: "Person",
      addresses: [{ id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" }],
      phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }],
    });
    expect(mapper.map(noAddress).addresses).toEqual([]);
    expect(mapper.map(noAddress).phones).toEqual([{ id: 202, number: "555", phone_type_name: "Work" }]);
  });

  it("reverseMaps the first child of each group", () => {
    expect(mapper.reverseMap(mapper.map(adaHome))).toEqual(adaHome);
    expect(mapper.reverseMap(mapper.map(noAddress))).toEqual(noAddress);
  });

  it("collapses duplicate join rows into one contact with unique children", () => {
    const [ada, charles] = mapper.collapse([adaHome, adaWork, adaHome, noAddress]);
    expect(ada).toEqual({
      id: 7,
      first_name: "Ada",
      last_name: "Lovelace",
      email: "ada@ex.com",
      contact_source_name: "Web",
      contact_type_name: "Person",
      addresses: [
        { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
        { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" },
      ],
      phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }],
    });
    expect(charles?.id).toBe(8);
    expect(charles?.addresses).toEqual([]);
    expect(charles?.phones).toHaveLength(1);
  });

  it("collapses already-mapped rows", () => {
    const [ada] = mapper.collapse([mapper.map(adaHome), mapper.map(adaWork)]);
    expect(ada?.addresses).toHaveLength(2);
    expect(ada?.phones).toHaveLength(1);
  });

  it("skips rows without a parent key and children without a group key", () => {
    const orphan = { ...adaHome, id: null };
    const mapped = mapper.map(adaHome);
    mapped.addresses = [
      { id: 0, line1: "x", city: "y", address_type_name: "" },
      { id: 101, line1: "dup", city: "London", address_type_name: "Home" },
    ];
    mapped.addresses[0] = {
      id: undefined as unknown as number,
      line1: "x",
      city: "y",
      address_type_name: "",
    };
    const extra = mapper.map(adaWork);
    extra.addresses = [
      {
        id: undefined as unknown as number,
        line1: "ghost",
        city: "None",
        address_type_name: "",
      },
    ];
    const [ada] = mapper.collapse([orphan, mapped, extra]);
    expect(ada?.addresses.map((address) => address.line1)).toEqual(["x", "dup"]);
  });

  it("falls back to the spec key when it is not a mapped dest field", () => {
    const names = new FlatToTypeMapper<{ id: number; name: string }>({
      key: "id",
      fields: { name: "name" },
    });
    expect(names.collapse([{ id: 1, name: "ada" }])).toEqual([{ id: 1, name: "ada" }]);
  });

  it("omits nested group fields when compiling a collection", () => {
    type Row = { id: number; items: { id: number; title: string; kids: { n: number }[] }[] };
    const nested = new FlatToTypeMapper<Row>({
      key: "id",
      fields: {
        id: "id",
        items: {
          key: "item_id",
          fields: {
            id: "item_id",
            title: "item_title",
            kids: { key: "kid_id", fields: { n: "kid_n" } },
          },
        },
      },
    });
    expect(nested.map({ id: 1, item_id: 2 })).toEqual({
      id: 1,
      items: [{ id: 2, title: null }],
    });
    expect(nested.map({}).id).toBeNull();
    expect(nested.reverseMap({} as Row)).toEqual({
      id: null,
      item_id: null,
      item_title: null,
    });
  });
});
