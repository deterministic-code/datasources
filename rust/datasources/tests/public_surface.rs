#[test]
fn providers_export_only_their_provider() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../providers");
    let mut found = 0;
    for entry in std::fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path().join("src/lib.rs");
        if !path.is_file() {
            continue;
        }
        found += 1;
        let source = std::fs::read_to_string(&path).unwrap();
        let public: Vec<_> = source
            .lines()
            .map(str::trim)
            .filter(|line| {
                line.starts_with("pub ")
                    || (line.starts_with("pub(")
                        && !line.starts_with("pub(crate)")
                        && !line.starts_with("pub(super)"))
            })
            .collect();
        assert_eq!(public.len(), 1, "{}: {public:?}", path.display());
        let line = public[0].trim_end_matches(';');
        assert!(
            line.starts_with("pub use ") && line.ends_with("Driver"),
            "{}: {line}",
            path.display()
        );
    }
    assert!(found >= 11, "expected every provider crate, found {found}");
}
