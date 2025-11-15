const VERSION: &str = env!("CARGO_PKG_VERSION");

fn pause_and_exit() {
    eprintln!("\nPress Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::process::exit(0);
}

fn main() {
    let parms = std::env::args().collect::<Vec<String>>();

    let accepted_args = ["-h", "--help", "-v", "--version"];
    if (parms.len() == 2 && (accepted_args.contains(&parms[1].as_str()))) || (parms.len() == 1) {
        print_help(
            std::path::PathBuf::from(parms[0].clone())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        pause_and_exit();
    }

    for arg in parms.iter().skip(1) {
        if let Some(code) = judge_unicode(arg) {
            if let Some(ch) = char::from_u32(code) {
                println!("<{}>\t{}", ch, format_unicode(code));
            } else {
                eprintln!(
                    "Error: Invalid Unicode code point: {}",
                    format_unicode(code)
                );
            }
        } else {
            let chars: Vec<char> = arg.chars().collect();
            let codes: Vec<String> = chars.iter().map(|ch| format_unicode(*ch as u32)).collect();

            for ch in &chars {
                print!("{}\t", ch);
            }
            println!();

            for code in &codes {
                print!("<{}>", code);
            }
            println!();
        }
    }
}
fn judge_unicode(s: &str) -> Option<u32> {
    let trimmed = s.trim();
    if trimmed.chars().count() <= 2 {
        return None;
    }

    // 使用字符迭代器而不是字节索引
    let mut chars = trimmed.chars();
    let first = chars.next()?;
    let second = chars.next()?;

    // 检查前两个字符是否为 "u+" 或 "U+"
    if !(first.eq_ignore_ascii_case(&'u') && second == '+') {
        return None;
    }

    let rest: String = chars.collect();
    let cleaned = rest.trim();

    let char_count = cleaned.chars().count();
    if !(4..=6).contains(&char_count) {
        return None;
    }

    if !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    let code = u32::from_str_radix(cleaned, 16).ok()?;

    if code > 0x10FFFF {
        return None;
    }

    Some(code)
}

fn format_unicode(code: u32) -> String {
    let width = match code {
        0x0000..=0xFFFF => 4,
        0x10000..=0xFFFFF => 5,
        _ => 6,
    };
    format!("U+{:0>width$X}", code, width = width)
}

fn print_help(exe_name: &str) {
    println!("┏━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ Unicode  Converter ┃");
    println!("┃ Version: {}       ┃", VERSION);
    println!("┗━━━━━━━━━━━━━━━━━━━━┛");

    println!("Usage:");
    println!();
    println!("{} <Unicode chars> ...", exe_name);
    println!(
        "[e.g.] >> {} u+0074 U+0065 u+0073 U+0074 U+793A U+4F8B",
        exe_name
    );
    println!(
        r#"        | <t>     U+0074
        | <e>     U+0065
        | <s>     U+0073
        | <t>     U+0074
        | <示>    U+793A
        | <例>    U+4F8B"#
    );
    println!();
    println!("{} <text> ...", exe_name);
    println!("[e.g.] >> {} 测试Test", exe_name);
    println!(
        r#"        | 测      试      T       e       s       t
        | <U+6D4B><U+8BD5><U+0054><U+0065><U+0073><U+0074>"#
    );
    println!();
    println!("{} <Unicode codes> <Unicode chars> ... (Mixed)", exe_name);
    println!("[e.g.] >> {} U+6D4B U+8BD5 Test", exe_name);
    println!(
        r#"        | <测>    U+6D4B
        | <试>    U+8BD5
        | T       e       s       t
        | <U+0054><U+0065><U+0073><U+0074>"#
    );
    println!();
}
