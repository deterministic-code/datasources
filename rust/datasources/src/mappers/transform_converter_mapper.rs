use crate::converters::TypeConverter;
use crate::mappers::IMapper;

pub struct TransformConverterMapper<To, From = To> {
    converter: Box<dyn TypeConverter<To, From> + Send + Sync>,
    on_map: Option<(
        Box<dyn Fn(&From) -> bool + Send + Sync>,
        Box<dyn Fn(From) -> To + Send + Sync>,
    )>,
    on_reverse: Option<(
        Box<dyn Fn(&To) -> bool + Send + Sync>,
        Box<dyn Fn(To) -> From + Send + Sync>,
    )>,
}

impl<To, From> TransformConverterMapper<To, From> {
    pub fn new(converter: impl TypeConverter<To, From> + Send + Sync + 'static) -> Self {
        Self {
            converter: Box::new(converter),
            on_map: None,
            on_reverse: None,
        }
    }
}

impl<T: 'static> TransformConverterMapper<T, T> {
    pub fn with_filter(
        converter: impl TypeConverter<T, T> + Send + Sync + 'static,
        filter: impl Fn(&T) -> bool + Send + Sync + 'static,
    ) -> Self {
        let filter = std::sync::Arc::new(filter);
        let map_filter = filter.clone();
        Self {
            converter: Box::new(converter),
            on_map: Some((Box::new(move |value| map_filter(value)), Box::new(|value| value))),
            on_reverse: Some((Box::new(move |value| filter(value)), Box::new(|value| value))),
        }
    }
}

impl<To, From> IMapper<From, To> for TransformConverterMapper<To, From> {
    fn map(&self, value: From) -> To {
        match &self.on_map {
            Some((filter, skip)) if !filter(&value) => skip(value),
            _ => self.converter.convert(&value).expect("convert"),
        }
    }

