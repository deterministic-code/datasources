# Examples

Samples from the code reference. Open a link for the explanation and the C#, Rust, and TypeScript code.

## The Mapping and Converting Pipelines

- [Example](./tutorials/mapping-converting-pipelines.md#example)

## Datasource

- [Create default from .env settings](./sections/datasource.md#create-default-from-env-settings)
- [Default using environment variables](./sections/datasource.md#default-using-environment-variables)
- [Create from a driver](./sections/datasource.md#create-from-a-driver)

## Connection

- [Create from Default](./sections/connection.md#create-from-default)
- [Create and run a command](./sections/connection.md#create-and-run-a-command)
- [Close with using](./sections/connection.md#close-with-using)
- [Start a transaction](./sections/connection.md#start-a-transaction)

## Transaction

- [Commit or abort](./sections/transaction.md#commit-or-abort)
- [Command running in one atomic transaction](./sections/transaction.md#command-running-in-one-atomic-transaction)

## Command

- [Run executeReader on a simple list](./sections/command.md#run-executereader-on-a-simple-list)
- [Run ExecuteReader and return NameValues](./sections/command.md#run-executereader-and-return-namevalues)
- [Execute, Insert, Read, Update, and Delete](./sections/command.md#execute-insert-read-update-and-delete)
- [Map flat contacts](./sections/command.md#map-flat-contacts)
- [Fill an output parameter](./sections/command.md#fill-an-output-parameter)
- [Read a return value](./sections/command.md#read-a-return-value)

## Reader

- [Read a row](./sections/reader.md#read-a-row)
- [Read and apply case mappings](./sections/reader.md#read-and-apply-case-mappings)
- [ExecuteReader, then apply flat to nested mapper.](./sections/reader.md#executereader-then-apply-flat-to-nested-mapper)
- [ExecuteReader maps flat contacts](./sections/reader.md#executereader-maps-flat-contacts)
- [Read multiple results returned in a single command](./sections/reader.md#read-multiple-results-returned-in-a-single-command)
- [Call a stored procedure that returns two results](./sections/reader.md#call-a-stored-procedure-that-returns-two-results)

## Repositories

- [Simple readonly](./sections/repository.md#simple-readonly)
- [Create, Read, Update, and Delete](./sections/repository.md#create-read-update-and-delete)
- [Enriched Create, Read, Update, and Delete](./sections/repository.md#enriched-create-read-update-and-delete)
- [Bulk](./sections/repository.md#bulk)

## Readonly Repository

- [Simple readonly](./sections/readonly-repository.md#simple-readonly)
- [Map an object explicitly](./sections/readonly-repository.md#map-an-object-explicitly)
- [Enriched readonly](./sections/readonly-repository.md#enriched-readonly)
- [Read a user by guid, username, and created](./sections/readonly-repository.md#read-a-user-by-guid-username-and-created)
- [Call a stored procedure](./sections/readonly-repository.md#call-a-stored-procedure)
- [Read and read single](./sections/readonly-repository.md#read-and-read-single)
- [Fill a lookup](./sections/readonly-repository.md#fill-a-lookup)
- [Fill roles from one join](./sections/readonly-repository.md#fill-roles-from-one-join)
- [Read with where clause and custom flat to nested custom object mapping options](./sections/readonly-repository.md#read-with-where-clause-and-custom-flat-to-nested-custom-object-mapping-options)

## CRUD Repository

- [Simple CRUD](./sections/crud-repository.md#simple-crud)
- [Enriched CRUD](./sections/crud-repository.md#enriched-crud)
- [Multi-tenant](./sections/crud-repository.md#multi-tenant)
- [Enriched contacts write](./sections/crud-repository.md#enriched-contacts-write)
- [Type mapping](./sections/crud-repository.md#type-mapping)

## Bulk Repository

- [Create parents and children](./sections/bulk-repository.md#create-parents-and-children)

## Repository Factories

- [Register and create](./sections/repository-factories.md#register-and-create)

## Mappers

- [Transform a contact](./sections/mappers.md#transform-a-contact)
- [Change casing](./sections/mappers.md#change-casing)
- [Lowercase plural](./sections/mappers.md#lowercase-plural)
- [Flat to nested](./sections/mappers.md#flat-to-nested)

## Identity Mapper

- [Map and reverse map](./sections/identity-mapper.md#map-and-reverse-map)

## Dictionary Mapper

- [Map and reverse map](./sections/dictionary-mapper.md#map-and-reverse-map)
- [First key wins on reverse](./sections/dictionary-mapper.md#first-key-wins-on-reverse)

## Pluralizer Mapper

- [Map contacts](./sections/pluralizer-mapper.md#map-contacts)
- [Reverse map contacts](./sections/pluralizer-mapper.md#reverse-map-contacts)
- [Lowercase plural](./sections/pluralizer-mapper.md#lowercase-plural)

## Change Casing Mapper

- [Map contacts](./sections/change-casing-mapper.md#map-contacts)
- [Reverse map contacts](./sections/change-casing-mapper.md#reverse-map-contacts)

## Transform Mapper

- [Flatten with TransformMapper](./sections/transform-mapper.md#flatten-with-transformmapper)
- [Prefix a contact field](./sections/transform-mapper.md#prefix-a-contact-field)
- [Map a contact name and email](./sections/transform-mapper.md#map-a-contact-name-and-email)

## Flat To Nested Mapper

- [Map join rows to a nested contact](./sections/flat-to-nested-mapper.md#map-join-rows-to-a-nested-contact)

## Metadata Row Type Converter

- [Convert a row](./sections/metadata-row-type-converter.md#convert-a-row)
- [Field converter](./sections/metadata-row-type-converter.md#field-converter)

## Select Statement

- [Wrap query text and an optional where](./sections/select-statement.md#wrap-query-text-and-an-optional-where)
- [Compile a SELECT](./sections/select-statement.md#compile-a-select)
- [Join tables](./sections/select-statement.md#join-tables)
- [Group, aggregate, order, and limit](./sections/select-statement.md#group-aggregate-order-and-limit)
- [Group rows](./sections/select-statement.md#group-rows)
- [Aggregate without grouping](./sections/select-statement.md#aggregate-without-grouping)
- [Order rows](./sections/select-statement.md#order-rows)
- [Limit rows](./sections/select-statement.md#limit-rows)

## Insert Statement

- [Compile an INSERT](./sections/insert-statement.md#compile-an-insert)

## Update Statement

- [Compile an UPDATE](./sections/update-statement.md#compile-an-update)

## Delete Statement

- [Compile a DELETE](./sections/delete-statement.md#compile-a-delete)

## Create Table Statement

- [Compile a CREATE TABLE](./sections/create-table-statement.md#compile-a-create-table)
- [Compile named constraints](./sections/create-table-statement.md#compile-named-constraints)

## Create Index Statement

- [Compile a CREATE INDEX](./sections/create-index-statement.md#compile-a-create-index)
- [Compile a CREATE UNIQUE INDEX](./sections/create-index-statement.md#compile-a-create-unique-index)

## Execute Stored Procedure Statement

- [Compile a CALL](./sections/execute-stored-procedure-statement.md#compile-a-call)

## Return Statement

- [Compile a SELECT function call](./sections/return-statement.md#compile-a-select-function-call)

## Where

- [Compare, groups, not, and in](./sections/where-clause.md#compare-groups-not-and-in)
- [Column, alias, literal](./sections/where-clause.md#column-alias-literal)
- [Attach to a query](./sections/where-clause.md#attach-to-a-query)

## Expressions

- [Compare tree](./sections/expressions.md#compare-tree)
- [Column, alias, literal](./sections/expressions.md#column-alias-literal)

## Expression Evaluator

- [Match and miss](./sections/expression-evaluator.md#match-and-miss)

## Datasource Factory

- [Register and Create](./sections/datasource-factory.md#register-and-create)
- [Missing provider](./sections/datasource-factory.md#missing-provider)

## Datasource URL

- [Postgres URL](./sections/datasource-url.md#postgres-url)
- [SQLite file](./sections/datasource-url.md#sqlite-file)
- [Not a URL](./sections/datasource-url.md#not-a-url)

## IEnriched

- [Register a fill](./sections/i-enriched.md#register-a-fill)

## Enrichment

- [Fill a lookup](./sections/enrichment.md#fill-a-lookup)

## IReadonlyEnrichmentStrategy

- [Fill a lookup](./sections/i-readonly-enrichment-strategy.md#fill-a-lookup)

## DefaultReadonlyEnrichmentStrategy

- [Fill a lookup](./sections/default-readonly-enrichment-strategy.md#fill-a-lookup)

## HashsetOptimizedReadonlyEnrichmentStrategy

- [Fill with one query](./sections/hashset-optimized-readonly-enrichment-strategy.md#fill-with-one-query)

## NestedToObjectEnrichmentStrategy

- [Collapse join rows](./sections/nested-to-object-enrichment-strategy.md#collapse-join-rows)

## IWriteEnrichmentStrategy

- [Roll back a bulk update](./sections/i-write-enrichment-strategy.md#roll-back-a-bulk-update)

## DefaultWriteEnrichmentStrategy

- [Roll back a bulk update](./sections/default-write-enrichment-strategy.md#roll-back-a-bulk-update)

## LazyWriteEnrichmentStrategy

- [Skip unloaded lists](./sections/lazy-write-enrichment-strategy.md#skip-unloaded-lists)

## UpdateEnrichmentStrategy

- [Replace children](./sections/update-enrichment-strategy.md#replace-children)

## DefaultUpdateEnrichmentStrategy

- [Replace-sync children](./sections/default-update-enrichment-strategy.md#replace-sync-children)

## CascadeDeleteUpdateEnrichmentStrategy

- [Recreate children](./sections/cascade-delete-update-enrichment-strategy.md#recreate-children)

## DeleteEnrichmentStrategy

- [Skip child deletes](./sections/delete-enrichment-strategy.md#skip-child-deletes)

## CascadeDeleteEnrichmentStrategy

- [Cascade delete](./sections/cascade-delete-enrichment-strategy.md#cascade-delete)

## IBulkWriteStrategy

- [One statement per row](./sections/i-bulk-write-strategy.md#one-statement-per-row)

## SingleCommandIBulkWriteStrategy

- [One statement per row](./sections/single-command-i-bulk-write-strategy.md#one-statement-per-row)

## BatchCommandIBulkWriteStrategy

- [Join statements](./sections/batch-command-i-bulk-write-strategy.md#join-statements)

## StoredProcedureBulkWriteStrategy

- [One JSON call](./sections/stored-procedure-bulk-write-strategy.md#one-json-call)

## Driver

- [Register and Create](./writing-providers.md#register-and-create)
- [Mock driver](./writing-providers.md#mock-driver)

## Schemas

### minimal-users

```sql
CREATE TABLE users (id int, username string NOT NULL, is_enabled bool NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id))
```

### pipeline-users

```sql
CREATE TABLE users (id int, uuid uuid NOT NULL, name string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id))
```

### user-account

```sql
CREATE TABLE user (id uuid, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id))
```

### user-type-lookup

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id uuid, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id))
```

### user-roles-read

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id uuid, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id));
CREATE TABLE roles (id int, name string NOT NULL, CONSTRAINT pk_roles PRIMARY KEY (id));
CREATE TABLE user_roles (user_id uuid NOT NULL, role_id int NOT NULL)
```

### user-view-version

```sql
CREATE TABLE user_type (id int, name string NOT NULL, CONSTRAINT pk_user_type PRIMARY KEY (id));
CREATE TABLE user (id int, username string NOT NULL, is_enabled bool NOT NULL, created datetime NOT NULL, user_type_id int NOT NULL, version int NOT NULL, CONSTRAINT pk_user PRIMARY KEY (id));
CREATE TABLE user_role (user_id int NOT NULL, name string NOT NULL)
```

### user-pascal

```sql
CREATE TABLE User (id int, is_enabled bool NOT NULL, username string NOT NULL, CONSTRAINT pk_User PRIMARY KEY (id))
```

### user-view-links

```sql
CREATE TABLE users (id int, username string NOT NULL, is_enabled bool NOT NULL, user_type_id int NOT NULL, CONSTRAINT pk_users PRIMARY KEY (id));
CREATE TABLE user_types (id int, name string NOT NULL, CONSTRAINT pk_user_types PRIMARY KEY (id));
CREATE TABLE roles (id int, name string NOT NULL, CONSTRAINT pk_roles PRIMARY KEY (id));
CREATE TABLE groups (id int, name string NOT NULL, CONSTRAINT pk_groups PRIMARY KEY (id));
CREATE TABLE user_roles (user_id int NOT NULL, role_id int NOT NULL, CONSTRAINT pk_user_roles PRIMARY KEY (user_id, role_id));
CREATE TABLE user_groups (user_id int NOT NULL, group_id int NOT NULL, CONSTRAINT pk_user_groups PRIMARY KEY (user_id, group_id))
```

### multi-tenant

```sql
CREATE TABLE account (tenant_id int NOT NULL, id int NOT NULL, name string NOT NULL, CONSTRAINT pk_account PRIMARY KEY (tenant_id, id))
```

### contacts

```sql
CREATE TABLE contact_source (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_contact_source PRIMARY KEY (id));
CREATE TABLE contact_type (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_contact_type PRIMARY KEY (id));
CREATE TABLE contacts_base (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_source_id int NOT NULL, contact_type_id int NOT NULL, first_name string NOT NULL, last_name string NOT NULL, email string, notes string, CONSTRAINT pk_contacts_base PRIMARY KEY (id), CONSTRAINT fk_contacts_source FOREIGN KEY (contact_source_id) REFERENCES contact_source (id), CONSTRAINT fk_contacts_type FOREIGN KEY (contact_type_id) REFERENCES contact_type (id));
CREATE TABLE address_type (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_address_type PRIMARY KEY (id));
CREATE TABLE phone_type (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_phone_type PRIMARY KEY (id));
CREATE TABLE address (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_id int NOT NULL, address_type_id int NOT NULL, line1 string NOT NULL, line2 string, city string NOT NULL, region string, postal_code string, country string, CONSTRAINT pk_address PRIMARY KEY (id), CONSTRAINT fk_address_contact FOREIGN KEY (contact_id) REFERENCES contacts_base (id), CONSTRAINT fk_address_type FOREIGN KEY (address_type_id) REFERENCES address_type (id));
CREATE TABLE phone (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_id int NOT NULL, phone_type_id int NOT NULL, number string NOT NULL, CONSTRAINT pk_phone PRIMARY KEY (id), CONSTRAINT fk_phone_contact FOREIGN KEY (contact_id) REFERENCES contacts_base (id), CONSTRAINT fk_phone_type FOREIGN KEY (phone_type_id) REFERENCES phone_type (id));
CREATE TABLE contact_groups_base (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, name string NOT NULL, description string, CONSTRAINT pk_contact_groups_base PRIMARY KEY (id));
CREATE TABLE contact_group_member (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, contact_id int NOT NULL, contact_group_id int NOT NULL, CONSTRAINT pk_contact_group_member PRIMARY KEY (id), CONSTRAINT fk_member_contact FOREIGN KEY (contact_id) REFERENCES contacts_base (id), CONSTRAINT fk_member_group FOREIGN KEY (contact_group_id) REFERENCES contact_groups_base (id), CONSTRAINT uq_contact_group_member UNIQUE (contact_id, contact_group_id));
CREATE TABLE legacy_contact (key string, first_name string NOT NULL, last_name string NOT NULL, email string, imported_at datetime, CONSTRAINT pk_legacy_contact PRIMARY KEY (key));
CREATE TABLE contact_change_log (id int, uuid uuid NOT NULL, created datetime NOT NULL, updated datetime NOT NULL, version binary NOT NULL, occurred_at datetime NOT NULL, contact_id int NOT NULL, changed_by string NOT NULL, change_summary string NOT NULL, CONSTRAINT pk_contact_change_log PRIMARY KEY (id))
```

### items

```sql
CREATE TABLE items (id int, name string, CONSTRAINT pk_items PRIMARY KEY (id))
```

### settings

```sql
CREATE TABLE settings (name string, value string)
```

### case-contacts

```sql
CREATE TABLE contacts (user_id int, first_name string, CONSTRAINT pk_contacts PRIMARY KEY (user_id))
```

### flat-users

```sql
CREATE TABLE users (id int, username string, contact_name string)
```

### flat-contact-users

```sql
CREATE TABLE users (id int, username string, email string, contact_name1 string, contact_name2 string, CONSTRAINT pk_users PRIMARY KEY (id))
```
