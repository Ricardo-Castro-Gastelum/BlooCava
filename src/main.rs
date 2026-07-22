mod config;
mod lang;
mod ui;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--version" => {
                println!("bloocava {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "-h" | "--help" => {
                print_help(lang::Lang::En);
                return Ok(());
            }
            _ => {
                eprintln!("Invalid argument. Use -h for help.");
                std::process::exit(1);
            }
        }
    }

    run_main_menu()
}

fn print_help(lang: lang::Lang) {
    let s = lang.strings();
    println!("{}", s.help_title);
    println!();
    println!("{}", s.help_usage);
    println!();
    println!("{}", s.help_options);
    println!("{}", s.help_opt_help);
    println!("{}", s.help_opt_version);
    println!();
    println!("{}", s.help_interactive);
}

fn cleanup_terminal(stdout: &mut io::Stdout) {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout, cursor::Show);
    let _ = execute!(stdout, ResetColor);
    let _ = execute!(stdout, cursor::MoveTo(0, 0));
    let _ = execute!(stdout, terminal::Clear(ClearType::All));
    let _ = stdout.flush();
}

fn run_main_menu() -> Result<(), String> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;
    execute!(stdout, cursor::Hide).ok();

    let mut selected: usize = 0;
    let mut current_lang = lang::Lang::Es;

    let result = (|| -> Result<(), String> {
        loop {
            let s = current_lang.strings();
            let options = [s.solid, s.gradient];

            execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).ok();

            execute!(stdout, cursor::MoveTo(2, 1)).ok();
            print_bloo_cava(&mut stdout);

            execute!(stdout, cursor::MoveTo(2, 13)).ok();
            execute!(stdout, SetForegroundColor(Color::Rgb { r: 150, g: 150, b: 150 })).ok();
            execute!(stdout, Print(s.title)).ok();
            execute!(stdout, ResetColor).ok();

            execute!(stdout, cursor::MoveTo(2, 15)).ok();
            execute!(stdout, SetForegroundColor(Color::Rgb { r: 100, g: 100, b: 100 })).ok();
            execute!(stdout, Print(s.select_option)).ok();
            execute!(stdout, ResetColor).ok();

            for (i, opt) in options.iter().enumerate() {
                let y = 17 + (i as u16);
                execute!(stdout, cursor::MoveTo(4, y)).ok();

                if i == selected {
                    execute!(stdout, SetForegroundColor(Color::White)).ok();
                    execute!(stdout, Print(format!("> {}", opt))).ok();
                } else {
                    execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
                    execute!(stdout, Print(format!("  {}", opt))).ok();
                }
                execute!(stdout, ResetColor).ok();
            }

            execute!(stdout, cursor::MoveTo(2, 20)).ok();
            execute!(stdout, SetForegroundColor(Color::Rgb { r: 100, g: 100, b: 100 })).ok();
            execute!(stdout, Print(s.nav_help)).ok();
            execute!(stdout, ResetColor).ok();

            execute!(stdout, cursor::MoveTo(2, 22)).ok();
            execute!(stdout, SetForegroundColor(Color::Rgb { r: 100, g: 100, b: 100 })).ok();
            execute!(stdout, Print(s.lang_select)).ok();
            execute!(stdout, ResetColor).ok();

            let lang_es = "[ES] Espanol";
            let lang_en = "[EN] English";
            let (active_es, active_en) = match current_lang {
                lang::Lang::Es => (true, false),
                lang::Lang::En => (false, true),
            };

            execute!(stdout, cursor::MoveTo(4, 23)).ok();
            if active_es {
                execute!(stdout, SetForegroundColor(Color::White)).ok();
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
            }
            execute!(stdout, Print(lang_es)).ok();
            execute!(stdout, ResetColor).ok();

            execute!(stdout, cursor::MoveTo(20, 23)).ok();
            if active_en {
                execute!(stdout, SetForegroundColor(Color::White)).ok();
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
            }
            execute!(stdout, Print(lang_en)).ok();
            execute!(stdout, ResetColor).ok();

            execute!(stdout, cursor::MoveTo(2, 25)).ok();
            execute!(stdout, SetForegroundColor(Color::Rgb { r: 80, g: 80, b: 80 })).ok();
            execute!(stdout, Print("[1] ES  [2] EN")).ok();
            execute!(stdout, ResetColor).ok();

            stdout.flush().ok();

            if let Event::Key(KeyEvent { code, .. }) = event::read().map_err(|e| format!("Event error: {}", e))? {
                match code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Up => {
                        selected = if selected == 0 {
                            options.len() - 1
                        } else {
                            selected - 1
                        };
                    }
                    KeyCode::Down => {
                        selected = (selected + 1) % options.len();
                    }
                    KeyCode::Char('1') => {
                        current_lang = lang::Lang::Es;
                        selected = 0;
                    }
                    KeyCode::Char('2') => {
                        current_lang = lang::Lang::En;
                        selected = 0;
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().ok();
                        execute!(stdout, cursor::Show).ok();
                        execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).ok();

                        match selected {
                            0 => ui::solid::run_solid_ui(current_lang)?,
                            1 => ui::gradient::run_gradient_ui(current_lang)?,
                            _ => {}
                        }

                        terminal::enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;
                        execute!(stdout, cursor::Hide).ok();
                    }
                    _ => {}
                }
            }
        } 
    })();
 
    cleanup_terminal(&mut stdout);
    result
}

fn print_bloo_cava(w: &mut io::Stdout) {
    let grey = Color::Rgb { r: 80, g: 80, b: 80 };
    let white = Color::White;
    let lines = [                                                                                                     
            "██████████    ████         ███████        ███████        ████████      ████████     ████      ████     ███████   ",   
            "████     ██   ████       ███     ███    ███     ███    ███     ███   ███      ███   ████      ████   ███     ███ ",
            "████     ███  ████      ████     ████  ████     ████  ████     ███  ████      ████  ████      ████  ████     ████", 
            "████     ███  ████      ████     ████  ████     ████  ████     █    ████      ████  ████      ████  ████     ████", 
            "████     ███  ████      ████     ████  ████     ████  ████          ████      ████  ████      ████  ████     ████", 
            "███████████   ████      ████     ████  ████     ████  ████          ██████████████  ████      ████  █████████████", 
            "████     ███  ████      ████     ████  ████     ████  ████          ████      ████  ████      ████  ████     ████",
            "████     ███  ████      ████     ████  ████     ████  ████     █    ████      ████  ████      ████  ████     ████",
            "████     ███  ████      ████     ████  ████     ████  ████     ███  ████      ████   █████  █████   ████     ████", 
            " ███     ███  ████        ██     ███     ██     ██     ███     ███  ████      ██       ███  ███     ████     ██  ", 
            "   █████████  █████████    ███████        ███████        ███████    ████      █          ████       ████     █   ",  
    ];

    let split = 53;
    for (i, line) in lines.iter().enumerate() {
        let bloo: String = line.chars().take(split).collect();
        let cava: String = line.chars().skip(split).collect();
        let _ = execute!(w, cursor::MoveTo(2, (i + 1) as u16));
        let _ = execute!(w, SetForegroundColor(grey));
        let _ = execute!(w, Print(&bloo));
        let _ = execute!(w, ResetColor);
        let _ = execute!(w, SetForegroundColor(white));
        let _ = execute!(w, Print(&cava));
        let _ = execute!(w, ResetColor);
    }
}
