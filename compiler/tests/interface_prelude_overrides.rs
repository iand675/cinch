//! Source copies of family homes participate in interface dependency order.
mod common;

use cinch::interface::Iface;

#[test]
fn source_family_homes_precede_embedded_consumers_with_checked_hashes() {
    let cases: &[(&str, &[(&str, &str)], &str, &str, &str)] = &[
        (
            "category",
            &[
                ("Functor.cinch", include_str!("../prelude/Category/Functor.cinch")),
                ("Applicative.cinch", include_str!("../prelude/Category/Applicative.cinch")),
            ],
            "Category.Applicative",
            "Category.Traversable",
            "module Main where\n  import Category.Functor\n  import Category.Applicative\n  main :: Int\n  main = 0\n",
        ),
        (
            "algebra",
            &[
                ("Semigroup.cinch", include_str!("../prelude/Algebra/Semigroup.cinch")),
                ("Monoid.cinch", include_str!("../prelude/Algebra/Monoid.cinch")),
            ],
            "Algebra.Monoid",
            "Category.Foldable",
            "module Main where\n  import Algebra.Semigroup\n  import Algebra.Monoid\n  main :: Int\n  main = 0\n",
        ),
    ];
    for &(tag, sources, provider, consumer, main) in cases {
        let mut files = sources.to_vec();
        files.push(("Main.cinch", main));
        let dir = common::project(tag, &files);
        common::check(&dir, "Main.cinch").expect("check source family replacements");
        let mut interfaces: Vec<Iface> = std::fs::read_dir(dir.join("build"))
            .unwrap()
            .map(|row| row.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "cinchif"))
            .map(|path| Iface::from_json(&std::fs::read_to_string(path).unwrap()).unwrap())
            .collect();
        interfaces.sort_by(|a, b| a.module.cmp(&b.module));
        cinch::interface::verify_chain(&interfaces)
            .expect("all recorded dependency hashes are checked");
        let defining = interfaces
            .iter()
            .find(|iface| iface.module == provider)
            .unwrap();
        let consuming = interfaces
            .iter()
            .find(|iface| iface.module == consumer)
            .unwrap();
        assert!(consuming
            .imports
            .iter()
            .any(|reference| reference.module == provider
                && reference.iface_hash == defining.iface_hash()));
        assert!(
            !defining
                .imports
                .iter()
                .any(|reference| reference.module == "Prelude"),
            "a family home must not acquire a facade back-edge when copied to disk"
        );
        common::cleanup(&dir);
    }
}
