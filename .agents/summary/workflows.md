# Workflows
<!-- metadata: type=workflows, audience=ai-agents, scope=user-flows -->

## Application Startup

```mermaid
sequenceDiagram
    participant main as main()
    participant i18n as gettext
    participant res as GResource
    participant app as MundiApplication
    participant win as MundiWindow

    main->>i18n: setlocale, bindtextdomain, textdomain
    main->>res: resources_register_include!
    main->>app: MundiApplication::new()
    main->>app: app.run()
    app->>app: startup(): setup_actions, load CSS
    app->>win: activate(): MundiWindow::new()
    win->>win: constructed(): populate_countries()
    win->>win: load_window_state()
    win->>win: present()
```

Key points:
1. i18n is initialized before any GTK code runs
2. GResource bundle is registered once at startup
3. CSS provider is loaded in `startup()` (runs once per app lifetime)
4. `activate()` creates the window (runs each time the app is activated)

## Discovery Mode

```mermaid
sequenceDiagram
    participant user as User
    participant mev as MapExerciseView
    participant mw as MapWidget

    user->>mw: Click on map
    mw->>mw: hit_test(x, y)
    mw-->>mev: signal "region-clicked"(id)
    mev->>mev: quiz_active == false
    mev->>mev: find_region_name(id)
    mev->>mev: discovery_label.set_text(name)
```

In discovery mode (default when entering an exercise), clicking any region shows its translated name. For capitals exercises, the label shows "Capital, capital of Region".

## Quiz Lifecycle

```mermaid
sequenceDiagram
    participant user as User
    participant mev as MapExerciseView
    participant quiz as Quiz
    participant mw as MapWidget
    participant sp as SoundPlayer
    participant qrv as QuizResultsView

    user->>mev: Click "Start Quiz"
    mev->>quiz: Quiz::new(regions, alternates)
    mev->>mw: reset_all_states()
    mev->>sp: play_music()
    mev->>mev: update_quiz_ui() → show prompt

    loop For each question
        user->>mw: Click region
        mw-->>mev: "region-clicked"(id)
        mev->>quiz: quiz.answer(region_id)
        alt Correct
            mev->>mw: set_region_state(target, Correct)
            mev->>sp: play_correct()
        else Wrong (attempts remaining)
            mev->>mw: set_region_state(id, Wrong)
            mev->>sp: play_wrong()
            Note over mw: Flash red 500ms then Normal
        else Wrong (no attempts left)
            mev->>mw: set_region_state(target, Wrong)
            mev->>sp: play_wrong()
            Note over mw: Target stays red permanently
        end
        mev->>mev: update_quiz_ui()
    end

    mev->>sp: stop_music()
    mev->>mev: save_stats(correct, total)
    mev->>mw: set_visible(false)
    mev->>qrv: show_results(correct, total, time, country, exercise)
```

### Quiz Scoring Detail

- Each question starts with 3 attempts
- Correct on 1st attempt: +3 points
- Correct on 2nd attempt: +2 points
- Correct on 3rd attempt: +1 point
- All attempts exhausted: +0 points, target region turns red permanently
- Total possible = `num_regions × 3`
- Percentage = `session_correct / session_total × 100`

### Timer

- Starts when quiz begins (`Instant::now()`)
- Updates every second via `glib::timeout_add_local`
- Displayed as `M:SS` in the header bar
- Stopped when quiz finishes; elapsed time passed to results view

## Leaderboard Flow

```mermaid
sequenceDiagram
    participant qrv as QuizResultsView
    participant lb as Leaderboard
    participant fs as Filesystem

    qrv->>lb: Leaderboard::load(country, exercise)
    lb->>fs: Read JSON from XDG_DATA_HOME
    lb-->>qrv: leaderboard
    qrv->>lb: qualifies(score, total, time)?
    alt Qualifies
        qrv->>qrv: Show name entry
        Note over qrv: User enters name, clicks Save
        qrv->>lb: insert(entry) → rank
        qrv->>lb: save(country, exercise)
        lb->>fs: Write JSON
        qrv->>qrv: populate_leaderboard(highlight: rank)
    else Does not qualify
        qrv->>qrv: Show "didn't make top 50"
    end
    qrv->>qrv: populate_leaderboard(highlight: None)
```

## Adding a New Country/Exercise

1. **Create SVG map**: `resources/maps/{country}/{exercise}.svg`
   - Each clickable region: `<path id="RegionName" d="M ... L ... Z"/>`
   - Decorative elements: `<path id="__border" d="..."/>`
   - Path data: absolute `M L Z` only (no curves, no transforms)

2. **Add region names**: In `src/region_names.rs`, add a new static slice:
   ```rust
   pub static COUNTRY_REGIONS: &[(&str, &str)] = &[
       ("SvgId", N_("Translated Name")),
       // ...
   ];
   ```

3. **Register exercise**: In `src/registry.rs`:
   - Create a `static COUNTRY_EXERCISES: &[MapExercise]` array
   - Add a `Country` entry to the `COUNTRIES` slice in `countries()`

4. **Register GResource**: Add the SVG path to `resources/resources.gresource.xml`

5. **Update translations**: Add `src/region_names.rs` to `po/POTFILES.in` (already listed), then regenerate the `.pot` file

## Adding a Capitals Exercise

Same as above, plus:

1. **SVG format**: Background outlines use `_bg_` prefix IDs; clickable dots are 1×1 `<path>` squares with capital name as ID
2. **Region names**: Tuples are `(capital_name, region_name)` instead of `(svg_id, region_name)`
3. **Exercise kind**: Set `kind: ExerciseKind::Capitals`
4. **Dual capitals**: Use `alternates: &[("AltCapital", "PrimaryCapital")]`

## Build Workflow

### Development

```bash
glib-compile-schemas data/          # Compile GSettings schema
GSETTINGS_SCHEMA_DIR=data cargo run # Run with local schema
```

### Production (Meson)

```bash
meson setup builddir
meson compile -C builddir
# Installs: binary, schemas, desktop file, icon, translations, metainfo
```

### CI Pipeline

```mermaid
graph LR
    A[checkout] --> B[apt install deps]
    B --> C[Setup Rust stable]
    C --> D[cargo fmt --check]
    D --> E[cargo clippy -- -D warnings]
    E --> F[cargo build]
    F --> G[cargo test]
    G --> H[meson setup + compile]
```

Runs on push/PR to `main`. Checks formatting, lints, builds, tests, and verifies Meson build.

## Release Workflow

1. Update version in `Cargo.toml` and `meson.build`
2. Add release entry in `data/io.github.nacho.mundi.metainfo.xml`
3. Run `cargo fmt` and `cargo update -p mundi`
4. Commit: `git commit -am "Release X.Y.Z"`
5. Tag: `git tag vX.Y.Z`
6. Push: `git push && git push --tags`
