use std::collections::HashMap;

use crate::assets;
use crate::themes::{color_at, Colors};

pub type Cell = (u8, u8); // (cp437_code, cga_attribute)
pub type Row = Vec<Cell>;
pub type Glyph = Vec<Row>;

const TYPE_OUTLINE: u8 = 0;
const TYPE_COLOR: u8 = 2;

const HEADER: &[u8] = b"\x13TheDraw FONTS file\x1a";
const FONT_MAGIC: &[u8] = &[0x55, 0xaa, 0x00, 0xff];
const FONT_HDR: usize = 213; // 25 header bytes + 94 offsets * 2

const DEFAULT_CELL: Cell = (32, 7);

#[rustfmt::skip]
const CP437: [u32; 256] = [
    0, 9786, 9787, 9829, 9830, 9827, 9824, 8226, 9688, 9675, 9689, 9794, 9792, 9834, 9835, 9788,
    9658, 9668, 8597, 8252, 182, 167, 9644, 8616, 8593, 8595, 8594, 8592, 8735, 8596, 9650, 9660,
    // 32-126: ASCII printable, 127: house symbol
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 8962,
    // 128-255: CP437 extended
    199, 252, 233, 226, 228, 224, 229, 231, 234, 235, 232, 239, 238, 236, 196, 197,
    201, 230, 198, 244, 246, 242, 251, 249, 255, 214, 220, 162, 163, 165, 8359, 402,
    225, 237, 243, 250, 241, 209, 170, 186, 191, 8976, 172, 189, 188, 161, 171, 187,
    9617, 9618, 9619, 9474, 9508, 9569, 9570, 9558, 9557, 9571, 9553, 9559, 9565, 9564, 9563, 9488,
    9492, 9524, 9516, 9500, 9472, 9532, 9566, 9567, 9562, 9556, 9577, 9574, 9568, 9552, 9580, 9575,
    9576, 9572, 9573, 9561, 9560, 9554, 9555, 9579, 9578, 9496, 9484, 9608, 9604, 9612, 9616, 9600,
    945, 223, 915, 960, 931, 963, 181, 964, 934, 920, 937, 948, 8734, 966, 949, 8745,
    8801, 177, 8805, 8804, 8992, 8993, 247, 8776, 176, 8729, 183, 8730, 8319, 178, 9632, 160,
];

#[rustfmt::skip]
const CGA_RGB: [(u8, u8, u8); 16] = [
    (0,0,0), (0,0,170), (0,170,0), (0,170,170),
    (170,0,0), (170,0,170), (170,85,0), (170,170,170),
    (85,85,85), (85,85,255), (85,255,85), (85,255,255),
    (255,85,85), (255,85,255), (255,255,85), (255,255,255),
];

const CGA_TO_ANSI: [u8; 8] = [0, 4, 2, 6, 1, 5, 3, 7];

pub struct Font {
    pub name: String,
    font_type: u8,
    pub spacing: u8,
    data: Vec<u8>,
    base: usize,
    offsets: [u16; 94],
    cache: HashMap<usize, Glyph>,
}

fn u16le(b: &[u8], i: usize) -> u16 {
    b[i] as u16 | (b[i + 1] as u16) << 8
}

fn outline_map(c: u8) -> u8 {
    match c {
        b'A' => 205, b'B' => 196, b'C' => 179, b'D' => 186,
        b'E' => 213, b'F' => 187, b'G' => 214, b'H' => 191,
        b'I' => 200, b'J' => 190, b'K' => 192, b'L' => 189,
        b'M' => 181, b'N' => 199, b'O' | b'@' | b'&' => 32,
        _ => c,
    }
}

fn parse_glyph(data: &[u8], pos: usize, font_type: u8) -> Glyph {
    let mut pos = pos + 2; // skip width, height bytes
    let mut rows: Glyph = Vec::new();
    let mut row: Row = Vec::new();
    while pos < data.len() {
        let c = data[pos];
        if c == 0 {
            if !row.is_empty() {
                rows.push(row);
            }
            break;
        }
        if c == 13 {
            rows.push(row);
            row = Vec::new();
            pos += 1;
            continue;
        }
        if font_type == TYPE_COLOR {
            let attr = if pos + 1 < data.len() { data[pos + 1] } else { 7 };
            row.push((c, attr));
            pos += 2;
        } else {
            let ch = if font_type == TYPE_OUTLINE { outline_map(c) } else { c };
            row.push((ch, 7));
            pos += 1;
        }
    }
    rows
}