    fn reverse_map(&self, value: To) -> From {
        match &self.on_reverse {
            Some((filter, skip)) if !filter(&value) => skip(value),
            _ => self.converter.reverse_convert(&value).expect("convert"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converters::TypeConverter;
    use crate::datasource_error::DatasourceError;
    use crate::result::Result;

    struct Wrap;

    impl TypeConverter<String, String> for Wrap {
        fn convert(&self, from: &String) -> Result<String> {
            Ok(format!("[{from}]"))
        }

        fn reverse_convert(&self, to: &String) -> Result<String> {
            Ok(to.trim_matches(['[', ']']).to_string())
        }
    }

    #[test]
    fn converts_both_ways_without_filter() {
        let mapper = TransformConverterMapper::new(Wrap);
        assert_eq!(IMapper::map(&mapper, "ab".into()), "[ab]");
        assert_eq!(IMapper::reverse_map(&mapper, "[ab]".into()), "ab");
    }

    #[test]
    fn converts_when_filter_is_true() {
        let mapper = TransformConverterMapper::with_filter(Wrap, |s| s.contains("ab"));
        assert_eq!(IMapper::map(&mapper, "ab".into()), "[ab]");
        assert_eq!(IMapper::reverse_map(&mapper, "[ab]".into()), "ab");
    }

    #[test]
    fn is_identity_when_filter_is_false() {
        let mapper = TransformConverterMapper::with_filter(Wrap, |s| s.contains("ab"));
        assert_eq!(IMapper::map(&mapper, "a".into()), "a");
        assert_eq!(IMapper::reverse_map(&mapper, "[a]".into()), "[a]");
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Address {
        street: String,
        city: String,
        country: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Contact {
        email: String,
        phone: String,
        address: Address,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct User {
        username: String,
        display_name: String,
        contact: Contact,
    }

    struct UserFromCells;

    impl TypeConverter<User, Vec<String>> for UserFromCells {
        fn convert(&self, from: &Vec<String>) -> Result<User> {
            match from.as_slice() {
                [username, display_name, email, phone, street, city, country] => Ok(User {
                    username: username.clone(),
                    display_name: display_name.clone(),
                    contact: Contact {
                        email: email.clone(),
                        phone: phone.clone(),
                        address: Address {
                            street: street.clone(),
                            city: city.clone(),
                            country: country.clone(),
                        },
                    },
                }),
                _ => Err(DatasourceError::message("expected 7 cells")),
            }
        }

        fn reverse_convert(&self, to: &User) -> Result<Vec<String>> {
            Ok(vec![
                to.username.clone(),
                to.display_name.clone(),
                to.contact.email.clone(),
                to.contact.phone.clone(),
                to.contact.address.street.clone(),
                to.contact.address.city.clone(),
                to.contact.address.country.clone(),
            ])
        }
    }

    #[test]
    fn expands_flat_cells_into_nested_user() {
        let cells = vec![
            "ada".into(),
            "Ada Lovelace".into(),
            "ada@example.com".into(),
            "+44 20 7946 0958".into(),
            "10 Downing St".into(),
            "London".into(),
            "UK".into(),
        ];
        let expected = User {
            username: "ada".into(),
            display_name: "Ada Lovelace".into(),
            contact: Contact {
                email: "ada@example.com".into(),
                phone: "+44 20 7946 0958".into(),
                address: Address {
                    street: "10 Downing St".into(),
                    city: "London".into(),
                    country: "UK".into(),
                },
            },
        };
        let mapper = TransformConverterMapper::new(UserFromCells);
        assert_eq!(IMapper::map(&mapper, cells.clone()), expected);
        assert_eq!(IMapper::reverse_map(&mapper, expected), cells);
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Account {
        id: i32,
        username: String,
        display_name: String,
        is_enabled: bool,
        created: chrono::DateTime<chrono::Utc>,
        contact: Contact,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Payload {
        Flat(Vec<String>),
        Nested(Account),
    }

    struct AccountFromCells;

    impl TypeConverter<Payload, Payload> for AccountFromCells {
        fn convert(&self, from: &Payload) -> Result<Payload> {
            match from {
                Payload::Flat(cells) => match cells.as_slice() {
                    [id, username, display_name, enabled, created, email, phone, street, city, _] => {
                        Ok(Payload::Nested(Account {
                            id: id.parse().expect("id"),
                            username: username.clone(),
                            display_name: display_name.clone(),
                            is_enabled: enabled == "1",
                            created: chrono::DateTime::parse_from_rfc3339(created)
                                .expect("created")
                                .with_timezone(&chrono::Utc),
                            contact: Contact {
                                email: email.clone(),
                                phone: phone.clone(),
                                address: Address {
                                    street: street.clone(),
                                    city: city.clone(),
                                    country: String::new(),
                                },
                            },
                        }))
                    }
                    _ => Err(DatasourceError::message("expected 10 cells")),
                },
                nested => Ok(nested.clone()),
            }
        }

        fn reverse_convert(&self, to: &Payload) -> Result<Payload> {
            Ok(match to {
                Payload::Nested(account) => Payload::Flat(vec![
                    account.id.to_string(),
                    account.username.clone(),
                    account.display_name.clone(),
                    if account.is_enabled { "1" } else { "0" }.into(),
                    account.created.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                    account.contact.email.clone(),
                    account.contact.phone.clone(),
                    account.contact.address.street.clone(),
                    account.contact.address.city.clone(),
                    String::new(),
                ]),
                flat => flat.clone(),
            })
        }
    }

    fn is_enabled(payload: &Payload) -> bool {
        match payload {
            Payload::Flat(cells) => cells.get(3).is_some_and(|enabled| enabled == "1"),
            Payload::Nested(account) => account.is_enabled,
        }
    }

    #[test]
    fn converts_types_and_filters_rows() {
        let ada = Payload::Flat(vec![
            "7".into(),
            "ada".into(),
            "Ada Lovelace".into(),
            "1".into(),
            "2020-01-02T03:04:05.000Z".into(),
            "ada@example.com".into(),
            "+44".into(),
            "10 Downing St".into(),
            "London".into(),
            "secret-note".into(),
        ]);
        let guest = Payload::Flat(vec![
            "0".into(),
            "guest".into(),
            "Guest".into(),
            "0".into(),
            "2020-01-01T00:00:00.000Z".into(),
            "guest@example.com".into(),
            "".into(),
            "1 Road".into(),
            "Paris".into(),
            "ignore-me".into(),
        ]);
        let expected = Payload::Nested(Account {
            id: 7,
            username: "ada".into(),
            display_name: "Ada Lovelace".into(),
            is_enabled: true,
            created: chrono::DateTime::parse_from_rfc3339("2020-01-02T03:04:05.000Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            contact: Contact {
                email: "ada@example.com".into(),
                phone: "+44".into(),
                address: Address {
                    street: "10 Downing St".into(),
                    city: "London".into(),
                    country: String::new(),
                },
            },
        });
        let mapper = TransformConverterMapper::with_filter(AccountFromCells, is_enabled);
        assert_eq!(IMapper::map(&mapper, ada), expected);
        assert_eq!(
            IMapper::reverse_map(&mapper, expected),
            Payload::Flat(vec![
                "7".into(),
                "ada".into(),
                "Ada Lovelace".into(),
                "1".into(),
                "2020-01-02T03:04:05.000Z".into(),
                "ada@example.com".into(),
                "+44".into(),
                "10 Downing St".into(),
                "London".into(),
                "".into(),
            ])
        );
        assert_eq!(IMapper::map(&mapper, guest.clone()), guest);
        assert_eq!(IMapper::reverse_map(&mapper, guest.clone()), guest);
    }
}
