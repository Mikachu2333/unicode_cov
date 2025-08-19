const VERSION: &str = env!("CARGO_PKG_VERSION");

fn pause_and_exit() {
    eprintln!("\nPress Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::process::exit(0);
}

fn main() {
    let parms = std::env::args().collect::<Vec<String>>();

    let accepted_args = vec!["-h", "--help", "-v", "--version"];
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

    for i in parms.iter().skip(1) {
        let (is_unicode, pure_str) = judge_unicode(i);

        if is_unicode {
            if let Ok(code) = u32::from_str_radix(&pure_str, 16) {
                if let Some(ch) = std::char::from_u32(code) {
                    println!("<{}>\tU+{:04X}", ch, code);
                }
            }
        } else {
            let chars: Vec<char> = i.chars().collect();
            let codes: Vec<String> = chars
                .iter()
                .map(|ch| format!("U+{:04X}", *ch as u32))
                .collect();

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
fn judge_unicode(s: &str) -> (bool, String) {
    let each = s
        .to_lowercase()
        .replace("\\\\", "\\")
        .trim_start_matches("u+")
        .trim_start_matches("\\u")
        .trim_matches(|t| t == '[' || t == ']')
        .to_string();

    if each.is_empty() || each.len() != 4 {
        return (false, each);
    }

    for c in each.chars() {
        if !c.is_ascii_hexdigit() {
            return (false, each);
        }
    }

    return (true, each);
}

fn print_help(exe_name: &str) {
    println!("┏━━━━━━━━━━━━━━━━━━━━┓");
    print!("┃ Unicode  Converter ┃");
    print!("  ");
    print!("Version:{}", VERSION);
    print!("  ");
    println!(
        "BuildTime:{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    println!("┗━━━━━━━━━━━━━━━━━━━━┛");

    println!("Usage:");
    println!();
    println!("{} <Unicode chars> ...", exe_name);
    println!(
        "[e.g.] >> {} 0074 U+0065 u+0073 \\U0074 \\u793A [4F8B]",
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
    println!("{} <Unicode codes> ...", exe_name);
    println!("[e.g.] >> {} 测试Test", exe_name);
    println!(
        r#"        | 测      试      T       e       s       t
        | <U+6D4B><U+8BD5><U+0054><U+0065><U+0073><U+0074>"#
    );
    println!();
    println!("{} <Unicode codes> <Unicode chars> ... (Mixed)", exe_name);
    println!("[e.g.] >> {} U+6D4B \\u8BD5 Test", exe_name);
    println!(
        r#"        | <测>    U+6D4B
        | <试>    U+8BD5
        | T       e       s       t
        | <U+0054><U+0065><U+0073><U+0074>"#
    );
    println!();
}
