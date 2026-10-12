<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Examples

Find the closest title. Open that language's snippet. Topic files already include the expanded samples.

## The Mapping and Converting Pipelines

- Example
  - `docs/examples/example/ReadPipelineIntegrationTests.User.csharp`
  - `docs/examples/example/ReadPipelineIntegrationTests.MapsAndConvertsUsersThroughTheReadPipeline.csharp`
  - `docs/examples/example/read_pipeline.ConvertSchemaForUser.rust`
  - `docs/examples/example/read_pipeline.maps_and_converts_users_through_the_read_pipeline.rust`
  - `docs/examples/example/read-pipeline.integration.test.User.typescript`
  - `docs/examples/example/read-pipeline.integration.test.maps-and-converts-users-through-the-read-pipeline.typescript`

## Datasource

- Create default from .env settings
  - `docs/examples/create-default-from-env-settings/DatasourceTests.CreateDefaultFromEnvSettings.csharp`
  - `docs/examples/create-default-from-env-settings/datasource.create_default_from_env_settings.rust`
  - `docs/examples/create-default-from-env-settings/Datasource.test.create-default-from-env-settings.typescript`
- Default using environment variables
  - `docs/examples/default-using-environment-variables/DatasourceFactoryTests.DefaultFromEnvironment.csharp`
  - `docs/examples/default-using-environment-variables/datasource_factory.default_from_environment.rust`
  - `docs/examples/default-using-environment-variables/DatasourceFactory.test.default-from-environment.typescript`
- Create using Dynamic After Register
  - `docs/examples/create-using-dynamic-after-register/DatasourcePluralIntegrationTests.CreatesUsingDynamicAfterRegister.csharp`
  - `docs/examples/create-using-dynamic-after-register/datasource_plural.creates_using_dynamic_after_register.rust`
  - `docs/examples/create-using-dynamic-after-register/datasource.repository.integration.test.creates-using-dynamic-after-register.typescript`
- Create from a driver
  - `docs/examples/create-from-a-driver/CreateFromDriverTests.CreateFromDriver.csharp`
  - `docs/examples/create-from-a-driver/create_from_driver.create_from_driver.rust`
  - `docs/examples/create-from-a-driver/createFromDriver.test.create-from-a-driver.typescript`

## Connection

- Create from Default
  - `docs/examples/create-from-default/ConnectionTests.CreateFromDefault.csharp`
  - `docs/examples/create-from-default/connection.create_from_default.rust`
  - `docs/examples/create-from-default/Connection.test.create-from-default.typescript`
- Create and run a command
  - `docs/examples/create-and-run-a-command/ConnectionLifecycleTests.CreateAndRunCommand.csharp`
  - `docs/examples/create-and-run-a-command/memory-lib.create_and_run_a_command.rust`
  - `docs/examples/create-and-run-a-command/index.test.create-and-run-a-command.typescript`
- Close with using
  - `docs/examples/close-with-using/ConnectionLifecycleTests.CloseWithUsing.csharp`
  - `docs/examples/close-with-using/memory-lib.open_and_close.rust`
  - `docs/examples/close-with-using/index.test.closes-with-using.typescript`
- Start a transaction
  - `docs/examples/start-a-transaction/ConnectionLifecycleTests.StartTransaction.csharp`
  - `docs/examples/start-a-transaction/memory-lib.start_a_transaction.rust`
  - `docs/examples/start-a-transaction/index.test.start-a-transaction.typescript`

## Transaction

- Commit or abort
  - `docs/examples/commit-or-abort/TransactionTests.CommitOrAbort.csharp`
  - `docs/examples/commit-or-abort/transaction.commit_or_abort.rust`
  - `docs/examples/commit-or-abort/Transaction.test.commit-or-abort.typescript`
- Command running in one atomic transaction
  - `docs/examples/command-running-in-one-atomic-transaction/TransactionLifecycleTests.CommandRunningInOneAtomicTransaction.csharp`
  - `docs/examples/command-running-in-one-atomic-transaction/memory-lib.command_running_in_one_atomic_transaction.rust`
  - `docs/examples/command-running-in-one-atomic-transaction/index.test.command-running-in-one-atomic-transaction.typescript`

## Command

- Run executeReader on a simple list
  - `docs/examples/run-executereader-on-a-simple-list/SqliteCommandIntegrationTests.RunExecuteReaderOnASimpleList.csharp`
  - `docs/examples/run-executereader-on-a-simple-list/command.run_execute_reader_on_a_simple_list.rust`
  - `docs/examples/run-executereader-on-a-simple-list/sqlite.integration.test.run-executereader-on-a-simple-list.typescript`
- Run ExecuteReader and return NameValues
  - `docs/examples/run-executereader-and-return-namevalues/SqliteCommandIntegrationTests.NameValue.csharp`
  - `docs/examples/run-executereader-and-return-namevalues/SqliteCommandIntegrationTests.RunExecuteReaderAndReturnNameValues.csharp`
  - `docs/examples/run-executereader-and-return-namevalues/command.ConvertSchemaForNameValue.rust`
  - `docs/examples/run-executereader-and-return-namevalues/command.run_execute_reader_and_return_name_values.rust`
  - `docs/examples/run-executereader-and-return-namevalues/sqlite.integration.test.NameValue.typescript`
  - `docs/examples/run-executereader-and-return-namevalues/sqlite.integration.test.run-executereader-and-return-namevalues.typescript`
- Execute, Insert, Read, Update, and Delete
  - `docs/examples/execute-insert-read-update-and-delete/SqliteCommandIntegrationTests.Item.csharp`
  - `docs/examples/execute-insert-read-update-and-delete/SqliteCommandIntegrationTests.ExecuteInsertReadUpdateAndDelete.csharp`
  - `docs/examples/execute-insert-read-update-and-delete/command.ConvertSchemaForItem.rust`
  - `docs/examples/execute-insert-read-update-and-delete/command.execute_insert_read_update_and_delete.rust`
  - `docs/examples/execute-insert-read-update-and-delete/sqlite.integration.test.Item.typescript`
  - `docs/examples/execute-insert-read-update-and-delete/sqlite.integration.test.execute-insert-read-update-and-delete.typescript`
