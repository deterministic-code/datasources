mod flat_to_nested_mapper;
mod i_flat_to_nested_mapper;
mod identity_mapper;
mod mapped_value;
mod row_type_mapper;
mod transform_converter_mapper;
mod transform_mapper;
mod type_map;
mod type_map_field;
mod type_map_group;

pub use flat_to_nested_mapper::FlatToNestedMapper;
pub use i_flat_to_nested_mapper::FlatToNestedMapper as FlatToNestedMapperTrait;
pub use identity_mapper::IdentityMapper;
pub use mapped_value::{MappedRow, MappedValue};
pub use row_type_mapper::RowTypeMapper;
pub use transform_converter_mapper::TransformConverterMapper;
pub use transform_mapper::TransformMapper;
pub use type_map::TypeMap;
pub use type_map_field::TypeMapField;
pub use type_map_group::TypeMapGroup;

pub trait Mapper<TFrom, TTo> {
    fn map(&self, value: TFrom) -> TTo;
    fn reverse_map(&self, value: TTo) -> TFrom;
}

impl<TFrom, TTo> Mapper<TFrom, TTo> for Box<dyn Mapper<TFrom, TTo> + Send + Sync> {
    fn map(&self, value: TFrom) -> TTo {
        (**self).map(value)
    }

    fn reverse_map(&self, value: TTo) -> TFrom {
        (**self).reverse_map(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct User {
        id: i32,
        name: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Contacts {
        email: String,
        phone: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Nested {
        user: User,
        contacts: Contacts,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Flat {
        user_id: i32,
        user_name: String,
        contact_email: String,
        contact_phone: String,
    }

    #[test]
    fn flattens_and_unflattens_user_and_contacts() {
        let nested = Nested {
            user: User {
                id: 1,
                name: "Ada".into(),
            },
            contacts: Contacts {
                email: "ada@x.dev".into(),
                phone: "555".into(),
            },
        };
        let mapper = TransformMapper::new(
            |n: Nested| Flat {
                user_id: n.user.id,
                user_name: n.user.name,
                contact_email: n.contacts.email,
                contact_phone: n.contacts.phone,
            },
            |f: Flat| Nested {
                user: User {
                    id: f.user_id,
                    name: f.user_name,
                },
                contacts: Contacts {
                    email: f.contact_email,
                    phone: f.contact_phone,
                },
            },
        );
        let flat = Mapper::map(&mapper, nested.clone());
        assert_eq!(flat.user_id, 1);
        assert_eq!(flat.user_name, "Ada");
        assert_eq!(flat.contact_email, "ada@x.dev");
        assert_eq!(flat.contact_phone, "555");
        assert_eq!(Mapper::reverse_map(&mapper, flat), nested);
    }
}
