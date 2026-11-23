mod assets;
mod tdf;
mod themes;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Cursor;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use is_terminal::IsTerminal;
use rand::Rng;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const CREDITS: &str = include_str!("../CREDITS.md");
const LICENSE: &str = include_str!("../LICENSE");

const DEPS_LICENSES: &str = "\
  anyhow 1.0  (MIT OR Apache-2.0)\n\
    https://github.com/dtolnay/anyhow\n\
  clap 4.6  (MIT OR Apache-2.0)\n\
    https://github.com/clap-rs/clap\n\
  figdriver 0.6  (MIT)\n\
    https://github.com/cmatsuoka/figdriver-rs\n\
  is-terminal 0.4  (MIT)\n\
    https://github.com/sunfishcode/is-terminal\n\
  rand 0.8  (MIT OR Apache-2.0)\n\
    https://github.com/rust-random/rand\n\
  rust-embed 8  (MIT)\n\
    https://github.com/pyros2097/rust-embed\n\
  serde_json 1.0  (MIT OR Apache-2.0)\n\
    https://github.com/serde-rs/json";

#[derive(Parser)]
#[command(name = "rban", version = VERSION, about = "Print text as ASCII/ANSI art")]
struct Args {
    /// Text to print
    text: Option<String>,

    #[command(flatten)]
    mode: ModeArgs,

    #[command(flatten)]
    filter: FilterArgs,

    /// List TDF (ANSI) font names and exit
    #[arg(long)]
    list_ansi: bool,

    /// List FIGlet font names and exit
    #[arg(long)]
    list_figlet: bool,

    /// List gradient themes and exit
    #[arg(long)]
    list_themes: bool,

    /// Print font credits and exit
    #[arg(long)]
    credits: bool,

    /// Print the software license and exit
    #[arg(long)]
    license: bool,

    /// Exact CGA colors as RGB instead of terminal palette
    #[arg(long)]
    truecolor: bool,

    /// Gradient color theme (implies --gradient)
    #[arg(long, value_name = "THEME")]
    theme: Option<String>,

    /// Paint any font with a theme gradient
    #[arg(long)]
    gradient: bool,

    /// Print [font-name] after each banner
    #[arg(long)]
    name: bool,

    /// Force color even when not writing to a terminal
    #[arg(long)]
    color: bool,

    /// Never use color
    #[arg(long)]
    no_color: bool,

    /// Font list config file
    #[arg(long, value_name = "PATH")]
    conf: Option<String>,

    /// Max FIGlet render width (default: 120)
    #[arg(long, short, default_value_t = 120)]
    width: usize,
}

#[derive(clap::Args)]
#[group(multiple = false)]
struct ModeArgs {
    /// A random font
    #[arg(long)]
    rand: bool,

    /// All fonts
    #[arg(long)]
    all: bool,

    /// All fonts, piped through a color-aware pager
    #[arg(long)]
    all_pager: bool,

    /// A specific font (FIGlet or TDF)
    #[arg(long, short, value_name = "NAME")]
    font: Option<String>,
}

#[derive(clap::Args)]
#[group(multiple = false)]
struct FilterArgs {
    /// Only colored TDF (ANSI) fonts
    #[arg(long)]
    ansi_only: bool,

    /// Only FIGlet (ASCII) fonts
    #[arg(long)]
    ascii_only: bool,
}

// ── fonts.conf ────────────────────────────────────────────────────────────────

fn find_conf(explicit: Option<&str>) -> Option<String> {
    if let Some(p) = explicit {
        return std::fs::read_to_string(p).ok();
    }
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config")
        });
    let user = xdg.join("rban").join("fonts.conf");
    if let Ok(s) = std::fs::read_to_string(&user) {
        return Some(s);
    }
    let sys = PathBuf::from("/etc/rban/fonts.conf");
    if let Ok(s) = std::fs::read_to_string(sys) {
        return Some(s);
    }
    // Fall back to the bundled fonts.conf embedded in the binary
    assets::bundled_conf()
}

