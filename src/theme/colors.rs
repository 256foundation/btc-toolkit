use iced::Color;

/// Industrial color palette optimized for mining operations and technical interfaces
/// Inspired by industrial control systems and modern dark themes

// Base Colors - Dark industrial backgrounds
pub const BACKGROUND_BASE: Color = Color::from_rgb(0.11, 0.12, 0.13); // #1C1F21
pub const BACKGROUND_ELEVATED: Color = Color::from_rgb(0.15, 0.16, 0.18); // #26292D
pub const BACKGROUND_CARD: Color = Color::from_rgb(0.18, 0.19, 0.21); // #2E3135
pub const BACKGROUND_INPUT: Color = Color::from_rgb(0.14, 0.15, 0.16); // #232628

// Surface overlays
pub const SURFACE_OVERLAY_10: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.02);
pub const SURFACE_OVERLAY_20: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.05);
pub const SURFACE_OVERLAY_30: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.08);

// Primary - Industrial Blue (for primary actions)
pub const PRIMARY: Color = Color::from_rgb(0.0, 0.48, 0.73); // #007ABA - Strong industrial blue
pub const PRIMARY_HOVER: Color = Color::from_rgb(0.0, 0.56, 0.82); // #008FD2
pub const PRIMARY_ACTIVE: Color = Color::from_rgb(0.0, 0.40, 0.62); // #00669E

// Accent - Bright Cyan (for highlights)
pub const ACCENT: Color = Color::from_rgb(0.0, 0.73, 0.83); // #00BAD4
pub const ACCENT_HOVER: Color = Color::from_rgb(0.0, 0.82, 0.92); // #00D1EA
pub const ACCENT_DIM: Color = Color::from_rgba(0.0, 0.73, 0.83, 0.3);

// Status Colors - Industrial standards
pub const SUCCESS: Color = Color::from_rgb(0.0, 0.8, 0.4); // #00CC66 - Bright green
pub const SUCCESS_DIM: Color = Color::from_rgba(0.0, 0.8, 0.4, 0.15);
pub const WARNING: Color = Color::from_rgb(1.0, 0.65, 0.0); // #FFA500 - Industrial orange
pub const WARNING_DIM: Color = Color::from_rgba(1.0, 0.65, 0.0, 0.15);
pub const DANGER: Color = Color::from_rgb(0.95, 0.26, 0.21); // #F24236 - Alarm red
pub const DANGER_DIM: Color = Color::from_rgba(0.95, 0.26, 0.21, 0.15);
pub const CRITICAL: Color = Color::from_rgb(0.9, 0.1, 0.1); // #E61A1A - Critical red

// Text Colors
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.92, 0.93, 0.94); // #EBEDEE
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.72, 0.74); // #B2B8BC
pub const TEXT_TERTIARY: Color = Color::from_rgb(0.5, 0.52, 0.54); // #808689
pub const TEXT_DISABLED: Color = Color::from_rgba(0.7, 0.72, 0.74, 0.4);
pub const TEXT_ON_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0); // White

// Border Colors
pub const BORDER_SUBTLE: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.06);
pub const BORDER_DEFAULT: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.12);
pub const BORDER_STRONG: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.18);
pub const BORDER_FOCUS: Color = PRIMARY;

// Data Visualization (for hashrate, temp, etc.)
pub const DATA_BLUE: Color = Color::from_rgb(0.25, 0.62, 0.90); // #3F9FE6
pub const DATA_CYAN: Color = Color::from_rgb(0.0, 0.82, 0.92); // #00D1EA
pub const DATA_GREEN: Color = Color::from_rgb(0.18, 0.80, 0.44); // #2DCC70
pub const DATA_YELLOW: Color = Color::from_rgb(0.95, 0.77, 0.06); // #F2C410
pub const DATA_ORANGE: Color = Color::from_rgb(0.90, 0.49, 0.13); // #E67D21
pub const DATA_RED: Color = Color::from_rgb(0.90, 0.29, 0.24); // #E64A3D

// Mining Status Colors
pub const MINING_ACTIVE: Color = Color::from_rgb(0.0, 0.8, 0.4); // Bright green
pub const MINING_IDLE: Color = Color::from_rgb(0.6, 0.62, 0.64); // Gray
pub const HASHBOARD_TEMP_NORMAL: Color = DATA_BLUE;
pub const HASHBOARD_TEMP_WARM: Color = DATA_YELLOW;
pub const HASHBOARD_TEMP_HOT: Color = DATA_ORANGE;
pub const HASHBOARD_TEMP_CRITICAL: Color = DATA_RED;

// Chip health gradient
pub const CHIP_FULL: Color = SUCCESS;
pub const CHIP_GOOD: Color = DATA_GREEN;
pub const CHIP_FAIR: Color = DATA_YELLOW;
pub const CHIP_POOR: Color = DATA_ORANGE;
pub const CHIP_CRITICAL: Color = DANGER;

// Shadow colors for depth
pub const SHADOW_LIGHT: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.15);
pub const SHADOW_MEDIUM: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.25);
pub const SHADOW_HEAVY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.40);
