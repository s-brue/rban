#!/usr/bin/env python3
"""Render rban's colored output to PNG images (used for the README).

    python3 tools/screenshots.py            # rebuilds assets/*.png
    python3 tools/screenshots.py hero.png   # rebuild one image

Each image is a fake terminal window: a prompt line with the command, then rban's real output.
"""
import re
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "assets"
BINARY = ROOT / "target" / "release" / "rban"
FONT = next((p for p in (
    "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    "/Library/Fonts/DejaVuSansMono.ttf",
) if Path(p).exists()), None)

THEMES = ["monokai", "dracula", "solarized", "nord", "gruvbox", "one-dark", "tokyo-night", "catppuccin",
          "synthwave", "synthwave-84"]
SHOTS = {
    "hero.png": (1, [
        ["Hello", "-f", "Coder Purple", "--name"],
        ["Hello", "-f", "Valley Cyan", "--name"],
        ["Hello", "-f", "Minds Green", "--name"],
    ]),
    "ansi-fonts.png": (1, [
        ["Hello", "-f", "Stargate Cya", "--name"],
        ["rban",  "-f", "Brave Red", "--name"],
        ["rban",  "-f", "Neutral Cyan", "--name"],
    ]),
    "ansi-gradient.png": (2, [
        ["rban", "-f", "Coder Purple"],         ["rban", "-f", "Coder Purple", "--theme", "monokai"],
        ["rban", "-f", "Valley Cyan"],          ["rban", "-f", "Valley Cyan", "--theme", "dracula"],
        ["rban", "-f", "Amazon Cyan"],          ["rban", "-f", "Amazon Cyan", "--theme", "tokyo-night"],
    ]),
    "themes.png": (2, [["rban", "-f", "ansi_compact", "--theme", t] for t in THEMES]),
    "figlet.png": (1, [
        ["rban", "-f", "ANSI Shadow", "--theme", "dracula", "--name"],
        ["rban", "-f", "isometric1", "--name"],
    ]),
}

# Terminal colors: Catppuccin Mocha
BG, FG = (24, 25, 33), (205, 214, 244)
PALETTE = [(69, 71, 90), (243, 139, 168), (166, 227, 161), (249, 226, 175), (137, 180, 250), (245, 194, 231),
           (148, 226, 213), (186, 194, 222), (88, 91, 112), (243, 139, 168), (166, 227, 161), (249, 226, 175),
           (137, 180, 250), (245, 194, 231), (148, 226, 213), (205, 214, 244)]
CW, CH, PAD, BAR = 10, 20, 22, 34

SGR = re.compile(r"\x1b\[([0-9;]*)m")


def parse(text):
    """ANSI string -> rows of (char, fg, bg) cells."""
    rows, row, fg, bg, pos = [], [], None, None, 0
    for m in list(SGR.finditer(text)) + [None]:
        chunk = text[pos:(m.start() if m else len(text))]
        for ch in chunk:
            if ch == "\n":
                rows.append(row)
                row = []
            elif ch != "\r":
                row.append((ch, fg, bg))
        if m is None:
            break
        pos = m.end()
        codes = [int(c) if c else 0 for c in m.group(1).split(";")]
        i = 0
        while i < len(codes):
            c = codes[i]
            if c == 0:
                fg = bg = None
            elif 30 <= c <= 37:
                fg = PALETTE[c - 30]
            elif 90 <= c <= 97:
                fg = PALETTE[c - 90 + 8]
            elif 40 <= c <= 47:
                bg = PALETTE[c - 40]
            elif c == 39:
                fg = None
            elif c == 49:
                bg = None
            elif c in (38, 48) and codes[i + 1:i + 2] == [2]:
                rgb = tuple(codes[i + 2:i + 5])
                fg, bg = (rgb, bg) if c == 38 else (fg, rgb)
                i += 4
            i += 1
    rows.append(row)
    return rows


def blend(a, b, t):
    return tuple(round(a[k] + (b[k] - a[k]) * t) for k in range(3))


def draw_cell(d, font, x, y, ch, fg, bg):
    fg, bg = fg or FG, bg or BG
    if bg != BG:
        d.rectangle([x, y, x + CW - 1, y + CH - 1], fill=bg)
    half_w, half_h = CW // 2, CH // 2
    blocks = {"█": (0, 0, CW, CH), "▀": (0, 0, CW, half_h), "▄": (0, half_h, CW, CH),
              "▌": (0, 0, half_w, CH), "▐": (half_w, 0, CW, CH)}
    shades = {"░": .25, "▒": .5, "▓": .75}
    if ch in blocks:
        x0, y0, x1, y1 = blocks[ch]
        d.rectangle([x + x0, y + y0, x + x1 - 1, y + y1 - 1], fill=fg)
    elif ch in shades:
        d.rectangle([x, y, x + CW - 1, y + CH - 1], fill=blend(bg, fg, shades[ch]))
    elif ch != " ":
        d.text((x, y + 1), ch, font=font, fill=fg)


def trim(rows):
    blank = lambda r: all(c[0] == " " and c[2] is None for c in r)
    while rows and blank(rows[0]):
        rows = rows[1:]
    while rows and blank(rows[-1]):
        rows = rows[:-1]
    return rows


def render_image(commands, columns=1):
    font = ImageFont.truetype(FONT, 16)
    blocks = [(prompt, trim(parse(out.rstrip("\n")))) for prompt, out in commands]
    grid = [blocks[i:i + columns] for i in range(0, len(blocks), columns)]
    gap = 3
    col_w = [max(max(len(p) + 2, max((len(r) for r in rows), default=0)) for p, rows in
                 (g[c] for g in grid if c < len(g))) for c in range(columns)]
    row_h = [max(len(rows) + 3 for _, rows in g) for g in grid]
    w = (sum(col_w) + gap * (columns - 1)) * CW + PAD * 2
    h = sum(row_h) * CH + PAD * 2 + BAR
    img = Image.new("RGB", (w, h), BG)
    d = ImageDraw.Draw(img)
    d.rectangle([0, 0, w, BAR], fill=(17, 17, 27))
    for i, c in enumerate(((255, 95, 86), (255, 189, 46), (39, 201, 63))):
        d.ellipse([16 + i * 22, 11, 28 + i * 22, 23], fill=c)
    y = BAR + PAD
    for g, rh in zip(grid, row_h):
        x0 = PAD
        for c, (prompt, rows) in enumerate(g):
            d.text((x0, y + 1), "$", font=font, fill=PALETTE[2])
            d.text((x0 + 2 * CW, y + 1), prompt, font=font, fill=FG)
            for r, row in enumerate(rows):
                for x, (ch, fg, bg) in enumerate(row):
                    draw_cell(d, font, x0 + x * CW, y + (r + 1) * CH, ch, fg, bg)
            x0 += (col_w[c] + gap) * CW
        y += rh * CH
    return img


def run(args):
    cmd = [str(BINARY), *args, "--color", "--truecolor"]
    out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
    prompt = "rban " + " ".join(f'"{a}"' if " " in a else a for a in args)
    return prompt, out


def main():
    if FONT is None:
        sys.exit("DejaVu Sans Mono not found; install it or edit FONT in this script")
    if not BINARY.exists():
        sys.exit(f"Binary not found at {BINARY}. Run: cargo build --release")
    OUT.mkdir(exist_ok=True)
    only = sys.argv[1:]
    for name, (columns, shots) in SHOTS.items():
        if only and name not in only:
            continue
        render_image([run(a) for a in shots], columns).save(OUT / name, optimize=True)
        print("wrote", OUT / name)


if __name__ == "__main__":
    main()