pub fn parse(data: &[u8]) -> anyhow::Result<Vec<Font>> {
    if data.len() < 20 || &data[..20] != HEADER {
        anyhow::bail!("not a TDF file");
    }
    let mut fonts = Vec::new();
    let mut i = 20;
    while i + FONT_HDR <= data.len() && &data[i..i + 4] == FONT_MAGIC {
        let nlen = (data[i + 4] as usize).min(12);
        let name: String = data[i + 5..i + 5 + nlen]
            .iter()
            .filter(|&&b| b != 0)
            .map(|&b| b as char)
            .collect();
        let font_type = data[i + 21];
        let spacing = data[i + 22];
        let block = u16le(data, i + 23) as usize;
        let base = i + FONT_HDR;
        let mut offsets = [0u16; 94];
        for k in 0..94 {
            offsets[k] = u16le(data, i + 25 + k * 2);
        }
        fonts.push(Font {
            name,
            font_type,
            spacing,
            data: data.to_vec(),
            base,
            offsets,
            cache: HashMap::new(),
        });
        i += FONT_HDR + block;
    }
    Ok(fonts)
}

pub fn get_glyph(font: &mut Font, k: usize) -> Option<Glyph> {
    let off = font.offsets[k];
    if off == 0xFFFF {
        return None;
    }
    if let Some(g) = font.cache.get(&k) {
        return Some(g.clone());
    }
    let pos = font.base + off as usize;
    let ftype = font.font_type;
    let glyph = parse_glyph(&font.data, pos, ftype);
    font.cache.insert(k, glyph.clone());
    Some(glyph)
}

pub fn render_cells(text: &str, font: &mut Font) -> Vec<Row> {
    let mut glyphs: Vec<Option<Glyph>> = Vec::new();
    for ch in text.chars() {
        if ch == ' ' {
            glyphs.push(None); // None = space
            continue;
        }
        let k = ch as i32 - 33;
        if (0..94).contains(&k) {
            if let Some(g) = get_glyph(font, k as usize) {
                if !g.is_empty() {
                    glyphs.push(Some(g));
                    continue;
                }
            }
        }
        // unknown or missing glyph: skip
    }
    if glyphs.is_empty() {
        return Vec::new();
    }
    let height = glyphs
        .iter()
        .filter_map(|g| g.as_ref())
        .map(|g| g.len())
        .max()
        .unwrap_or(0);
    if height == 0 {
        return Vec::new();
    }

    let gap = font.spacing.saturating_sub(1) as usize;
    let mut out: Vec<Row> = vec![Vec::new(); height];
    let mut first = true;

    for g in &glyphs {
        if !first && gap > 0 {
            for row in out.iter_mut() {
                row.extend(std::iter::repeat(DEFAULT_CELL).take(gap));
            }
        }
        match g {
            None => {
                for row in out.iter_mut() {
                    row.extend(std::iter::repeat(DEFAULT_CELL).take(4));
                }
            }
            Some(glyph) => {
                let w = glyph.iter().map(|r| r.len()).max().unwrap_or(0);
                for y in 0..height {
                    let src = glyph.get(y).map(|r| r.as_slice()).unwrap_or(&[]);
                    out[y].extend_from_slice(src);
                    out[y].extend(std::iter::repeat(DEFAULT_CELL).take(w - src.len()));
                }
            }
        }
        first = false;
    }
    out
}

fn is_empty(code: u8, attr: u8) -> bool {
    (attr >> 4) & 7 == 0 && (code == 0 || code == 32 || code == 255 || attr & 15 == 0)
}

fn cp437_char(code: u8) -> char {
    if code == 0 {
        ' '
    } else {
        char::from_u32(CP437[code as usize]).unwrap_or(' ')
    }
}

fn lum(idx: usize) -> f64 {
    let (r, g, b) = CGA_RGB[idx];
    0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64
}

fn sgr(attr: u8, truecolor: bool) -> String {
    let fg = (attr & 15) as usize;
    let bg = ((attr >> 4) & 7) as usize;
    if truecolor {
        let (r, g, b) = CGA_RGB[fg];
        let mut s = format!("38;2;{r};{g};{b}");
        if bg != 0 {
            let (r, g, b) = CGA_RGB[bg];
            s.push_str(&format!(";48;2;{r};{g};{b}"));
        }
        format!("\x1b[0;{s}m")
    } else {
        let fg_code = if fg < 8 {
            30 + CGA_TO_ANSI[fg] as u32
        } else {
            90 + CGA_TO_ANSI[fg - 8] as u32
        };
        let mut s = fg_code.to_string();
        if bg != 0 {
            s.push_str(&format!(";{}", 40 + CGA_TO_ANSI[bg]));
        }
        format!("\x1b[0;{s}m")
    }
}

