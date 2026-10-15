//! Bundled fallback fonts for standalone operation.
//!
//! The following fonts are embedded in the binary:
//!
//! - **Carlito** — metric-compatible with Calibri (Word's default font),
//!   licensed under the SIL Open Font License 1.1
//! - **Caladea** — metric-compatible with Cambria, licensed under the Apache
//!   License 2.0
//! - **Gelasio** upright and italic variable faces, approximate substitutes for
//!   Georgia, licensed under the SIL Open Font License 1.1
//! - **Selawik** Regular and Bold, bounded Segoe UI substitutes without genuine
//!   italic faces, licensed under the SIL Open Font License 1.1
//! - **Noto Sans Symbols 2** approved monochrome bullet and checkbox subset,
//!   licensed under the SIL Open Font License 1.1
//! - **Liberation Sans** — metric-compatible with Arial, licensed under the
//!   SIL Open Font License 1.1
//! - **Liberation Serif** — metric-compatible with Times New Roman, licensed
//!   under the SIL Open Font License 1.1
//! - **Liberation Mono** — metric-compatible with Courier New, licensed under
//!   the SIL Open Font License 1.1
//! - **Noto Sans Arabic**, **Noto Sans Devanagari**, **Noto Sans Thai**,
//!   **Noto Sans SC**, **Noto Sans Hebrew**, **Noto Sans KR**, and **Noto Sans
//!   JP** — deterministic complex-script fallbacks licensed under the SIL Open
//!   Font License 1.1
//!
//! The Noto Sans SC, Hebrew, KR and JP faces are deterministic subsets, not
//! the full families. Each ships only its approved fixture repertoire, which
//! its `fonts/SUBSET-*.md` record names and reproduces.

