use rand::Rng;

pub type Rgb = (u8, u8, u8);
pub type Colors = Vec<Rgb>;

pub static THEME_NAMES: &[&str] = &[
    "catppuccin", "dracula", "gruvbox", "monokai", "nord", "one-dark",
    "solarized", "synthwave", "synthwave-84", "tokyo-night",
];

static THEMES: &[(&str, &[&str])] = &[
    ("monokai",      &["F92672","FD971F","E6DB74","A6E22E","66D9EF","AE81FF"]),
    ("dracula",      &["FF79C6","BD93F9","8BE9FD","50FA7B","F1FA8C","FFB86C","FF5555"]),
    ("solarized",    &["B58900","CB4B16","DC322F","D33682","6C71C4","268BD2","2AA198","859900"]),
    ("nord",         &["BF616A","D08770","EBCB8B","A3BE8C","88C0D0","81A1C1","5E81AC","B48EAD"]),
    ("gruvbox",      &["FB4934","FE8019","FABD2F","B8BB26","8EC07C","83A598","D3869B"]),
    ("one-dark",     &["E06C75","E5C07B","98C379","56B6C2","61AFEF","C678DD"]),
    ("tokyo-night",  &["F7768E","FF9E64","E0AF68","9ECE6A","73DACA","7DCFFF","7AA2F7","BB9AF7"]),
    ("catppuccin",   &["F38BA8","FAB387","F9E2AF","A6E3A1","94E2D5","74C7EC","89B4FA","CBA6F7","F5C2E7"]),
    ("synthwave",    &["FF2975","F222FF","8C1EFF","3D7BFF","2DE2E6"]),
    ("synthwave-84", &["FE4450","FF7EDB","FEDE5D","72F1B8","36F9F6"]),
];

fn from_hex(h: &str) -> Rgb {
    let v = u32::from_str_radix(h, 16).unwrap_or(0);
    (((v >> 16) & 0xFF) as u8, ((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8)
}

pub fn pick_colors(theme: &str) -> Colors {
    let mut rng = rand::thread_rng();
    let resolved = if theme == "random" {
        THEMES[rng.gen_range(0..THEMES.len())].0
    } else {
        theme
    };
    let stops: Vec<Rgb> = THEMES
        .iter()
        .find(|(name, _)| *name == resolved)
        .map(|(_, hexes)| hexes.iter().map(|h| from_hex(h)).collect())
        .unwrap_or_else(|| vec![(255, 255, 255)]);

    let n = stops.len();
    let start = if n > 2 { rng.gen_range(0..=(n - 3)) } else { 0 };
    let end = (start + 3).min(n);
    let mut colors = stops[start..end].to_vec();
    if rng.gen_bool(0.5) {
        colors.reverse();
    }
    colors
}

pub fn color_at(colors: &Colors, t: f64) -> Rgb {
    let t = t.clamp(0.0, 1.0) * (colors.len() - 1) as f64;
    let k = (t as usize).min(colors.len() - 2);
    let f = t - k as f64;
    let (r0, g0, b0) = colors[k];
    let (r1, g1, b1) = colors[k + 1];
    (
        (r0 as f64 + (r1 as f64 - r0 as f64) * f).round() as u8,
        (g0 as f64 + (g1 as f64 - g0 as f64) * f).round() as u8,
        (b0 as f64 + (b1 as f64 - b0 as f64) * f).round() as u8,
    )
}
