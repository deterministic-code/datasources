namespace Datasources.Tests.Contact;

internal sealed record ContactView(
    int id,
    string first_name,
    string last_name,
    string? email,
    string contact_source_name,
    string contact_type_name,
    AddressView[] addresses,
    PhoneView[] phones)
{
    public bool Equals(ContactView? other) =>
        other is not null
        && id == other.id
        && first_name == other.first_name
        && last_name == other.last_name
        && email == other.email
        && contact_source_name == other.contact_source_name
        && contact_type_name == other.contact_type_name
        && addresses.SequenceEqual(other.addresses)
        && phones.SequenceEqual(other.phones);

    public override int GetHashCode() => id;
}