- Map flat contacts
  - `docs/examples/map-flat-contacts/SqliteCommandIntegrationTests.User.csharp`
  - `docs/examples/map-flat-contacts/SqliteCommandIntegrationTests.ExecuteReaderMapsFlatContacts.csharp`
  - `docs/examples/map-flat-contacts/command.Contact.rust`
  - `docs/examples/map-flat-contacts/command.User.rust`
  - `docs/examples/map-flat-contacts/command.execute_reader_maps_flat_contacts.rust`
  - `docs/examples/map-flat-contacts/sqlite.integration.test.Contact.typescript`
  - `docs/examples/map-flat-contacts/sqlite.integration.test.User.typescript`
  - `docs/examples/map-flat-contacts/sqlite.integration.test.maps-flat-contacts.typescript`
- Fill an output parameter
  - `docs/examples/fill-an-output-parameter/MysqlCommandIntegrationTests.FillOutputParameter.csharp`
  - `docs/examples/fill-an-output-parameter/integration.fill_output_parameter.rust`
  - `docs/examples/fill-an-output-parameter/mysql.integration.test.fillOutputParameter.typescript`
- Read a return value
  - `docs/examples/read-a-return-value/PostgresCommandIntegrationTests.ReadReturnValue.csharp`
  - `docs/examples/read-a-return-value/integration.read_return_value.rust`
  - `docs/examples/read-a-return-value/postgres.integration.test.readReturnValue.typescript`

## Reader

- Read a row
  - `docs/examples/read-a-row/SqliteCommandIntegrationTests.RunExecuteReaderOnASimpleList.csharp`
  - `docs/examples/read-a-row/command.run_execute_reader_on_a_simple_list.rust`
  - `docs/examples/read-a-row/sqlite.integration.test.run-executereader-on-a-simple-list.typescript`
- Read and apply case mappings
  - `docs/examples/read-and-apply-case-mappings/SqliteCommandIntegrationTests.ReadAndApplyCaseMappings.csharp`
  - `docs/examples/read-and-apply-case-mappings/command.read_and_apply_case_mappings.rust`
  - `docs/examples/read-and-apply-case-mappings/sqlite.integration.test.read-and-apply-case-mappings.typescript`
- ExecuteReader, then apply flat to nested mapper.
  - `docs/examples/executereader-then-apply-flat-to-nested-mapper/SqliteCommandIntegrationTests.ExecuteReaderThenApplyFlatToNestedMapper.csharp`
  - `docs/examples/executereader-then-apply-flat-to-nested-mapper/command.execute_reader_then_apply_flat_to_nested_mapper.rust`
  - `docs/examples/executereader-then-apply-flat-to-nested-mapper/sqlite.integration.test.executereader-then-apply-flat-to-nested-mapper.typescript`
- ExecuteReader maps flat contacts
  - `docs/examples/executereader-maps-flat-contacts/SqliteCommandIntegrationTests.User.csharp`
  - `docs/examples/executereader-maps-flat-contacts/SqliteCommandIntegrationTests.ExecuteReaderMapsFlatContacts.csharp`
  - `docs/examples/executereader-maps-flat-contacts/command.Contact.rust`
  - `docs/examples/executereader-maps-flat-contacts/command.User.rust`
  - `docs/examples/executereader-maps-flat-contacts/command.execute_reader_maps_flat_contacts.rust`
  - `docs/examples/executereader-maps-flat-contacts/sqlite.integration.test.Contact.typescript`
  - `docs/examples/executereader-maps-flat-contacts/sqlite.integration.test.User.typescript`
  - `docs/examples/executereader-maps-flat-contacts/sqlite.integration.test.maps-flat-contacts.typescript`
- Read multiple results returned in a single command
  - `docs/examples/read-multiple-results-returned-in-a-single-command/SqliteCommandIntegrationTests.ReadMultipleResults.csharp`
  - `docs/examples/read-multiple-results-returned-in-a-single-command/command.read_multiple_results.rust`
  - `docs/examples/read-multiple-results-returned-in-a-single-command/sqlite.integration.test.reads-multiple-results-returned-in-a-single-command.typescript`
- Call a stored procedure that returns two results
  - `docs/examples/call-a-stored-procedure-that-returns-two-results/MysqlCommandIntegrationTests.CallProcedureReturnsTwoResults.csharp`
  - `docs/examples/call-a-stored-procedure-that-returns-two-results/integration.call_procedure_returns_two_results.rust`
  - `docs/examples/call-a-stored-procedure-that-returns-two-results/mysql.integration.test.callProcedureReturnsTwoResults.typescript`

## Repositories

- Simple readonly
  - `docs/examples/simple-readonly/ReadonlyRepositoryIntegrationTests.User.csharp`
  - `docs/examples/simple-readonly/ReadonlyRepositoryIntegrationTests.SnakeThenPlural.csharp`
  - `docs/examples/simple-readonly/ReadonlyRepositoryIntegrationTests.Snake.csharp`
  - `docs/examples/simple-readonly/ReadonlyRepositoryIntegrationTests.ReadsUsersByIdAndByUsernameAndEnabled.csharp`
  - `docs/examples/simple-readonly/readonly_repository.ConvertSchemaForUser-1.rust`
  - `docs/examples/simple-readonly/readonly_repository.snake_then_plural.rust`
  - `docs/examples/simple-readonly/readonly_repository.snake.rust`
  - `docs/examples/simple-readonly/readonly_repository.reads_users_by_id_and_by_username_and_enabled.rust`
  - `docs/examples/simple-readonly/readonly.repository.integration.test.User-1.typescript`
  - `docs/examples/simple-readonly/readonly.repository.integration.test.snakeThenPlural.typescript`
  - `docs/examples/simple-readonly/readonly.repository.integration.test.snake.typescript`
  - `docs/examples/simple-readonly/readonly.repository.integration.test.reads-users-by-id-and-by-username-and-enabled.typescript`
