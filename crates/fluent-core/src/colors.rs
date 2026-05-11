use gpui::Hsla;

/// All surface, fill, foreground, and stroke colors for the theme.
///
/// Token names follow the Fluent 2 design token vocabulary where possible.
/// See <https://fluent2.microsoft.design/color> for specification.
#[derive(Clone, Debug)]
pub struct ColorScheme {
    // --- Fill backgrounds (interactive elements: buttons, inputs, etc.) ---
    /// Neutral fill — default state (Fluent: colorNeutralBackground3)
    pub neutral: Hsla,
    /// Neutral fill — pointer hover
    pub neutral_hover: Hsla,
    /// Neutral fill — disabled
    pub neutral_disabled: Hsla,
    /// Neutral fill — selected/pressed
    pub neutral_selected: Hsla,

    /// Accent fill — default state (brand colour)
    pub accent: Hsla,
    /// Accent fill — pointer hover
    pub accent_hover: Hsla,
    /// Accent fill — disabled
    pub accent_disabled: Hsla,
    /// Accent fill — selected/pressed
    pub accent_selected: Hsla,

    /// Subtle fill — default state (typically transparent)
    pub subtle: Hsla,
    /// Subtle fill — pointer hover
    pub subtle_hover: Hsla,
    /// Subtle fill — disabled
    pub subtle_disabled: Hsla,
    /// Subtle fill — selected/pressed
    pub subtle_selected: Hsla,

    /// Page / window background (Fluent: colorNeutralBackground1)
    pub surface: Hsla,
    /// Dimmed background layer behind surfaces
    pub surface_dim: Hsla,
    /// Frosted / blurred backdrop layer
    pub surface_blur_layer: Hsla,

    // --- Foreground / text colours ---
    /// Primary text on neutral fill
    pub on_neutral: Hsla,
    /// Disabled text on neutral fill
    pub on_neutral_disabled: Hsla,
    /// Text on selected neutral fill
    pub on_neutral_selected: Hsla,
    /// Brand-coloured text on neutral surface (links, icons)
    pub on_neutral_accent: Hsla,

    /// Text / icon on accent fill
    pub on_accent: Hsla,
    /// Disabled text on accent fill
    pub on_accent_disabled: Hsla,
    /// Text on selected accent fill
    pub on_accent_selected: Hsla,

    /// Secondary text on subtle fill
    pub on_subtle: Hsla,
    /// Disabled text on subtle fill
    pub on_subtle_disabled: Hsla,
    /// Text on selected subtle fill
    pub on_subtle_selected: Hsla,

    // --- Strokes / borders ---
    /// Default border / outline (Fluent: colorNeutralStroke1)
    pub stroke_neutral: Hsla,
    /// Disabled border
    pub stroke_neutral_disabled: Hsla,
    /// Secondary / dimmer border
    pub stroke_neutral_dim: Hsla,
    /// Very subtle border (dividers, separators)
    pub stroke_neutral_subtle: Hsla,
    /// Brand / accent border
    pub stroke_accent: Hsla,

    // --- Semantic status colours ---
    /// Informational status accent
    pub status_info: Hsla,
    /// Informational status background
    pub status_info_bg: Hsla,
    /// Informational status border
    pub status_info_border: Hsla,
    /// Success status accent
    pub status_success: Hsla,
    /// Success status background
    pub status_success_bg: Hsla,
    /// Success status border
    pub status_success_border: Hsla,
    /// Warning status accent
    pub status_warning: Hsla,
    /// Warning status background
    pub status_warning_bg: Hsla,
    /// Warning status border
    pub status_warning_border: Hsla,
    /// Error / danger status accent
    pub status_error: Hsla,
    /// Error / danger status background
    pub status_error_bg: Hsla,
    /// Error / danger status border
    pub status_error_border: Hsla,

    // --- Ribbon-specific ---
    /// Ribbon bar background
    pub ribbon_bg: Hsla,
    /// Active-tab fill in the ribbon tab strip
    pub ribbon_tab_active_bg: Hsla,
    /// Active-tab bottom indicator / underline accent
    pub ribbon_tab_indicator: Hsla,
    /// Vertical separator between ribbon groups
    pub ribbon_group_separator: Hsla,

    // --- Layout (dock panels, tab strip) ---
    /// Sidebar / dock panel background
    pub panel_bg: Hsla,
    /// Panel edge border
    pub panel_border: Hsla,
    /// Tab bar background (content area)
    pub tab_strip_bg: Hsla,
    /// Active tab in content tab strip
    pub tab_active_bg: Hsla,
    /// Hovered tab in content tab strip
    pub tab_hover_bg: Hsla,
}

/// Convert an RRGGBB hex literal to `Hsla` via `gpui::rgb`.
fn hex(v: u32) -> Hsla {
    gpui::rgb(v).into()
}