/// Returns bundled font data: `(family_name, font_bytes)` pairs.
///
/// Returns Carlito, Caladea, Liberation Sans, Liberation Serif, and Liberation
/// Mono, each with Regular, Bold, Italic, and BoldItalic variants. Gelasio keeps
/// its original upright and italic variable faces. Selawik supplies Regular and
/// Bold. Noto Sans Symbols 2 supplies only the approved subset repertoire.
pub fn bundled_font_data() -> Vec<(&'static str, &'static [u8])> {
    vec![
        // Carlito — metric-compatible replacement for Calibri
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-Regular.ttf").as_slice(),
        ),
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-Bold.ttf").as_slice(),
        ),
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-Italic.ttf").as_slice(),
        ),
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-BoldItalic.ttf").as_slice(),
        ),
        // Caladea — metric-compatible replacement for Cambria
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-Regular.ttf").as_slice(),
        ),
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-Bold.ttf").as_slice(),
        ),
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-Italic.ttf").as_slice(),
        ),
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-BoldItalic.ttf").as_slice(),
        ),
        // Original variable faces preserve the full approved wght range.
        (
            "Gelasio",
            include_bytes!("../fonts/Gelasio-Regular.ttf").as_slice(),
        ),
        (
            "Gelasio",
            include_bytes!("../fonts/Gelasio-Italic.ttf").as_slice(),
        ),
        // Selawik has no genuine italic. Style synthesis remains provider-owned.
        (
            "Selawik",
            include_bytes!("../fonts/Selawik-Regular.ttf").as_slice(),
        ),
        (
            "Selawik",
            include_bytes!("../fonts/Selawik-Bold.ttf").as_slice(),
        ),
        (
            "Noto Sans Symbols 2",
            include_bytes!("../fonts/NotoSansSymbols2-bullets-subset.ttf").as_slice(),
        ),
        // Liberation Sans — metric-compatible replacement for Arial
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-BoldItalic.ttf").as_slice(),
        ),
        // Liberation Serif — metric-compatible replacement for Times New Roman
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-BoldItalic.ttf").as_slice(),
        ),
        // Liberation Mono — metric-compatible replacement for Courier New
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-BoldItalic.ttf").as_slice(),
        ),
        (
            "Noto Sans Arabic",
            include_bytes!("../fonts/NotoSansArabic.ttf").as_slice(),
        ),
        (
            "Noto Sans Devanagari",
            include_bytes!("../fonts/NotoSansDevanagari.ttf").as_slice(),
        ),
        (
            "Noto Sans Thai",
            include_bytes!("../fonts/NotoSansThai.ttf").as_slice(),
        ),
        (
            "Noto Sans SC",
            include_bytes!("../fonts/NotoSansSC-FX058-subset.ttf").as_slice(),
        ),
        (
            "Noto Sans Hebrew",
            include_bytes!("../fonts/NotoSansHebrew-F266a-subset.ttf").as_slice(),
        ),
        (
            "Noto Sans KR",
            include_bytes!("../fonts/NotoSansKR-F266a-subset.ttf").as_slice(),
        ),
        (
            "Noto Sans JP",
            include_bytes!("../fonts/NotoSansJP-F266a-subset.ttf").as_slice(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::bundled_font_data;
    use std::collections::BTreeSet;
    use std::path::Path;

    #[test]
    fn every_bundled_font_family_has_a_licence_file() {
        let family_licences = [
            ("Caladea", "LICENSE-Caladea"),
            ("Carlito", "LICENSE-Carlito"),
            ("Gelasio", "LICENSE-Gelasio"),
            ("Selawik", "LICENSE-Selawik"),
            ("Liberation Mono", "LICENSE-Liberation"),
            ("Liberation Sans", "LICENSE-Liberation"),
            ("Liberation Serif", "LICENSE-Liberation"),
            ("Noto Sans Arabic", "LICENSE-Noto"),
            ("Noto Sans Devanagari", "LICENSE-Noto"),
            ("Noto Sans Hebrew", "LICENSE-Noto"),
            ("Noto Sans JP", "LICENSE-Noto"),
            ("Noto Sans KR", "LICENSE-Noto"),
            ("Noto Sans SC", "LICENSE-Noto"),
            ("Noto Sans Symbols 2", "LICENSE-Noto"),
            ("Noto Sans Thai", "LICENSE-Noto"),
        ];
        let bundled_families = bundled_font_data()
            .into_iter()
            .map(|(family, _)| family)
            .collect::<BTreeSet<_>>();
        let licensed_families = family_licences
            .iter()
            .map(|(family, _)| *family)
            .collect::<BTreeSet<_>>();

        assert_eq!(bundled_families, licensed_families);

        let fonts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fonts");
        for (family, licence_file) in family_licences {
            assert!(
                fonts_dir.join(licence_file).is_file(),
                "{family} is missing {licence_file}"
            );
        }

        assert!(fonts_dir.join("NOTICE-Caladea").is_file());
        assert!(fonts_dir.join("NOTICE-Gelasio").is_file());
        assert!(fonts_dir.join("NOTICE-Selawik").is_file());
        assert!(fonts_dir.join("NOTICE-Noto").is_file());
        for subset_record in [
            "SUBSET-NotoSansSC.md",
            "SUBSET-NotoSansHebrew.md",
            "SUBSET-NotoSansKR.md",
            "SUBSET-NotoSansJP.md",
        ] {
            assert!(
                fonts_dir.join(subset_record).is_file(),
                "{subset_record} is missing"
            );
        }
    }

    #[test]
    fn deterministic_complex_script_fonts_cover_the_approved_fixture_repertoire() {
        let fixtures = [
            ("Noto Sans Arabic", "العربية"),
            ("Noto Sans Devanagari", "कि"),
            ("Noto Sans Thai", "ภาษาไทยยินดีต้อนรับ"),
            ("Noto Sans SC", "〈中〉、你好世界"),
            ("Noto Sans Hebrew", "שלום עולם"),
            ("Noto Sans KR", "안녕하세요 세계"),
            ("Noto Sans JP", "こんにちは、カタカナ世界"),
        ];
        for (family, text) in fixtures {
            let bytes = bundled_font_data()
                .into_iter()
                .find_map(|(candidate, bytes)| (candidate == family).then_some(bytes))
                .expect("approved family is bundled");
            let face = ttf_parser::Face::parse(bytes, 0).expect("bundled font parses");
            assert!(
                text.chars()
                    .filter(|character| !character.is_whitespace())
                    .all(|character| face.glyph_index(character).is_some()),
                "{family} misses its approved fixture repertoire"
            );
        }
    }

    // Base runs supply the authenticated retained inputs. After asset adoption,
    // ordinary tests read the same original bytes from the package's fonts directory.
    fn fx210_approved_face(tracked_name: &str, retained_name: &str) -> Vec<u8> {
        let path = match std::env::var_os("F_X210_APPROVED_FONT_INPUTS") {
            Some(directory) => std::path::PathBuf::from(directory).join(retained_name),
            None => Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("fonts")
                .join(tracked_name),
        };
        std::fs::read(&path).unwrap_or_else(|error| {
            panic!(
                "F-X210 font input precondition failed at {}: {error}",
                path.display()
            )
        })
    }

    #[test]
    fn fx210_georgia_uses_pinned_gelasio_faces_and_selected_styles() {
        let upright = fx210_approved_face("Gelasio-Regular.ttf", "Gelasio[wght].ttf");
        let italic = fx210_approved_face("Gelasio-Italic.ttf", "Gelasio-Italic[wght].ttf");
        for (bytes, is_italic) in [(&upright, false), (&italic, true)] {
            let face = ttf_parser::Face::parse(bytes, 0).expect("approved Gelasio input parses");
            assert_eq!(face.units_per_em(), 2048, "Gelasio input precondition");
            assert_eq!(face.is_italic(), is_italic, "Gelasio input precondition");
            let axes: Vec<_> = face.variation_axes().into_iter().collect();
            assert_eq!(axes.len(), 1, "Gelasio input precondition");
            assert_eq!(axes[0].tag, ttf_parser::Tag::from_bytes(b"wght"));
            assert_eq!(
                (axes[0].min_value, axes[0].def_value, axes[0].max_value),
                (400.0, 400.0, 700.0)
            );
            assert!(
                (32_u8..=126).all(|code| face.glyph_index(char::from(code)).is_some()),
                "Gelasio input ASCII coverage precondition"
            );
        }
        let mut manager = crate::FontManager::new_deterministic().expect("bundled manager loads");
        for (bold, is_italic) in [(false, false), (true, false), (false, true), (true, true)] {
            let expected = if is_italic { &italic } else { &upright };
            let id = manager
                .resolve_font(Some("Georgia"), bold, is_italic)
                .expect("Georgia resolves");
            let selected = manager.font_data(id).expect("selected Georgia data exists");
            assert_eq!(
                selected.family, "Gelasio",
                "F-X210 Georgia must select its approved bundled substitute"
            );
            assert_eq!(
                selected.data.as_ref(),
                expected.as_slice(),
                "Georgia must retain the pinned original face bytes"
            );
            assert_eq!(selected.face_index, 0);
            assert_eq!((selected.bold, selected.italic), (bold, is_italic));
            assert!(!selected.synthetic_bold && !selected.synthetic_italic);
            let coordinates = if bold {
                vec![(*b"wght", 700.0)]
            } else {
                Vec::new()
            };
            assert_eq!(selected.variations, coordinates);
            let mut reference =
                ttf_parser::Face::parse(expected, 0).expect("original reference parses");
            if bold {
                reference
                    .set_variation(ttf_parser::Tag::from_bytes(b"wght"), 700.0)
                    .expect("original weight axis exists");
            }
            let glyph = reference.glyph_index('A').expect("original face covers A");
            let advance = f64::from(
                reference
                    .glyph_hor_advance(glyph)
                    .expect("original advance exists"),
            ) * 12.0
                / f64::from(reference.units_per_em());
            let shaped = manager
                .shape_text(id, "A", 12.0)
                .expect("selected face shapes A");
            assert_eq!(shaped.glyph_ids, vec![glyph.0]);
            assert_eq!(shaped.advances.len(), 1);
            assert!(
                (shaped.width - advance).abs() < 1e-6,
                "selected style advance must match original selected-instance hmtx"
            );
            assert!((shaped.advances[0] - advance).abs() < 1e-6);
        }
    }

    #[test]
    fn fx210_segoe_ui_uses_pinned_selawik_faces_without_inventing_italic() {
        let regular = fx210_approved_face("Selawik-Regular.ttf", "selawk.ttf");
        let bold_face = fx210_approved_face("Selawik-Bold.ttf", "selawkb.ttf");
        for (bytes, weight) in [(&regular, 400), (&bold_face, 700)] {
            let face = ttf_parser::Face::parse(bytes, 0).expect("approved Selawik input parses");
            assert_eq!(face.units_per_em(), 2048, "Selawik input precondition");
            assert_eq!(
                face.weight().to_number(),
                weight,
                "Selawik input precondition"
            );
            assert!(
                !face.is_italic() && !face.is_variable(),
                "Selawik has no genuine italic or variable face"
            );
            assert!(
                (32_u8..=126).all(|code| face.glyph_index(char::from(code)).is_some()),
                "Selawik input ASCII coverage precondition"
            );
        }
        let mut manager = crate::FontManager::new_deterministic().expect("bundled manager loads");
        for (bold, italic) in [(false, false), (true, false), (false, true), (true, true)] {
            let expected = if bold { &bold_face } else { &regular };
            let id = manager
                .resolve_font(Some("Segoe UI"), bold, italic)
                .expect("Segoe UI resolves");
            let selected = manager
                .font_data(id)
                .expect("selected Segoe UI data exists");
            assert_eq!(
                selected.family, "Selawik",
                "F-X210 Segoe UI must select its approved bundled substitute"
            );
            assert_eq!(
                selected.data.as_ref(),
                expected.as_slice(),
                "Segoe UI must retain the pinned original face bytes"
            );
            assert_eq!(selected.face_index, 0);
            assert_eq!((selected.bold, selected.italic), (bold, italic));
            assert!(!selected.synthetic_bold);
            assert_eq!(selected.synthetic_italic, italic);
            assert!(selected.variations.is_empty());
            let reference =
                ttf_parser::Face::parse(expected, 0).expect("original reference parses");
            let glyph = reference.glyph_index('A').expect("original face covers A");
            let advance = f64::from(
                reference
                    .glyph_hor_advance(glyph)
                    .expect("original advance exists"),
            ) * 12.0
                / f64::from(reference.units_per_em());
            let shaped = manager
                .shape_text(id, "A", 12.0)
                .expect("selected face shapes A");
            assert_eq!(shaped.glyph_ids, vec![glyph.0]);
            assert_eq!(shaped.advances.len(), 1);
            assert!(
                (shaped.width - advance).abs() < 1e-6,
                "Selawik synthesis must preserve original source advance"
            );
            assert!((shaped.advances[0] - advance).abs() < 1e-6);
        }
    }

    #[test]
    fn fx210_new_bundled_substitutes_do_not_override_caller_labelled_styles() {
        // Existing Caladea faces deliberately carry caller labels. This checks
        // byte ownership and CSS style ranking, not authentic Georgia metrics.
        let caller_faces: Vec<_> = bundled_font_data()
            .into_iter()
            .filter(|(family, _)| *family == "Caladea")
            .map(|(_, bytes)| bytes)
            .collect();
        assert_eq!(caller_faces.len(), 4);
        let mut inputs = Vec::new();
        for family in ["Georgia", "Segoe UI"] {
            for bytes in &caller_faces {
                inputs.push(crate::FontFile {
                    family: family.to_owned(),
                    data: bytes.to_vec(),
                });
            }
        }
        let mut manager = crate::FontManager::new_deterministic().unwrap();
        assert!(manager.load_additional_fonts(&inputs));
        for family in ["Georgia", "Segoe UI"] {
            for (bold, italic) in [(false, false), (true, false), (false, true), (true, true)] {
                let expected = caller_faces
                    .iter()
                    .find(|bytes| {
                        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
                        face.is_bold() == bold && face.is_italic() == italic
                    })
                    .expect("four genuine caller styles exist");
                let id = manager.resolve_font(Some(family), bold, italic).unwrap();
                let selected = manager.font_data(id).unwrap();
                assert_eq!(selected.family, "Caladea");
                assert_eq!(selected.data.as_ref(), *expected);
                assert_eq!(selected.face_index, 0);
                assert_eq!((selected.bold, selected.italic), (bold, italic));
                assert!(!selected.synthetic_bold && !selected.synthetic_italic);
                assert!(selected.variations.is_empty());
                let repeat = manager.resolve_font(Some(family), bold, italic).unwrap();
                assert_eq!(repeat, id, "warm selection retains exact caller identity");
                let repeated = manager.font_data(repeat).unwrap();
                assert_eq!(repeated.data.as_ref(), selected.data.as_ref());
                assert_eq!(repeated.family, selected.family);
                assert_eq!(repeated.face_index, selected.face_index);
                assert_eq!(
                    (repeated.bold, repeated.italic),
                    (selected.bold, selected.italic)
                );
                assert_eq!(repeated.variations, selected.variations);
                assert_eq!(
                    (repeated.synthetic_bold, repeated.synthetic_italic),
                    (selected.synthetic_bold, selected.synthetic_italic)
                );
            }
        }
    }

    #[test]
    fn fx210_selawik_regular_retains_bounded_ascii_and_word_metric_contract() {
        let bytes = include_bytes!("../fonts/Selawik-Regular.ttf");
        let reference = ttf_parser::Face::parse(bytes, 0).unwrap();
        let mut manager = crate::FontManager::new_deterministic().unwrap();
        let id = manager
            .resolve_font(Some("Segoe UI"), false, false)
            .unwrap();
        assert_eq!(
            manager.font_data(id).unwrap().data.as_ref(),
            bytes.as_slice()
        );
        for code in 32_u8..=126 {
            let character = char::from(code);
            let glyph = reference
                .glyph_index(character)
                .expect("approved ASCII is covered");
            let expected = f64::from(reference.glyph_hor_advance(glyph).unwrap());
            let shaped = manager
                .shape_text(id, &character.to_string(), 2048.0)
                .unwrap();
            assert_eq!(shaped.glyph_ids, vec![glyph.0]);
            assert_eq!(shaped.advances, vec![expected]);
            assert_eq!(shaped.width, expected);
        }
        // Authenticated Selawik1.01 and private SegoeUI5.31 share these Windows
        // units. This does not assert kerning, generic hhea or Bold parity.
        let word = manager.word_line_metrics(id, 2048.0).unwrap();
        assert_eq!(word.units_per_em, 2048);
        assert_eq!(
            (word.ascent, word.descent, word.line_gap),
            (2210.0, 514.0, 0.0)
        );
        let generic = manager.metrics(id, 2048.0).unwrap();
        assert_eq!(generic.ascent, f64::from(reference.ascender()));
        assert_eq!(generic.descent, -f64::from(reference.descender()));
        assert_eq!(generic.line_gap, f64::from(reference.line_gap()));
    }

    #[test]
    fn fx210_approved_symbol_subset_covers_only_its_seventeen_semantic_inputs() {
        let bytes = include_bytes!("../fonts/NotoSansSymbols2-bullets-subset.ttf");
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let characters = [
            '\u{21e8}', '\u{25c6}', '\u{25fb}', '\u{2610}', '\u{2611}', '\u{2612}', '\u{2713}',
            '\u{2717}', '\u{274d}', '\u{2751}', '\u{2752}', '\u{2756}', '\u{2794}', '\u{27a2}',
            '\u{29eb}', '\u{2b25}', '\u{2b27}',
        ];
        assert_eq!(face.units_per_em(), 1000);
        for tag in [b"COLR", b"CBDT", b"SVG "] {
            assert!(
                face.raw_face()
                    .table(ttf_parser::Tag::from_bytes(tag))
                    .is_none(),
                "approved semantic fallback stays monochrome"
            );
        }
        assert!(face.glyph_index('A').is_none());
        assert!(
            face.glyph_index('\u{f0a7}').is_none(),
            "the subset is Unicode coverage, not a rewritten symbol cmap"
        );
        let mut manager = crate::FontManager::new_deterministic().unwrap();
        let id = manager
            .resolve_font(Some("Noto Sans Symbols 2"), false, false)
            .unwrap();
        let selected = manager.font_data(id).unwrap();
        assert_eq!(selected.family, "Noto Sans Symbols 2");
        assert_eq!(selected.data.as_ref(), bytes.as_slice());
        assert!(!selected.synthetic_bold && !selected.synthetic_italic);
        assert!(selected.variations.is_empty());
        for character in characters {
            let glyph = face
                .glyph_index(character)
                .expect("all seventeen semantic inputs are covered");
            assert_ne!(glyph.0, 0);
            let shaped = manager
                .shape_text(id, &character.to_string(), 1000.0)
                .unwrap();
            assert_eq!(shaped.glyph_ids, vec![glyph.0]);
            assert_eq!(
                shaped.advances,
                vec![f64::from(face.glyph_hor_advance(glyph).unwrap())]
            );
        }
    }
}