fn tinted(cells: &[Row], colors: &Colors) -> String {
    let live: Vec<(usize, u8, u8)> = cells
        .iter()
        .flat_map(|row| row.iter().enumerate().filter(|(_, (c, a))| !is_empty(*c, *a)))
        .map(|(x, (c, a))| (x, *c, *a))
        .collect();

    if live.is_empty() {
        return cells.iter().map(|_| String::new()).collect::<Vec<_>>().join("\n");
    }

    let lo = live.iter().map(|(x, _, _)| *x).min().unwrap_or(0);
    let hi = live.iter().map(|(x, _, _)| *x).max().unwrap_or(0);
    let span = (hi - lo).max(1) as f64;

    let top = live
        .iter()
        .map(|(_, _, a)| lum((*a & 15) as usize).max(lum(((*a >> 4) & 7) as usize)))
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let shade = |base: (u8, u8, u8), l: f64| -> (u8, u8, u8) {
        let f = 0.4 + 0.6 * l / top;
        (
            (base.0 as f64 * f).round() as u8,
            (base.1 as f64 * f).round() as u8,
            (base.2 as f64 * f).round() as u8,
        )
    };

    let mut lines = Vec::new();
    for row in cells {
        let mut buf = String::new();
        let mut styled = false;
        for (x, (c, a)) in row.iter().enumerate() {
            let bg = (*a >> 4) & 7;
            if is_empty(*c, *a) {
                if styled {
                    buf.push_str("\x1b[0m");
                }
                buf.push(' ');
                styled = false;
                continue;
            }
            styled = true;
            let base = color_at(colors, (x - lo) as f64 / span);
            let (r, g, b) = shade(base, lum((*a & 15) as usize));
            let mut s = format!("38;2;{r};{g};{b}");
            if bg != 0 {
                let (r, g, b) = shade(base, lum(bg as usize));
                s.push_str(&format!(";48;2;{r};{g};{b}"));
            }
            buf.push_str(&format!("\x1b[{s}m{}", cp437_char(*c)));
        }
        lines.push(buf + "\x1b[0m");
    }
    lines.join("\n")
}

pub fn trim_cells(cells: Vec<Row>) -> Vec<Row> {
    let blank = |row: &Row| row.iter().all(|(c, a)| is_empty(*c, *a));
    let start = cells.iter().position(|r| !blank(r)).unwrap_or(0);
    let end = cells.iter().rposition(|r| !blank(r)).map(|i| i + 1).unwrap_or(0);
    if start >= end {
        return Vec::new();
    }
    cells[start..end].to_vec()
}

pub fn to_text(cells: &[Row], color: bool, truecolor: bool, tint: Option<&Colors>) -> String {
    if let Some(colors) = tint {
        return tinted(cells, colors);
    }
    let mut lines = Vec::new();
    for row in cells {
        if !color {
            let s: String = row.iter().map(|(c, _)| cp437_char(*c)).collect();
            lines.push(s.trim_end().to_string());
            continue;
        }
        let mut buf = String::new();
        let mut cur: Option<u8> = None;
        for (c, attr) in row {
            if Some(*attr) != cur {
                buf.push_str(&sgr(*attr, truecolor));
                cur = Some(*attr);
            }
            buf.push(cp437_char(*c));
        }
        buf.push_str("\x1b[0m");
        lines.push(buf);
    }
    lines.join("\n")
}

fn load_index() -> anyhow::Result<HashMap<String, String>> {
    let bytes = assets::ansi_file("index.json")
        .ok_or_else(|| anyhow::anyhow!("ansi/index.json not found in bundle"))?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn tdf_names() -> anyhow::Result<Vec<String>> {
    let mut names: Vec<String> = load_index()?.into_keys().collect();
    names.sort();
    Ok(names)
}

pub fn load_font(name: &str) -> anyhow::Result<Font> {
    let index = load_index()?;

    let key = if index.contains_key(name) {
        name.to_string()
    } else {
        let name_lower = name.to_lowercase();
        index
            .keys()
            .find(|k| k.to_lowercase() == name_lower)
            .ok_or_else(|| anyhow::anyhow!("TDF font not found: {name}"))?
            .clone()
    };

    let filename = index[&key].clone();
    let data = assets::ansi_file(&filename)
        .ok_or_else(|| anyhow::anyhow!("TDF file not found in bundle: {filename}"))?;
    let mut fonts = parse(&data)?;

    let name_lower = key.to_lowercase();
    if let Some(pos) = fonts.iter().position(|f| f.name.trim().to_lowercase() == name_lower) {
        return Ok(fonts.remove(pos));
    }
    if let Some(pos) = fonts
        .iter()
        .position(|f| name_lower.contains(&f.name.trim().to_lowercase()))
    {
        return Ok(fonts.remove(pos));
    }
    if fonts.is_empty() {
        anyhow::bail!("no fonts in TDF file {filename}");
    }
    Ok(fonts.remove(0))
}

pub fn render(
    text: &str,
    name: &str,
    color: bool,
    truecolor: bool,
    tint: Option<&Colors>,
) -> anyhow::Result<String> {
    let mut font = load_font(name)?;
    let result: String = text
        .split('\n')
        .map(|line| {
            let cells =
                trim_cells(render_cells(if line.is_empty() { " " } else { line }, &mut font));
            to_text(&cells, color, truecolor, tint)
        })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(result)
}
