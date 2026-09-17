namespace Datasources.Tests.Contact;

internal sealed record ContactLookups(
    int ContactSourceId,
    int ContactTypeId,
    int AddressTypeId,
    int PhoneTypeId);
