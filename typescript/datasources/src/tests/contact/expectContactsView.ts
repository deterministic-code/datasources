import { expect } from "vitest";

type View = { id: number; addresses: unknown[]; phones: unknown[] };

export const expectContactsView = (contacts: View[], count: number) => {
  const of = (id: number) => contacts.find((contact) => contact.id === id);
  expect(contacts).toHaveLength(count);
  expect(of(10)?.addresses).toHaveLength(2);
  expect(of(10)?.phones).toHaveLength(1);
  expect(of(4)?.addresses).toEqual([]);
  expect(of(4)?.phones).toHaveLength(1);
  expect(of(6)?.addresses).toHaveLength(2);
  expect(of(6)?.phones).toEqual([]);
  expect(of(12)?.addresses).toEqual([]);
  expect(of(12)?.phones).toEqual([]);
};
