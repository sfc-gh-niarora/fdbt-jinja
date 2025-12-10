# fdbt-jinja

A fork of [MiniJinja](https://github.com/mitsuhiko/minijinja) with DBT template compatibility features.

## What is this?

This fork extends MiniJinja to support DBT-specific Jinja2 features that are not present in the original implementation. It powers the [fdbt](https://github.com/sfc-gh-niarora/fdbt) fast DBT compiler.

## DBT Compatibility Features

### 1. Native `{% do %}` Statement
Execute expressions for side effects without output:
```jinja
{%- set items = [] -%}
{%- do items.append("value") -%}
```

### 2. Typed Return Values from Macros
Macros can return values with their original types (integers, lists, etc.) instead of always converting to strings:
```jinja
{%- macro get_number() -%}
  {% return 42 %}
{%- endmacro -%}

{%- for i in range(get_number()) -%}
  {{ i }}
{%- endfor -%}
```

### 3. Mutable Lists
Lists support in-place modification methods:
```jinja
{%- set fields = [] -%}
{%- do fields.append("id") -%}
{%- do fields.extend(["name", "email"]) -%}
```

### 4. Native `{% return %}` Statement
Early return from macros with type preservation:
```jinja
{%- macro get_value(x) -%}
  {%- if x > 10 -%}
    {% return x * 2 %}
  {%- endif -%}
  {% return 0 %}
{%- endmacro -%}
```

## Branch Structure

- **`dbt-features`** (default): Contains all DBT compatibility features
- **`main`**: Tracks upstream MiniJinja

## Usage

In your `Cargo.toml`:
```toml
[dependencies]
minijinja = { git = "https://github.com/sfc-gh-niarora/fdbt-jinja", branch = "dbt-features" }
```

## Testing

All new features include comprehensive unit tests:
```bash
cd minijinja
cargo test --all-features
```

## Relationship to Upstream

This fork is maintained to track upstream MiniJinja while adding DBT-specific features. Changes are:
- **Minimal**: Only what's needed for DBT compatibility
- **Tested**: All features have unit tests
- **Documented**: Clear comments explain DBT requirements

## Why a Fork?

These features are specific to DBT's template engine behavior and may not align with MiniJinja's design goals. Rather than burden the upstream project with DBT-specific concerns, we maintain this fork for the DBT tooling ecosystem.

## Credits

- Original MiniJinja by [Armin Ronacher](https://github.com/mitsuhiko)
- DBT compatibility features by the fdbt team

## License

Same as upstream MiniJinja: Apache License 2.0

