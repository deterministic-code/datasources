use datasources::named_stub_connection;

named_stub_connection!(DynamoDbConnection, DynamoDbDriver, "dynamodb");
