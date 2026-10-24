mod add_extra_contact;
mod as_contacts_view;
mod contact_columns;
mod contact_lookup_ids;
mod contact_lookups;
mod contact_view;
mod contact_reads;
mod contacts_join_query;
mod contacts_parity;
mod contacts_service_of;
mod contacts_view;
mod expect_contacts_view;
mod from_enriched_contacts;
mod from_mapped_contacts;
mod seed_contacts;

pub use add_extra_contact::add_extra_contact;
pub use as_contacts_view::as_contacts_view;
pub use contact_columns::{columns, contact_entity, join_entity, references, snake};
pub use contact_lookup_ids::contact_lookup_ids;
pub use contact_lookups::ContactLookups;
pub use contact_view::{AddressView, ContactView, PhoneView};
pub use contact_reads::{contact_graph, contact_reads_all, contact_reads_without_phones};
pub use contacts_join_query::contacts_join_query;
pub use contacts_parity::{
    apply_sql, contact_schema, contact_seed, contacts_parity, register_contact_dialect, statements_of,
};
pub use contacts_service_of::contacts_service_of;
pub use contacts_view::contacts_view;
pub use expect_contacts_view::{contact_with, expect_contacts_view};
pub use from_mapped_contacts::from_mapped_contacts;
pub use seed_contacts::{seed_contacts_sql, seed_identity_sql, SEED_CONTACT_COUNT};