impl ColorScheme {
    /// Fluent 2 dark colour scheme.
    pub fn dark() -> Self {
        Self {
            // --- Fills ---
            neutral: hex(0x272727),
            neutral_hover: hex(0x303030),
            neutral_disabled: hex(0x1C1C1C),
            neutral_selected: hex(0x2D2D2D),

            // Dark mode uses a muted brand blue (#115EA3)
            accent: hex(0x115EA3),
            accent_hover: hex(0x0F548C),
            accent_disabled: hex(0x404040),
            accent_selected: hex(0x0C3B5E),

            subtle: hex(0x1C1C1C),
            subtle_hover: hex(0x252525),
            subtle_disabled: hex(0x1C1C1C),
            subtle_selected: hex(0x222222),

            surface: hex(0x1C1C1C),
            surface_dim: hex(0x141414),
            surface_blur_layer: hex(0x292929),

            // --- Foregrounds ---
            on_neutral: hex(0xFFFFFF),
            on_neutral_disabled: hex(0x5C5C5C),
            on_neutral_selected: hex(0xFFFFFF),
            on_neutral_accent: hex(0x479EF5), // brand-coloured text (#479EF5)

            on_accent: hex(0xFFFFFF),
            on_accent_disabled: hex(0x9E9E9E),
            on_accent_selected: hex(0xFFFFFF),

            on_subtle: hex(0xDEDEDE),
            on_subtle_disabled: hex(0x5C5C5C),
            on_subtle_selected: hex(0xFFFFFF),

            // --- Strokes ---
            stroke_neutral: hex(0x666666),
            stroke_neutral_disabled: hex(0x404040),
            stroke_neutral_dim: hex(0x525252),
            stroke_neutral_subtle: hex(0x383838),
            stroke_accent: hex(0x479EF5),

            // --- Semantic status ---
            status_info: hex(0x479EF5),
            status_info_bg: hex(0x082338),
            status_info_border: hex(0x115EA3),
            status_success: hex(0x54B054),
            status_success_bg: hex(0x052505),
            status_success_border: hex(0x2A7D2E),
            status_warning: hex(0xFCE100),
            status_warning_bg: hex(0x3A2E00),
            status_warning_border: hex(0x817400),
            status_error: hex(0xF85149),
            status_error_bg: hex(0x3B1010),
            status_error_border: hex(0xA4262C),

            // --- Ribbon ---
            ribbon_bg: hex(0x212121),
            ribbon_tab_active_bg: hex(0x2D2D2D),
            ribbon_tab_indicator: hex(0x479EF5),
            ribbon_group_separator: hex(0x3D3D3D),

            // --- Layout ---
            panel_bg: hex(0x1C1C1C),
            panel_border: hex(0x383838),
            tab_strip_bg: hex(0x212121),
            tab_active_bg: hex(0x2D2D2D),
            tab_hover_bg: hex(0x272727),
        }
    }

    /// Fluent 2 light colour scheme.
    pub fn light() -> Self {
        Self {
            // --- Fills ---
            neutral: hex(0xF0F0F0),
            neutral_hover: hex(0xE8E8E8),
            neutral_disabled: hex(0xF5F5F5),
            neutral_selected: hex(0xE3E3E3),

            // Light mode uses full brand blue (#0078D4)
            accent: hex(0x0078D4),
            accent_hover: hex(0x006CBF),
            accent_disabled: hex(0xD1D1D1),
            accent_selected: hex(0x005AA3),

            subtle: hex(0xFFFFFF),
            subtle_hover: hex(0xF5F5F5),
            subtle_disabled: hex(0xFAFAFA),
            subtle_selected: hex(0xEBEBEB),

            surface: hex(0xFFFFFF),
            surface_dim: hex(0xF0F0F0),
            surface_blur_layer: hex(0xFAFAFA),

            // --- Foregrounds ---
            on_neutral: hex(0x242424),
            on_neutral_disabled: hex(0xBDBDBD),
            on_neutral_selected: hex(0x242424),
            on_neutral_accent: hex(0x0078D4),

            on_accent: hex(0xFFFFFF),
            on_accent_disabled: hex(0x737373),
            on_accent_selected: hex(0xFFFFFF),

            on_subtle: hex(0x424242),
            on_subtle_disabled: hex(0xBDBDBD),
            on_subtle_selected: hex(0x242424),

            // --- Strokes ---
            stroke_neutral: hex(0xD1D1D1),
            stroke_neutral_disabled: hex(0xE0E0E0),
            stroke_neutral_dim: hex(0xC7C7C7),
            stroke_neutral_subtle: hex(0xEBEBEB),
            stroke_accent: hex(0x0078D4),

            // --- Semantic status ---
            status_info: hex(0x0078D4),
            status_info_bg: hex(0xE5F3FF),
            status_info_border: hex(0x0078D4),
            status_success: hex(0x107C10),
            status_success_bg: hex(0xEAF6EA),
            status_success_border: hex(0x107C10),
            status_warning: hex(0xF7630C),
            status_warning_bg: hex(0xFFF4CE),
            status_warning_border: hex(0xF7630C),
            status_error: hex(0xC42B1C),
            status_error_bg: hex(0xFDE7E9),
            status_error_border: hex(0xC42B1C),

            // --- Ribbon ---
            ribbon_bg: hex(0xF3F3F3),
            ribbon_tab_active_bg: hex(0xFFFFFF),
            ribbon_tab_indicator: hex(0x0078D4),
            ribbon_group_separator: hex(0xE0E0E0),

            // --- Layout ---
            panel_bg: hex(0xFAFAFA),
            panel_border: hex(0xE5E5E5),
            tab_strip_bg: hex(0xF3F3F3),
            tab_active_bg: hex(0xFFFFFF),
            tab_hover_bg: hex(0xEBEBEB),
        }
    }
}