- Create, Read, Update, and Delete
  - `docs/examples/create-read-update-and-delete/ReadonlyRepositoryIntegrationTests.User.csharp`
  - `docs/examples/create-read-update-and-delete/ReadonlyRepositoryIntegrationTests.WritesUserFromTheType.csharp`
  - `docs/examples/create-read-update-and-delete/readonly_repository.ConvertSchemaForUser-1.rust`
  - `docs/examples/create-read-update-and-delete/readonly_repository.writes_user_from_the_type.rust`
  - `docs/examples/create-read-update-and-delete/readonly.repository.integration.test.User-1.typescript`
  - `docs/examples/create-read-update-and-delete/readonly.repository.integration.test.writes-a-user-from-the-type.typescript`
- Enriched Create, Read, Update, and Delete
  - `docs/examples/enriched-create-read-update-and-delete/ReadonlyRepositoryIntegrationTests.usersviewcrudtests-User.csharp`
  - `docs/examples/enriched-create-read-update-and-delete/ReadonlyRepositoryIntegrationTests.UserType.csharp`
  - `docs/examples/enriched-create-read-update-and-delete/ReadonlyRepositoryIntegrationTests.UserRole.csharp`
  - `docs/examples/enriched-create-read-update-and-delete/ReadonlyRepositoryIntegrationTests.UpdatesUsersViewWhenTheVersionMatches.csharp`
  - `docs/examples/enriched-create-read-update-and-delete/readonly_repository.ConvertSchemaForUser-2.rust`
  - `docs/examples/enriched-create-read-update-and-delete/readonly_repository.ConvertSchemaForUserType-3.rust`
  - `docs/examples/enriched-create-read-update-and-delete/readonly_repository.ConvertSchemaForUserRole-2.rust`
  - `docs/examples/enriched-create-read-update-and-delete/readonly_repository.updates_users_view_when_the_version_matches.rust`
  - `docs/examples/enriched-create-read-update-and-delete/readonly.repository.integration.test.users-view-crud-User.typescript`
  - `docs/examples/enriched-create-read-update-and-delete/readonly.repository.integration.test.users-view-crud-UserType.typescript`
  - `docs/examples/enriched-create-read-update-and-delete/readonly.repository.integration.test.UserRole.typescript`
  - `docs/examples/enriched-create-read-update-and-delete/readonly.repository.integration.test.updates-a-user-view-when-the-version-matches.typescript`
- Bulk
  - `docs/examples/bulk/BulkCRUDRepositoryTests.CreateWritesParentsAndChildren.csharp`
  - `docs/examples/bulk/enriched.bulk_create_writes_parents_and_children.rust`
  - `docs/examples/bulk/BulkCRUDRepository.test.creates-parents-and-children.typescript`

## Readonly Repository

- Simple readonly
  - `docs/examples/simple-readonly/ReadonlyRepositoryIntegrationTests.ReadsUsersByIdAndByUsernameAndEnabled.csharp`
  - `docs/examples/simple-readonly/readonly_repository.reads_users_by_id_and_by_username_and_enabled.rust`
  - `docs/examples/simple-readonly/readonly.repository.integration.test.reads-users-by-id-and-by-username-and-enabled.typescript`
- Map an object explicitly
  - `docs/examples/map-an-object-explicitly/ReadonlyRepositoryIntegrationTests.UserConverter.csharp`
  - `docs/examples/map-an-object-explicitly/ReadonlyRepositoryIntegrationTests.MapsAnObjectExplicitly.csharp`
  - `docs/examples/map-an-object-explicitly/readonly_repository.TypeConverterForUserConverter.rust`
  - `docs/examples/map-an-object-explicitly/readonly_repository.maps_an_object_explicitly.rust`
  - `docs/examples/map-an-object-explicitly/readonly.repository.integration.test.UserConverter.typescript`
  - `docs/examples/map-an-object-explicitly/readonly.repository.integration.test.maps-an-object-explicitly.typescript`
- Enriched readonly
  - `docs/examples/enriched-readonly/ReadonlyRepositoryIntegrationTests.FillsUserTypeOnUsersView.csharp`
  - `docs/examples/enriched-readonly/readonly_repository.fills_user_type_on_user_view.rust`
  - `docs/examples/enriched-readonly/readonly.repository.integration.test.fills-user-type-on-a-user-view.typescript`
- Read a user by guid, username, and created
  - `docs/examples/read-a-user-by-guid-username-and-created/ReadonlyRepositoryIntegrationTests.ReadsUserByGuidUsernameAndCreated.csharp`
  - `docs/examples/read-a-user-by-guid-username-and-created/readonly_repository.reads_user_by_guid_username_and_created.rust`
  - `docs/examples/read-a-user-by-guid-username-and-created/readonly.repository.integration.test.reads-user-by-guid-username-and-created.typescript`
- Call a stored procedure
  - `docs/examples/call-a-stored-procedure/RepositoryTests.ReadCallsStoredProcedure.csharp`
  - `docs/examples/call-a-stored-procedure/datasources-mod.evaluates_where_statement.rust`
  - `docs/examples/call-a-stored-procedure/repository.test.calls-a-stored-procedure-and-maps-rows.typescript`
- Read and read single
  - `docs/examples/read-and-read-single/RepositoryTests.GetBuildsSelectAndMapsRows.csharp`
  - `docs/examples/read-and-read-single/datasources-mod.evaluates_where_statement.rust`
  - `docs/examples/read-and-read-single/repository.test.builds-select-and-maps-rows.typescript`
- Fill a lookup
  - `docs/examples/fill-a-lookup/ReadonlyRepositoryTests.ReadFillsLookupSelectAndLeavesMissingDefault.csharp`
  - `docs/examples/fill-a-lookup/enriched.enriched_fills_lookup_select_and_leaves_missing_default.rust`
  - `docs/examples/fill-a-lookup/ReadonlyRepository.test.fills-lookup-select-and-leaves-missing-default.typescript`
