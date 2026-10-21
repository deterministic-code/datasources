[assembly: CollectionBehavior(DisableTestParallelization = true)]

namespace Datasources.Sqlite.Tests;

[CollectionDefinition("DatasourceFactory", DisableParallelization = true)]
public class DatasourceFactoryCollection;
