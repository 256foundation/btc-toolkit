# Icon Assets

## Source

All icons in this directory are from **[Heroicons](https://github.com/tailwindlabs/heroicons)** by Tailwind Labs.

- **License**: MIT License
- **Repository**: https://github.com/tailwindlabs/heroicons
- **Version**: 24x24 outline icons (optimized)
- **Style**: Outline stroke icons with 1.5 stroke-width

## Current Icons

| File             | Icon Name            | Usage                  |
| ---------------- | -------------------- | ---------------------- |
| `arrow-left.svg` | Arrow Left           | Back navigation        |
| `cog.svg`        | Cog (Settings)       | Settings/configuration |
| `refresh.svg`    | Arrow Path           | Refresh/reload         |
| `check.svg`      | Check Circle         | Success status         |
| `warning.svg`    | Exclamation Triangle | Warning status         |
| `error.svg`      | X Circle             | Error status           |
| `add.svg`        | Plus Circle          | Add/create actions     |
| `play.svg`       | Play                 | Start/play actions     |
| `stop.svg`       | Stop                 | Stop actions           |
| `network.svg`    | Globe Alt            | Network/internet       |

## Adding New Icons

To add more icons from Heroicons:

### 1. Find the icon you need

Browse the icon catalog:

- **Website**: https://heroicons.com/
- **GitHub**: https://github.com/tailwindlabs/heroicons/tree/master/optimized/24/outline

### 2. Download the SVG file

Use curl to download the optimized outline (24x24) version:

```bash
cd assets/icons
curl -sL "https://raw.githubusercontent.com/tailwindlabs/heroicons/master/optimized/24/outline/ICON-NAME.svg" -o ICON-NAME.svg
```

**Example:**

```bash
# Download the "trash" icon
curl -sL "https://raw.githubusercontent.com/tailwindlabs/heroicons/master/optimized/24/outline/trash.svg" -o trash.svg
```

### 3. Add to src/theme/icons.rs

After downloading, add the icon to the theme:

```rust
// 1. Add constant for the embedded SVG
pub const TRASH: &[u8] = include_bytes!("../../assets/icons/trash.svg");

// 2. (Optional) Add convenience function
pub fn trash() -> Svg<'static> {
    icon(TRASH)
}
```

### 4. Use in your UI

```rust
use crate::theme;

// Using the convenience function
theme::icons::trash()

// Or using the constant directly
theme::icons::icon(theme::icons::TRASH)
```

## Icon Naming Convention

Heroicons uses kebab-case naming (e.g., `arrow-left`, `x-circle`, `cog-6-tooth`).

**Common conversions:**

- `arrow-left` → `ARROW_LEFT` (constant), `arrow-left.svg` (file)
- `cog-6-tooth` → `COG` (simplified for common use)
- `exclamation-triangle` → `WARNING` (semantic naming)

## Icon Sizes

The icon helper functions provide three size variants:

- `icon(data)` - Standard 20px (ICON_SIZE)
- `icon_sm(data)` - Small 16px (ICON_SIZE_SM)
- `icon_lg(data)` - Large 24px (ICON_SIZE_LG)
- `icon_size(data, size)` - Custom size

## Finding Icon Names

### Method 1: Browse the website

Visit https://heroicons.com/ and search visually

### Method 2: List all available icons

```bash
curl -s "https://api.github.com/repos/tailwindlabs/heroicons/contents/optimized/24/outline" | \
  grep '"name"' | \
  cut -d'"' -f4 | \
  sort
```

### Method 3: Search in the repository

https://github.com/tailwindlabs/heroicons/tree/master/optimized/24/outline

## Alternative Icon Styles

If you need different styles, Heroicons also provides:

- **24x24 solid**: `optimized/24/solid/ICON-NAME.svg`
- **20x20 solid**: `optimized/20/solid/ICON-NAME.svg`
- **16x16 solid**: `optimized/16/solid/ICON-NAME.svg`

To use solid icons, change the download URL:

```bash
curl -sL "https://raw.githubusercontent.com/tailwindlabs/heroicons/master/optimized/24/solid/ICON-NAME.svg" -o ICON-NAME-solid.svg
```
