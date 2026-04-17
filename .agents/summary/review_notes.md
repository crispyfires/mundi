# Review Notes
<!-- metadata: type=review, audience=ai-agents, scope=quality -->

## Consistency Check

### ✅ Passed

- **Version consistency**: 0.9.0 referenced consistently across codebase_info.md, data_models.md (build-time config example), and matches `Cargo.toml` and `meson.build`
- **Component names**: All GTK4 subclass names (`MundiApplication`, `MundiWindow`, `MapExerciseView`, `MapWidget`, `QuizResultsView`, `PreferencesDialog`) used consistently across architecture.md, components.md, and interfaces.md
- **Signal documentation**: `region-clicked` and `retry` signals documented identically in interfaces.md and components.md
- **GSettings schema**: Keys and paths consistent between interfaces.md, data_models.md, and architecture.md persistence table
- **SVG conventions**: ID prefix rules (`__`, `_bg_`, `_river_`) documented consistently in architecture.md and data_models.md
- **Dependency versions**: All crate versions in dependencies.md match `Cargo.toml`
- **File references**: Source file paths in components.md metadata comments match actual codebase structure
- **Exercise kind behavior**: `ExerciseKind::Standard` vs `Capitals` differences documented consistently in data_models.md, workflows.md, and components.md
- **Scoring formula**: `session_correct += attempts_left` documented identically in data_models.md and workflows.md

### ⚠️ Minor Notes

- **codebase_info.md** lists "12 files" in `src/` but `config.rs.in` is a template, not a compiled source file. The 12 `.rs` files in `src/` are: `main.rs`, `application.rs`, `window.rs`, `registry.rs`, `region_names.rs`, `map_widget.rs`, `map_exercise_view.rs`, `quiz.rs`, `quiz_results_view.rs`, `leaderboard.rs`, `sound_player.rs`, `preferences_dialog.rs`. Plus `config.rs.in` as a template = 13 entries in the source files table. This is accurate since the table explicitly notes `config.rs.in` as a template.

## Completeness Check

### ✅ Well-Covered Areas

- Application architecture and navigation model
- All GTK4 subclass components with responsibilities and template children
- GObject signals and properties
- GSettings schema (full hierarchy including relocatable stats)
- Data models (static, runtime, persistent)
- Quiz lifecycle and scoring mechanics
- SVG conventions and rendering pipeline
- Build system (both Cargo and Meson paths)
- CI pipeline and release workflow
- Adding new countries/exercises workflow
- Dependency catalog with purposes

### 📋 Areas With Limited Coverage

1. **Unit tests**: The codebase has no visible test files or test modules. The CI runs `cargo test` but there appear to be no tests defined. This is a gap in the codebase itself, not the documentation.

2. **Accessibility**: The UI templates use standard Adwaita widgets which provide baseline accessibility, but there's no documentation of specific accessibility considerations (e.g., screen reader support for the map widget, keyboard navigation for quiz interaction). The custom `MapWidget` likely has limited accessibility since it's a pure rendering widget with click/motion handlers.

3. **Error handling patterns**: The codebase uses `unwrap()` and `expect()` in several places (SVG parsing, GSettings access, resource loading). The documentation doesn't call out the error handling strategy or where panics are acceptable vs. where graceful degradation would be preferred.

4. **Translation workflow details**: The documentation covers the i18n architecture but doesn't detail the practical workflow for translators (how to create a new `.po` file, how to test translations, how to submit translations).

5. **Map creation tooling**: The SVG conventions are well-documented, but there's no guidance on tools or techniques for creating compliant SVG maps from source data (e.g., converting from standard SVG maps to the simple `M L Z` format).

6. **LOCALEDIR handling**: In development mode (`cargo run`), `LOCALEDIR` defaults to `/usr/share/locale` which won't find translations. The development workflow section mentions `GSETTINGS_SCHEMA_DIR` but not how to test with translations locally.

## Recommendations

1. **Add tests**: Consider adding unit tests for `Quiz` (pure logic, easy to test) and `Leaderboard` (serialization round-trips, ranking logic).

2. **Document accessibility limitations**: Note that `MapWidget` is a custom rendering widget without built-in accessibility tree support. Consider adding `ATK`/`AccessibleRole` annotations in the future.

3. **Translation guide**: Add a brief section to workflows.md or a separate `TRANSLATING.md` covering how to add a new language.

4. **Map creation guide**: Consider documenting the SVG simplification process (tools like Inkscape path operations, removing transforms, converting curves to line segments).