- Fill roles from one join
  - `docs/examples/fill-roles-from-one-join/ReadonlyRepositoryIntegrationTests.FillsRolesFromAJoin.csharp`
  - `docs/examples/fill-roles-from-one-join/readonly_repository.fills_roles_from_a_join.rust`
  - `docs/examples/fill-roles-from-one-join/users-view-roles.repository.integration.test.fills-roles-from-a-join.typescript`
- Read with where clause and custom flat to nested custom object mapping options
  - `docs/examples/read-with-where-clause-and-custom-flat-to-nested-custom-object-mapping-options/ReadWithWhereClauseAndCustomFlatToNestedObjectMappingOptionsTests.ReadWithWhereClauseAndCustomFlatToNestedObjectMappingOptions.csharp`
  - `docs/examples/read-with-where-clause-and-custom-flat-to-nested-custom-object-mapping-options/read_with_where_clause_and_custom_flat_to_nested_object_mapping_options.read_with_where_clause_and_custom_flat_to_nested_object_mapping_options.rust`
  - `docs/examples/read-with-where-clause-and-custom-flat-to-nested-custom-object-mapping-options/readWithWhereClauseAndCustomFlatToNestedObjectMappingOptions.test.read-with-where-clause-and-custom-flat-to-nested-object-mapping-options.typescript`

## CRUD Repository

- Simple CRUD
  - `docs/examples/simple-crud/ReadonlyRepositoryIntegrationTests.WritesUserFromTheType.csharp`
  - `docs/examples/simple-crud/readonly_repository.writes_user_from_the_type.rust`
  - `docs/examples/simple-crud/readonly.repository.integration.test.writes-a-user-from-the-type.typescript`
- Enriched CRUD
  - `docs/examples/enriched-crud/UserViewCRUDRepositoryTests.CreateUpdateAndDeleteSyncsLinkRows.csharp`
  - `docs/examples/enriched-crud/user_view.create_update_and_delete_syncs_link_rows.rust`
  - `docs/examples/enriched-crud/UserViewCRUDRepository.test.creates-updates-and-deletes-through-link-rows.typescript`
- Multi-tenant
  - `docs/examples/multi-tenant/ReadonlyRepositoryIntegrationTests.ReadsUpdatesAndDeletesByTenantAndId.csharp`
  - `docs/examples/multi-tenant/readonly_repository.reads_updates_and_deletes_by_tenant_and_id.rust`
  - `docs/examples/multi-tenant/readonly.repository.integration.test.reads-updates-and-deletes-by-tenant-and-id.typescript`
- Enriched contacts write
  - `docs/examples/enriched-contacts-write/ContactsRepositoryTests.WritesEnrichedContact.csharp`
  - `docs/examples/enriched-contacts-write/contacts.writes_enriched_contact.rust`
  - `docs/examples/enriched-contacts-write/contacts.repository.integration.test.writes-enriched-contact.typescript`
- Type mapping
  - `docs/examples/type-mapping/ReadonlyRepositoryIntegrationTests.UpdatesUsersViewWhenTheVersionMatches.csharp`
  - `docs/examples/type-mapping/readonly_repository.updates_users_view_when_the_version_matches.rust`
  - `docs/examples/type-mapping/readonly.repository.integration.test.updates-a-user-view-when-the-version-matches.typescript`

## Bulk Repository

- Create parents and children
  - `docs/examples/create-parents-and-children/BulkCRUDRepositoryTests.CreateWritesParentsAndChildren.csharp`
  - `docs/examples/create-parents-and-children/enriched.bulk_create_writes_parents_and_children.rust`
  - `docs/examples/create-parents-and-children/BulkCRUDRepository.test.creates-parents-and-children.typescript`

## Repository Factories

- Register and create
  - `docs/examples/register-and-create/RepositoryFactoryTests.RegistersAndCreates.csharp`
  - `docs/examples/register-and-create/repository_factory.registers_and_creates.rust`
  - `docs/examples/register-and-create/RepositoryFactory.test.registers-and-creates.typescript`

## Mappers

- Transform a contact
  - `docs/examples/transform-a-contact/TransformMapperTests.MapsAContactNameAndEmail.csharp`
  - `docs/examples/transform-a-contact/mappers-mod.maps_a_contact_name_and_email.rust`
  - `docs/examples/transform-a-contact/transformMapper.test.maps-a-contact-name-and-email.typescript`
- Change casing
  - `docs/examples/change-casing/ChangeCasingMapperTests.MapsContactNames.csharp`
  - `docs/examples/change-casing/change_casing_mapper.maps_contact_names.rust`
  - `docs/examples/change-casing/ChangeCasingMapper.test.maps-contact-names.typescript`
  - `docs/examples/change-casing/ChangeCasingMapperTests.ReverseMapsContactNames.csharp`
  - `docs/examples/change-casing/change_casing_mapper.reverse_maps_contact_names.rust`
  - `docs/examples/change-casing/ChangeCasingMapper.test.reverse-maps-contact-names.typescript`
- Lowercase plural
  - `docs/examples/lowercase-plural/PluralizerMapperTests.LowercasesThenPluralizes.csharp`
  - `docs/examples/lowercase-plural/pluralizer_mapper.lowercases_then_pluralizes.rust`
  - `docs/examples/lowercase-plural/PluralizerMapper.test.lowercases-then-pluralizes.typescript`
- Flat to nested
  - `docs/examples/flat-to-nested/FlatToNestedMapperTests.MapsJoinRowsToANestedContact.csharp`
  - `docs/examples/flat-to-nested/flat_to_nested_mapper.maps_join_rows_to_a_nested_contact.rust`
  - `docs/examples/flat-to-nested/FlatToNestedMapper.test.maps-join-rows-to-a-nested-contact.typescript`

## Identity Mapper

- Map and reverse map
  - `docs/examples/map-and-reverse-map/IdentityMapperTests.MapsAndReversesTheSameValue.csharp`
  - `docs/examples/map-and-reverse-map/identity_mapper.maps_and_reverses_the_same_value.rust`
  - `docs/examples/map-and-reverse-map/IdentityMapper.test.maps-and-reverses-the-same-value.typescript`

## Dictionary Mapper