fn load_groups(content: &str, all_tdf: &[String]) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    let mut section = String::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].to_lowercase();
        } else if !section.is_empty() {
            groups.entry(section.clone()).or_default().push(line.to_string());
        }
    }
    if groups.get("ansi").map(|v| v.iter().any(|s| s == "*")).unwrap_or(false) {
        groups.insert("ansi".to_string(), all_tdf.to_vec());
    }
    groups
}

// ── FIGlet font index (from embedded ascii/ bundle) ───────────────────────────

fn norm(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

fn build_fig_index() -> HashMap<String, String> {
    // Map norm(stem) -> filename inside the ascii/ bundle
    assets::ascii_files()
        .filter(|name| name.ends_with(".flf") || name.ends_with(".tlf"))
        .map(|name| {
            let stem = name
                .rsplit_once('.')
                .map(|(s, _)| s)
                .unwrap_or(&name)
                .to_string();
            (norm(&stem), name.into_owned())
        })
        .collect()
}

// ── random pick with persistent bag ──────────────────────────────────────────

fn cache_path() -> PathBuf {
    let base = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cache")
        });
    base.join("rban.json")
}

fn pick_random(group: &str, pool: &[String]) -> String {
    let path = cache_path();
    let mut seen: HashMap<String, Vec<String>> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .and_then(|v: serde_json::Value| {
            v["seen"].as_object().map(|o| {
                o.iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            v.as_array()
                                .map(|a| {
                                    a.iter()
                                        .filter_map(|x| x.as_str().map(String::from))
                                        .collect()
                                })
                                .unwrap_or_default(),
                        )
                    })
                    .collect()
            })
        })
        .unwrap_or_default();

    let used: Vec<String> = seen
        .get(group)
        .map(|v| v.iter().filter(|f| pool.contains(f)).cloned().collect())
        .unwrap_or_default();
    let mut left: Vec<&String> = pool.iter().filter(|f| !used.contains(f)).collect();

    if left.is_empty() {
        let last = used.last().cloned();
        left = pool.iter().filter(|f| Some(*f) != last.as_ref()).collect();
        if left.is_empty() {
            left = pool.iter().collect();
        }
    }

    let mut rng = rand::thread_rng();
    let choice = left[rng.gen_range(0..left.len())].clone();
    let mut new_used = used;
    new_used.push(choice.clone());
    seen.insert(group.to_string(), new_used);

    let _ = std::fs::create_dir_all(path.parent().unwrap_or(std::path::Path::new(".")));
    let _ = std::fs::write(
        &path,
        serde_json::to_string(&serde_json::json!({"seen": seen})).unwrap_or_default(),
    );

    choice
}

// ── rendering ─────────────────────────────────────────────────────────────────

fn trim_blank_edges(lines: Vec<String>) -> Vec<String> {
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map(|i| i + 1)
        .unwrap_or(0);
    if start >= end {
        return Vec::new();
    }
    lines[start..end].to_vec()
}

fn apply_gradient(lines: &[String], colors: &themes::Colors) -> Vec<String> {
    let xs: Vec<usize> = lines
        .iter()
        .flat_map(|l| l.char_indices().filter(|(_, c)| *c != ' ').map(|(i, _)| i))
        .collect();
    let lo = xs.iter().min().copied().unwrap_or(0);
    let hi = xs.iter().max().copied().unwrap_or(1);
    let span = (hi - lo).max(1) as f64;

    lines
        .iter()
        .map(|line| {
            let mut buf = String::new();
            for (x, ch) in line.char_indices() {
                if ch == ' ' {
                    buf.push(ch);
                } else {
                    let (r, g, b) = themes::color_at(colors, (x - lo) as f64 / span);
                    buf.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}"));
                }
            }
            buf.push_str("\x1b[0m");
            buf
        })
        .collect()
}

