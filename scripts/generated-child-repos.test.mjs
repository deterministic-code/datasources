import assert from "node:assert/strict";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { childRepoViolations, MESSAGE, repoFiles, scanPaths } from "./generated-child-repos.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

test("child fields typed as CRUDRepository are violations", () => {
  assert.equal(childRepoViolations('    pub address: Arc<CRUDRepository<Address>>,').length, 1);
  assert.equal(childRepoViolations("  readonly address: CRUDRepository<Address>;").length, 1);
  assert.equal(childRepoViolations("    public ICRUDRepository<Address> Address { get; }").length, 1);
  assert.equal(childRepoViolations("    pub {{field}}: Arc<CRUDRepository<{{typeName}}>>,").length, 1);
});

test("inner Deref and class inheritance are allowed", () => {
  assert.equal(childRepoViolations("    pub(crate) inner: Arc<CRUDRepository<Contact>>,").length, 0);
  assert.equal(childRepoViolations("    type Target = CRUDRepository<Contact>;").length, 0);
  assert.equal(childRepoViolations("public sealed class ContactsRepository : CRUDRepository<Contact>").length, 0);
  assert.equal(childRepoViolations("export class ContactsRepository extends CRUDRepository<Contact> {").length, 0);
  assert.equal(childRepoViolations("    pub address: AddressesRepository,").length, 0);
});

test("templates and sample generated repositories use generated child types", () => {
  const hits = scanPaths(repoFiles(root));
  assert.deepEqual(hits, [], MESSAGE);
});