- Map and reverse map
  - `docs/examples/map-and-reverse-map/DictionaryMapperTests.MapsKnownNamesAndPassesThroughTheRest.csharp`
  - `docs/examples/map-and-reverse-map/dictionary_mapper.maps_known_names_and_passes_through_the_rest.rust`
  - `docs/examples/map-and-reverse-map/DictionaryMapper.test.maps-known-names-and-passes-through-the-rest.typescript`
- First key wins on reverse
  - `docs/examples/first-key-wins-on-reverse/DictionaryMapperTests.FirstKeyWinsOnReverse.csharp`
  - `docs/examples/first-key-wins-on-reverse/dictionary_mapper.first_key_wins_on_reverse.rust`
  - `docs/examples/first-key-wins-on-reverse/DictionaryMapper.test.first-key-wins-on-reverse.typescript`

## Pluralizer Mapper

- Map contacts
  - `docs/examples/map-contacts/PluralizerMapperTests.MapsContacts.csharp`
  - `docs/examples/map-contacts/pluralizer_mapper.maps_contacts.rust`
  - `docs/examples/map-contacts/PluralizerMapper.test.maps-contacts.typescript`
- Reverse map contacts
  - `docs/examples/reverse-map-contacts/PluralizerMapperTests.ReverseMapsContacts.csharp`
  - `docs/examples/reverse-map-contacts/pluralizer_mapper.reverse_maps_contacts.rust`
  - `docs/examples/reverse-map-contacts/PluralizerMapper.test.reverse-maps-contacts.typescript`
- Lowercase plural
  - `docs/examples/lowercase-plural/PluralizerMapperTests.LowercasesThenPluralizes.csharp`
  - `docs/examples/lowercase-plural/pluralizer_mapper.lowercases_then_pluralizes.rust`
  - `docs/examples/lowercase-plural/PluralizerMapper.test.lowercases-then-pluralizes.typescript`

## Change Casing Mapper

- Map contacts
  - `docs/examples/map-contacts/ChangeCasingMapperTests.MapsContactNames.csharp`
  - `docs/examples/map-contacts/change_casing_mapper.maps_contact_names.rust`
  - `docs/examples/map-contacts/ChangeCasingMapper.test.maps-contact-names.typescript`
- Reverse map contacts
  - `docs/examples/reverse-map-contacts/ChangeCasingMapperTests.ReverseMapsContactNames.csharp`
  - `docs/examples/reverse-map-contacts/change_casing_mapper.reverse_maps_contact_names.rust`
  - `docs/examples/reverse-map-contacts/ChangeCasingMapper.test.reverse-maps-contact-names.typescript`

## Transform Mapper

- Flatten with TransformMapper
  - `docs/examples/flatten-with-transformmapper/TransformMapperTests.FlattensAndUnflattensUserAndContacts.csharp`
  - `docs/examples/flatten-with-transformmapper/mappers-mod.flattens_and_unflattens_user_and_contacts.rust`
  - `docs/examples/flatten-with-transformmapper/transformMapper.test.flattens-and-unflattens-user-and-contacts.typescript`
- Prefix a contact field
  - `docs/examples/prefix-a-contact-field/TransformMapperTests.PrefixesAContactField.csharp`
  - `docs/examples/prefix-a-contact-field/mappers-mod.prefixes_a_contact_field.rust`
  - `docs/examples/prefix-a-contact-field/transformMapper.test.prefixes-a-contact-field.typescript`
- Map a contact name and email
  - `docs/examples/map-a-contact-name-and-email/TransformMapperTests.MapsAContactNameAndEmail.csharp`
  - `docs/examples/map-a-contact-name-and-email/mappers-mod.maps_a_contact_name_and_email.rust`
  - `docs/examples/map-a-contact-name-and-email/transformMapper.test.maps-a-contact-name-and-email.typescript`

## Flat To Nested Mapper

- Map join rows to a nested contact
  - `docs/examples/map-join-rows-to-a-nested-contact/FlatToNestedMapperTests.MapsJoinRowsToANestedContact.csharp`
  - `docs/examples/map-join-rows-to-a-nested-contact/flat_to_nested_mapper.maps_join_rows_to_a_nested_contact.rust`
  - `docs/examples/map-join-rows-to-a-nested-contact/FlatToNestedMapper.test.maps-join-rows-to-a-nested-contact.typescript`

## Metadata Row Type Converter

- Convert a row
  - `docs/examples/convert-a-row/RowTypeConverterTests.ConvertAndReverseConvertRoundTrip.csharp`
  - `docs/examples/convert-a-row/metadata_row_type_converter.copies_schema_columns_and_drops_the_rest.rust`
  - `docs/examples/convert-a-row/RowTypeConverter.test.convert-and-reverseconvert-round-trip.typescript`
- Field converter
  - `docs/examples/field-converter/RowTypeConverterTests.AppliesFieldOverrideOnDictionaryRows.csharp`
  - `docs/examples/field-converter/row_type_converter.field_override_beats_type_default.rust`
  - `docs/examples/field-converter/RowTypeConverter.test.lets-field-converters-override.typescript`

## Select Statement

- Wrap query text and an optional where
  - `docs/examples/wrap-query-text-and-an-optional-where/StatementCompileTests.WrapsQueryTextAndAnOptionalWhere.csharp`
  - `docs/examples/wrap-query-text-and-an-optional-where/statement_compile.wraps_query_text_and_an_optional_where.rust`
  - `docs/examples/wrap-query-text-and-an-optional-where/StatementCompile.test.wraps-query-text-and-an-optional-where.typescript`
- Compile a SELECT
  - `docs/examples/compile-a-select/StatementCompileTests.CompilesASelect.csharp`
  - `docs/examples/compile-a-select/statement_compile.compiles_a_select.rust`
  - `docs/examples/compile-a-select/StatementCompile.test.compiles-a-select.typescript`
- Join tables
  - `docs/examples/join-tables/StatementCompileTests.JoinsTables.csharp`
  - `docs/examples/join-tables/statement_compile.joins_tables.rust`
  - `docs/examples/join-tables/StatementCompile.test.joins-tables.typescript`