fn render_figlet(
    text: &str,
    fig_filename: &str,
    width: usize,
    gradient: bool,
    theme: &str,
    color: bool,
) -> Result<String> {
    let bytes = assets::ascii_file(fig_filename)
        .ok_or_else(|| anyhow::anyhow!("FIGlet font not found in bundle: {fig_filename}"))?;
    let font = figdriver::FIGfont::from_reader(Cursor::new(bytes))
        .with_context(|| format!("Failed to load FIGlet font {fig_filename}"))?;
    let mut wrapper = figdriver::Wrapper::new(font, width);

    let captured: RefCell<Vec<String>> = RefCell::new(Vec::new());
    wrapper.write_line(text, &|lines| {
        captured.borrow_mut().extend_from_slice(lines);
    });
    let lines = trim_blank_edges(captured.into_inner());

    if gradient && color {
        let colors = themes::pick_colors(theme);
        Ok(apply_gradient(&lines, &colors).join("\n"))
    } else {
        Ok(lines.join("\n"))
    }
}

fn render(
    text: &str,
    group: &str,
    font_name: &str,
    fig_index: &HashMap<String, String>,
    color: bool,
    truecolor: bool,
    theme: &str,
    force_gradient: bool,
) -> Result<String> {
    let use_gradient = force_gradient || group == "gradient";

    if group != "ansi" {
        if let Some(filename) = fig_index.get(&norm(font_name)) {
            return render_figlet(text, filename, 120, use_gradient, theme, color);
        }
    }

    let tint = if use_gradient && color {
        Some(themes::pick_colors(theme))
    } else {
        None
    };
    tdf::render(text, font_name, color, truecolor, tint.as_ref())
}

// ── main ──────────────────────────────────────────────────────────────────────

fn spawn_pager(extra_args: &[&str]) -> Option<std::process::Child> {
    use std::process::{Command, Stdio};
    let pager_env = std::env::var("PAGER").ok();
    pager_env
        .as_deref()
        .and_then(|cmd| {
            Command::new("sh")
                .args(["-c", cmd])
                .stdin(Stdio::piped())
                .spawn()
                .ok()
        })
        .or_else(|| {
            Command::new("less")
                .args(extra_args)
                .stdin(Stdio::piped())
                .spawn()
                .ok()
        })
        .or_else(|| {
            Command::new("more")
                .stdin(Stdio::piped())
                .spawn()
                .ok()
        })
}

fn page_text(text: &str) {
    use std::io::Write;
    if !std::io::stdout().is_terminal() {
        print!("{text}");
        return;
    }
    let mut child = spawn_pager(&[]);
    if let Some(ref mut c) = child {
        if let Some(mut stdin) = c.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = c.wait();
    } else {
        print!("{text}");
    }
}


