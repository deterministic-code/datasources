using Datasources;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class MysqlCallsTests
{
    [Fact]
    public void RewritesInsertUpdateAndDelete()
    {
        var insert = new InsertStatement("`tbl_Contacts`", [new Parameter("m_FirstName", "Ada"), new Parameter("LastName", "Lovelace")]);
        var inserted = MysqlCalls.Insert("add_contact");
        Assert.Equal(
            "CALL add_contact($m_FirstName, $LastName)",
            DefaultStatementTextRenderer.Instance.Render(inserted.Map(insert)));
        Assert.Same(insert, inserted.ReverseMap(insert));

        var where = new Where(new Datasources.Operator.Equals("Id", 1));
        var update = new UpdateStatement("`tbl_Contacts`", [new Parameter("m_FirstName", "Ada")], where);
        var updated = MysqlCalls.Update("update_contact");
        Assert.StartsWith(
            "CALL update_contact(",
            DefaultStatementTextRenderer.Instance.Render(updated.Map(update)),
            StringComparison.Ordinal);
        Assert.Same(update, updated.ReverseMap(update));

        var delete = new DeleteStatement("`tbl_ContactGroupMember`", new Where(new Datasources.Operator.And(
            new Datasources.Operator.Equals("ContactId", 1),
            new Datasources.Operator.Equals("ContactGroupId", 2))));
        var deleted = MysqlCalls.Delete("delete_contact_group_member");
        Assert.Equal(
            "CALL delete_contact_group_member($ContactId, $ContactGroupId)",
            DefaultStatementTextRenderer.Instance.Render(deleted.Map(delete)));
        Assert.Same(delete, deleted.ReverseMap(delete));

        var bare = MysqlCalls.Delete("delete_contact").Map(new DeleteStatement("`tbl_Contacts`"));
        Assert.Equal("CALL delete_contact()", DefaultStatementTextRenderer.Instance.Render(bare));
    }
}