- Group, aggregate, order, and limit
  - `docs/examples/group-aggregate-order-and-limit/StatementCompileTests.GroupsAggregatesOrdersAndLimits.csharp`
  - `docs/examples/group-aggregate-order-and-limit/statement_compile.groups_aggregates_orders_and_limits.rust`
  - `docs/examples/group-aggregate-order-and-limit/StatementCompile.test.groups-aggregates-orders-and-limits.typescript`
- Group rows
  - `docs/examples/group-rows/StatementCompileTests.GroupsRows.csharp`
  - `docs/examples/group-rows/statement_compile.groups_rows.rust`
  - `docs/examples/group-rows/StatementCompile.test.groups-rows.typescript`
- Aggregate without grouping
  - `docs/examples/aggregate-without-grouping/StatementCompileTests.AggregatesWithoutGrouping.csharp`
  - `docs/examples/aggregate-without-grouping/statement_compile.aggregates_without_grouping.rust`
  - `docs/examples/aggregate-without-grouping/StatementCompile.test.aggregates-without-grouping.typescript`
- Order rows
  - `docs/examples/order-rows/StatementCompileTests.OrdersRows.csharp`
  - `docs/examples/order-rows/statement_compile.orders_rows.rust`
  - `docs/examples/order-rows/StatementCompile.test.orders-rows.typescript`
- Limit rows
  - `docs/examples/limit-rows/StatementCompileTests.LimitsRows.csharp`
  - `docs/examples/limit-rows/statement_compile.limits_rows.rust`
  - `docs/examples/limit-rows/StatementCompile.test.limits-rows.typescript`

## Insert Statement

- Compile an INSERT
  - `docs/examples/compile-an-insert/StatementCompileTests.CompilesAnInsert.csharp`
  - `docs/examples/compile-an-insert/statement_compile.compiles_an_insert.rust`
  - `docs/examples/compile-an-insert/StatementCompile.test.compiles-an-insert.typescript`

## Update Statement

- Compile an UPDATE
  - `docs/examples/compile-an-update/StatementCompileTests.CompilesAnUpdate.csharp`
  - `docs/examples/compile-an-update/statement_compile.compiles_an_update.rust`
  - `docs/examples/compile-an-update/StatementCompile.test.compiles-an-update.typescript`

## Delete Statement

- Compile a DELETE
  - `docs/examples/compile-a-delete/StatementCompileTests.CompilesADelete.csharp`
  - `docs/examples/compile-a-delete/statement_compile.compiles_a_delete.rust`
  - `docs/examples/compile-a-delete/StatementCompile.test.compiles-a-delete.typescript`

## Create Table Statement

- Compile a CREATE TABLE
  - `docs/examples/compile-a-create-table/StatementCompileTests.CompilesACreateTable.csharp`
  - `docs/examples/compile-a-create-table/statement_compile.compiles_a_create_table.rust`
  - `docs/examples/compile-a-create-table/StatementCompile.test.compiles-a-create-table.typescript`
- Compile named constraints
  - `docs/examples/compile-named-constraints/StatementCompileTests.CompilesNamedConstraints.csharp`
  - `docs/examples/compile-named-constraints/statement_compile.compiles_named_constraints.rust`
  - `docs/examples/compile-named-constraints/StatementCompile.test.compiles-named-constraints.typescript`

## Create Index Statement

- Compile a CREATE INDEX
  - `docs/examples/compile-a-create-index/StatementCompileTests.CompilesACreateIndex.csharp`
  - `docs/examples/compile-a-create-index/statement_compile.compiles_a_create_index.rust`
  - `docs/examples/compile-a-create-index/StatementCompile.test.compiles-a-create-index.typescript`
- Compile a CREATE UNIQUE INDEX
  - `docs/examples/compile-a-create-unique-index/StatementCompileTests.CompilesACreateUniqueIndex.csharp`
  - `docs/examples/compile-a-create-unique-index/statement_compile.compiles_a_create_unique_index.rust`
  - `docs/examples/compile-a-create-unique-index/StatementCompile.test.compiles-a-create-unique-index.typescript`

## Execute Stored Procedure Statement

- Compile a CALL
  - `docs/examples/compile-a-call/StatementCompileTests.CompilesACall.csharp`
  - `docs/examples/compile-a-call/statement_compile.compiles_a_call.rust`
  - `docs/examples/compile-a-call/StatementCompile.test.compiles-a-call.typescript`

## Return Statement

- Compile a SELECT function call
  - `docs/examples/compile-a-select-function-call/StatementCompileTests.CompilesASelectFunctionCall.csharp`
  - `docs/examples/compile-a-select-function-call/statement_compile.compiles_a_select_function_call.rust`
  - `docs/examples/compile-a-select-function-call/StatementCompile.test.compiles-a-select-function-call.typescript`

## Where

- Compare, groups, not, and in
  - `docs/examples/compare-groups-not-and-in/StatementCompileTests.RendersCompareGroupsNotAndIn.csharp`
  - `docs/examples/compare-groups-not-and-in/statement_compile.renders_compare_groups_not_and_in.rust`
  - `docs/examples/compare-groups-not-and-in/StatementCompile.test.renders-compare-groups-not-and-in.typescript`
- Column, alias, literal
  - `docs/examples/column-alias-literal/ComparisonAliasTests.RendersComparisonSymbols.csharp`
  - `docs/examples/column-alias-literal/datasources-mod.comparison_aliases.rust`
  - `docs/examples/column-alias-literal/builders.test.renders-comparison-symbols.typescript`
- Attach to a query
  - `docs/examples/attach-to-a-query/StatementCompileTests.AttachesWhereToAQuery.csharp`
  - `docs/examples/attach-to-a-query/statement_compile.attaches_where_to_a_query.rust`
  - `docs/examples/attach-to-a-query/StatementCompile.test.attaches-where-to-a-query.typescript`

## Expressions