fn main() {
    if let Err(e) = run() {
        eprintln!("rban: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    #[cfg(unix)]
    libc_sigpipe();

    let args = Args::parse();

    if args.list_themes {
        println!("{}", themes::THEME_NAMES.join("\n"));
        return Ok(());
    }
    if args.credits {
        let header = format!(
            "rban {VERSION} — by {AUTHORS}\nhttps://github.com/s-brue/rban\n\n"
        );
        page_text(&format!("{header}{CREDITS}"));
        return Ok(());
    }
    if args.license {
        let deps = format!(
            "\nDependencies (compiled into this binary):\n{DEPS_LICENSES}\n"
        );
        let footer = "\nThe bundled fonts are NOT under this license.\nRun `rban --credits` for font attributions.\n";
        page_text(&format!("{LICENSE}{deps}{footer}"));
        return Ok(());
    }

    let all_tdf = tdf::tdf_names().unwrap_or_default();
    let fig_index = build_fig_index();

    if args.list_ansi {
        println!("{}", all_tdf.join("\n"));
        return Ok(());
    }
    if args.list_figlet {
        let mut names: Vec<String> = fig_index.keys().cloned().collect();
        names.sort();
        println!("{}", names.join("\n"));
        return Ok(());
    }

    if let Some(ref t) = args.theme {
        if t != "random" && !themes::THEME_NAMES.contains(&t.as_str()) {
            eprintln!("rban: unknown theme {t:?} (see --list-themes)");
            std::process::exit(1);
        }
    }

    let text = args.text.as_deref().ok_or_else(|| anyhow::anyhow!("missing text"))?;

    let conf_content = find_conf(args.conf.as_deref())
        .ok_or_else(|| anyhow::anyhow!("could not load fonts.conf"))?;
    let groups = load_groups(&conf_content, &all_tdf);

    let active: &[&str] = if args.filter.ansi_only {
        &["ansi"]
    } else if args.filter.ascii_only {
        &["fonts", "gradient"]
    } else {
        &["fonts", "gradient", "ansi"]
    };

    let items: Vec<(&str, String)> = active
        .iter()
        .flat_map(|&g| {
            groups
                .get(g)
                .map(|v| v.iter().map(|f| (g, f.clone())).collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .collect();

    if items.is_empty() {
        eprintln!("rban: no fonts in the config for that selection");
        std::process::exit(1);
    }

    let grad_set: HashSet<String> = groups
        .get("gradient")
        .map(|v| v.iter().map(|f| norm(f)).collect())
        .unwrap_or_default();
    let pick_group = |name: &str| -> &str {
        if grad_set.contains(&norm(name)) { "gradient" } else { "fonts" }
    };

    let chosen: Vec<(&str, String)> = if let Some(ref f) = args.mode.font {
        vec![(pick_group(f), f.clone())]
    } else if args.mode.all || args.mode.all_pager {
        items
    } else {
        // Default and --rand both pick randomly (no-repeat bag per group)
        let mut rng = rand::thread_rng();
        let non_empty: Vec<&&str> = active
            .iter()
            .filter(|&&g| groups.get(g).map(|v| !v.is_empty()).unwrap_or(false))
            .collect();
        if non_empty.is_empty() {
            eprintln!("rban: no fonts available for random selection");
            std::process::exit(1);
        }
        let g = non_empty[rng.gen_range(0..non_empty.len())];
        let pool = groups[*g].clone();
        let f = pick_random(g, &pool);
        vec![(*g, f)]
    };

    let use_color = !args.no_color
        && std::env::var("NO_COLOR").is_err()
        && (args.color || args.mode.all_pager || std::io::stdout().is_terminal());

    let theme = args.theme.as_deref().unwrap_or("random");
    let force_gradient = args.gradient || args.theme.is_some();

    let mut rc = 0;
    if args.mode.all_pager {
        use std::io::Write;
        let mut child = if std::io::stdout().is_terminal() {
            spawn_pager(&["-R"])
        } else {
            None
        };
        let mut pager_stdin = child.as_mut().and_then(|c| c.stdin.take());
        let mut first = true;
        for (group, font_name) in &chosen {
            match render(text, group, font_name, &fig_index, use_color, args.truecolor, theme, force_gradient) {
                Ok(out) => {
                    if let Some(ref mut stdin) = pager_stdin {
                        if !first { let _ = stdin.write_all(b"\n"); }
                        let _ = stdin.write_all(out.as_bytes());
                        let _ = stdin.write_all(b"\n");
                        if args.name { let _ = stdin.write_all(format!("[{font_name}]\n").as_bytes()); }
                    } else {
                        if !first { println!(); }
                        println!("{out}");
                        if args.name { println!("[{font_name}]"); }
                    }
                    first = false;
                }
                Err(e) => {
                    eprintln!("[{font_name}] {e}");
                    rc = 1;
                }
            }
        }
        drop(pager_stdin); // EOF → pager puede terminar
        if let Some(ref mut c) = child { let _ = c.wait(); }
    } else {
        let mut shown = 0;
        for (group, font_name) in &chosen {
            match render(text, group, font_name, &fig_index, use_color, args.truecolor, theme, force_gradient) {
                Ok(out) => {
                    if shown > 0 {
                        println!();
                    }
                    println!("{out}");
                    if args.name {
                        println!("[{font_name}]");
                    }
                    shown += 1;
                }
                Err(e) => {
                    eprintln!("[{font_name}] {e}");
                    rc = 1;
                }
            }
        }
    }
    std::process::exit(rc);
}

#[cfg(unix)]
fn libc_sigpipe() {
    unsafe extern "C" {
        fn signal(signum: i32, handler: usize) -> usize;
    }
    unsafe { signal(13 /* SIGPIPE */, 0 /* SIG_DFL */); }
}
