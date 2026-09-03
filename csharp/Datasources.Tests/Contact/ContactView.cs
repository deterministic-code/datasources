namespace Datasources.Tests.Contact;

internal record ContactView(
    int Id,
    string FirstName,
    string LastName,
    string? Email,
    string ContactSourceName,
    string ContactTypeName,
    AddressView[] Addresses,
    PhoneView[] Phones)
{
    public virtual bool Equals(ContactView? other) =>
        other is not null
        && Id == other.Id
        && FirstName == other.FirstName
        && LastName == other.LastName
        && Email == other.Email
        && ContactSourceName == other.ContactSourceName
        && ContactTypeName == other.ContactTypeName
        && Addresses.SequenceEqual(other.Addresses)
        && Phones.SequenceEqual(other.Phones);

    public override int GetHashCode() =>
        HashCode.Combine(Id, FirstName, LastName, Email, ContactSourceName, ContactTypeName);
}