- Compare tree
  - `docs/examples/compare-tree/StatementCompileTests.CompareTree.csharp`
  - `docs/examples/compare-tree/statement_compile.compare_tree.rust`
  - `docs/examples/compare-tree/StatementCompile.test.compare-tree.typescript`
- Column, alias, literal
  - `docs/examples/column-alias-literal/ComparisonAliasTests.RendersComparisonSymbols.csharp`
  - `docs/examples/column-alias-literal/datasources-mod.comparison_aliases.rust`
  - `docs/examples/column-alias-literal/builders.test.renders-comparison-symbols.typescript`

## Expression Evaluator

- Match and miss
  - `docs/examples/match-and-miss/CoreTests.EvaluatesWhere.csharp`
  - `docs/examples/match-and-miss/datasources-mod.evaluates_where_statement.rust`
  - `docs/examples/match-and-miss/builders.test.evaluates-compare-groups-not-and-in.typescript`

## Datasource Factory

- Register and Create
  - `docs/examples/register-and-create/RegisterAndCreateTests.RegisterAndCreate.csharp`
  - `docs/examples/register-and-create/memory-lib.register_and_create.rust`
  - `docs/examples/register-and-create/index.test.register-and-create.typescript`
- Missing provider
  - `docs/examples/missing-provider/DatasourceFactoryTests.MissingProviderThrows.csharp`
  - `docs/examples/missing-provider/datasource_factory.missing_provider_throws.rust`
  - `docs/examples/missing-provider/DatasourceFactory.test.missing-provider-throws.typescript`

## Datasource URL

- Postgres URL
  - `docs/examples/postgres-url/DatasourceUrlParserTests.PostgresUrl.csharp`
  - `docs/examples/postgres-url/datasource_url_parser.postgres_url.rust`
  - `docs/examples/postgres-url/DatasourceUrlParser.test.postgres-url.typescript`
- SQLite file
  - `docs/examples/sqlite-file/DatasourceUrlParserTests.SqliteRelativeFile.csharp`
  - `docs/examples/sqlite-file/datasource_url_parser.sqlite_relative_file.rust`
  - `docs/examples/sqlite-file/DatasourceUrlParser.test.sqlite-relative-file.typescript`
- Not a URL
  - `docs/examples/not-a-url/DatasourceUrlParserTests.NotAUrl.csharp`
  - `docs/examples/not-a-url/datasource_url_parser.not_a_url.rust`
  - `docs/examples/not-a-url/DatasourceUrlParser.test.not-a-url.typescript`

## IEnriched

- Register a fill
  - `docs/examples/register-a-fill/ReadonlyRepositoryTests.ReadFillsLookupSelectAndLeavesMissingDefault.csharp`
  - `docs/examples/register-a-fill/enriched.enriched_fills_lookup_select_and_leaves_missing_default.rust`
  - `docs/examples/register-a-fill/ReadonlyRepository.test.fills-lookup-select-and-leaves-missing-default.typescript`

## Enrichment

- Fill a lookup
  - `docs/examples/fill-a-lookup/ReadonlyRepositoryTests.ReadFillsLookupSelectAndLeavesMissingDefault.csharp`
  - `docs/examples/fill-a-lookup/enriched.enriched_fills_lookup_select_and_leaves_missing_default.rust`
  - `docs/examples/fill-a-lookup/ReadonlyRepository.test.fills-lookup-select-and-leaves-missing-default.typescript`

## IReadonlyEnrichmentStrategy

- Fill a lookup
  - `docs/examples/fill-a-lookup/ReadonlyRepositoryTests.ReadFillsLookupSelectAndLeavesMissingDefault.csharp`
  - `docs/examples/fill-a-lookup/enriched.enriched_fills_lookup_select_and_leaves_missing_default.rust`
  - `docs/examples/fill-a-lookup/ReadonlyRepository.test.fills-lookup-select-and-leaves-missing-default.typescript`

## DefaultReadonlyEnrichmentStrategy

- Fill a lookup
  - `docs/examples/fill-a-lookup/ReadonlyRepositoryTests.ReadFillsLookupSelectAndLeavesMissingDefault.csharp`
  - `docs/examples/fill-a-lookup/enriched.enriched_fills_lookup_select_and_leaves_missing_default.rust`
  - `docs/examples/fill-a-lookup/ReadonlyRepository.test.fills-lookup-select-and-leaves-missing-default.typescript`

## HashsetOptimizedReadonlyEnrichmentStrategy

- Fill with one query
  - `docs/examples/fill-with-one-query/ReadonlyRepositoryTests.HashsetFillsLookupSelectAndLeavesMissingDefault.csharp`
  - `docs/examples/fill-with-one-query/readonly_repository.fills_roles_with_a_hashset.rust`
  - `docs/examples/fill-with-one-query/ReadonlyRepository.test.hashset-fills-lookup-select-and-leaves-missing-default.typescript`

## NestedToObjectEnrichmentStrategy

- Collapse join rows
  - `docs/examples/collapse-join-rows/ReadonlyRepositoryTests.FlatToNestedCollapsesJoinRows.csharp`
  - `docs/examples/collapse-join-rows/readonly_repository.fills_roles_from_a_join.rust`
  - `docs/examples/collapse-join-rows/ReadonlyRepository.test.collapses-join-rows.typescript`

## IWriteEnrichmentStrategy

- Roll back a bulk update
  - `docs/examples/roll-back-a-bulk-update/BulkCRUDRepositoryTests.BatchUpdateRollsBackWhenTransactionIsOn.csharp`
  - `docs/examples/roll-back-a-bulk-update/enriched.bulk_update_rolls_back_when_transaction_is_on.rust`
  - `docs/examples/roll-back-a-bulk-update/BulkCRUDRepository.test.rolls-back-a-bulk-update-when-usetransaction-is-on.typescript`

## DefaultWriteEnrichmentStrategy

- Roll back a bulk update
  - `docs/examples/roll-back-a-bulk-update/BulkCRUDRepositoryTests.BatchUpdateRollsBackWhenTransactionIsOn.csharp`
  - `docs/examples/roll-back-a-bulk-update/enriched.bulk_update_rolls_back_when_transaction_is_on.rust`
  - `docs/examples/roll-back-a-bulk-update/BulkCRUDRepository.test.rolls-back-a-bulk-update-when-usetransaction-is-on.typescript`

