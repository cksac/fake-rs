# Changelogs

## v4.3.0 (2025-04-04)
- Fix address street name template for locales IT_IT, PT_PT and DE_DE
- Add company constants for PT_PT locale

## v4.2.0 (2025-03-21)
- Add IT_IT (italian) locale (@campeis [#224](https://github.com/cksac/fake-rs/pull/224))
- Fix rust_decimal scale exceeds maximum supported scale in v1.37

## v4.1.0 (2025-03-21, yanked)

## v4.0.0 (2025-02-16)
## v3.2.0 (2025-02-14, yanked)
- Add more french translation(@odarrouzet [#213](https://github.com/cksac/fake-rs/pull/213))
- Add de_de, pt_pt locale support in fake cli
- Enable Link-Time Optimization
- Add base64 faker (@xd009642 [#216](https://github.com/cksac/fake-rs/pull/216))
- Implement Dummy for std::time::Duration (@patrickariel [#217](https://github.com/cksac/fake-rs/pull/217))
- Add Portuguese (Portugal) locale support (@dgsantana [#218](https://github.com/cksac/fake-rs/pull/218))
- Implement Dummy for EmailAddress newtype (@patrickariel [#219](https://github.com/cksac/fake-rs/pull/219))
- Use explicit traits for slices in dummy derive (@patrickariel [#220](https://github.com/cksac/fake-rs/pull/220))
- Update rand to 0.9.0 (@thomas-tribus [#221](https://github.com/cksac/fake-rs/pull/221))

## v3.1.0 (2024-12-27)
- Allow CityName implementation to be overrite by each locale
- Added DE_DE Locale (@xoryouyou [#75](https://github.com/cksac/fake-rs/pull/75))
- Added fake cli (@akhildevelops [#209](https://github.com/cksac/fake-rs/pull/209))
- Implement dummy for uuid v6, v7 (@LNSD [#207](https://github.com/cksac/fake-rs/pull/207))
- Add Precision::<N> support for chrono::NaiveDateTime (@m4tx [#204](https://github.com/cksac/fake-rs/pull/204))