## LazyWriteEnrichmentStrategy

- Skip unloaded lists
  - `docs/examples/skip-unloaded-lists/UserViewCRUDRepositoryTests.LazyCreateAndUpdateSkipUnloadedCollectionsAndDeleteCascades.csharp`
  - `docs/examples/skip-unloaded-lists/user_view.lazy_write_skips_unloaded_links.rust`
  - `docs/examples/skip-unloaded-lists/UserViewCRUDRepository.test.lazy-write-skips-unloaded-links.typescript`

## UpdateEnrichmentStrategy

- Replace children
  - `docs/examples/replace-children/CRUDRepositoryTests.CascadeUpdateDeletesChildrenAndRecreates.csharp`
  - `docs/examples/replace-children/enriched.cascade_update_deletes_children_and_recreates.rust`
  - `docs/examples/replace-children/CRUDRepository.test.cascade-update-deletes-children-and-recreates.typescript`

## DefaultUpdateEnrichmentStrategy

- Replace-sync children
  - `docs/examples/replace-sync-children/CRUDRepositoryTests.UpdateReplaceSyncsChildren.csharp`
  - `docs/examples/replace-sync-children/enriched.deep_update_replace_syncs_children.rust`
  - `docs/examples/replace-sync-children/CRUDRepository.test.update-replace-syncs-children.typescript`

## CascadeDeleteUpdateEnrichmentStrategy

- Recreate children
  - `docs/examples/recreate-children/CRUDRepositoryTests.CascadeUpdateDeletesChildrenAndRecreates.csharp`
  - `docs/examples/recreate-children/enriched.cascade_update_deletes_children_and_recreates.rust`
  - `docs/examples/recreate-children/CRUDRepository.test.cascade-update-deletes-children-and-recreates.typescript`

## DeleteEnrichmentStrategy

- Skip child deletes
  - `docs/examples/skip-child-deletes/CRUDRepositoryTests.DeleteStrategyCanSkipChildren.csharp`
  - `docs/examples/skip-child-deletes/delete_enrichment_strategy.delete_strategy_can_skip_children.rust`
  - `docs/examples/skip-child-deletes/CRUDRepository.test.delete-strategy-can-skip-children.typescript`

## CascadeDeleteEnrichmentStrategy

- Cascade delete
  - `docs/examples/cascade-delete/CRUDRepositoryTests.DeleteCascadesChildrenThenParent.csharp`
  - `docs/examples/cascade-delete/enriched.deep_delete_cascades_children_then_parent.rust`
  - `docs/examples/cascade-delete/CRUDRepository.test.delete-cascades-children-then-parent.typescript`

## IBulkWriteStrategy

- One statement per row
  - `docs/examples/one-statement-per-row/BulkWriteStrategyTests.SingleCommandIBulkWriteStrategyWritesOneStatementPerRow.csharp`
  - `docs/examples/one-statement-per-row/bulk_write_strategy.single_command_i_bulk_write_strategy_writes_one_statement_per_row.rust`
  - `docs/examples/one-statement-per-row/BulkWriteStrategy.test.single-commands-writes-one-statement-per-row.typescript`

## SingleCommandIBulkWriteStrategy

- One statement per row
  - `docs/examples/one-statement-per-row/BulkWriteStrategyTests.SingleCommandIBulkWriteStrategyWritesOneStatementPerRow.csharp`
  - `docs/examples/one-statement-per-row/bulk_write_strategy.single_command_i_bulk_write_strategy_writes_one_statement_per_row.rust`
  - `docs/examples/one-statement-per-row/BulkWriteStrategy.test.single-commands-writes-one-statement-per-row.typescript`

## BatchCommandIBulkWriteStrategy

- Join statements
  - `docs/examples/join-statements/BulkWriteStrategyTests.BatchCommandIBulkWriteStrategyJoinsStatements.csharp`
  - `docs/examples/join-statements/bulk_write_strategy.batch_command_i_bulk_write_strategy_joins_statements.rust`
  - `docs/examples/join-statements/BulkWriteStrategy.test.batch-commands-joins-statements.typescript`
- Persist list children
  - `docs/examples/persist-list-children/BulkWriteStrategyTests.BatchPersistsChildrenInOneCommand.csharp`
  - `docs/examples/persist-list-children/bulk_write_strategy.batch_persists_children_in_one_command.rust`
  - `docs/examples/persist-list-children/BulkWriteStrategy.test.batch-persists-children-in-one-command.typescript`

## StoredProcedureBulkWriteStrategy

- One JSON call
  - `docs/examples/one-json-call/BulkWriteStrategyTests.StoredProcedureBulkWriteStrategySendsOneJsonCall.csharp`
  - `docs/examples/one-json-call/bulk_write_strategy.stored_procedure_bulk_write_strategy_sends_one_json_call.rust`
  - `docs/examples/one-json-call/BulkWriteStrategy.test.stored-procedure-commands-sends-one-json-call.typescript`
- Persist list children
  - `docs/examples/persist-list-children/BulkWriteStrategyTests.StoredProcedurePersistsChildrenInOneCall.csharp`
  - `docs/examples/persist-list-children/bulk_write_strategy.stored_procedure_persists_children_in_one_call.rust`
  - `docs/examples/persist-list-children/BulkWriteStrategy.test.stored-procedure-persists-children-in-one-call.typescript`

## Driver

- Register and Create
  - `docs/examples/register-and-create/RegisterAndCreateTests.RegisterAndCreate.csharp`
  - `docs/examples/register-and-create/memory-lib.register_and_create.rust`
  - `docs/examples/register-and-create/index.test.register-and-create.typescript`
- Mock driver
  - `docs/examples/mock-driver/MockDriver.MockDriver.csharp`
  - `docs/examples/mock-driver/mock_driver.DriverForMockDriver.rust`
  - `docs/examples/mock-driver/mock-driver.MockDriver.typescript`
