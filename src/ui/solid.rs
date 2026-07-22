use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::config;
use crate::lang::Lang;

pub fn run_solid_ui(lang: Lang) -> Result<(), String> {
    let s = lang.strings();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;

    let mut color_input = String::new();
    let mut cursor_pos = 0;
    let mut message: Option<String> = None;
    let mut message_is_error = false;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).ok();

        execute!(stdout, cursor::MoveTo(2, 1)).ok();
        print_bloo_cava(&mut stdout);

        execute!(stdout, cursor::MoveTo(2, 9)).ok();
        execute!(stdout, SetForegroundColor(Color::Rgb { r: 150, g: 150, b: 150 })).ok();
        execute!(stdout, Print(s.solid_mode)).ok();
        execute!(stdout, ResetColor).ok();

        execute!(stdout, cursor::MoveTo(2, 11)).ok();
        execute!(stdout, SetForegroundColor(Color::White)).ok();
        execute!(stdout, Print(s.hex_color_hint)).ok();
        execute!(stdout, ResetColor).ok();

        let input_x = 26;
        execute!(stdout, cursor::MoveTo(input_x, 11)).ok();
        execute!(stdout, SetForegroundColor(Color::White)).ok();
        execute!(stdout, Print(&color_input)).ok();
        execute!(stdout, ResetColor).ok();

        let preview_y = 13;
        execute!(stdout, cursor::MoveTo(2, preview_y)).ok();
        execute!(stdout, SetForegroundColor(Color::White)).ok();
        execute!(stdout, Print(s.preview)).ok();
        execute!(stdout, ResetColor).ok();

        execute!(stdout, cursor::MoveTo(2, preview_y + 1)).ok();
        if !color_input.is_empty() && color_input.starts_with('#') && color_input.len() == 7 {
            if let Some(color) = parse_hex_color(&color_input) {
                for i in 0..20 {
                    let height = ((i as f64 / 19.0) * 6.0) as u32;
                    let block_char = match height {
                        0 => " ",
                        1 => "\u{2581}",
                        2 => "\u{2582}",
                        3 => "\u{2583}",
                        4 => "\u{2584}",
                        5 => "\u{2585}",
                        6 => "\u{2586}",
                        _ => "\u{2588}",
                    };
                    execute!(stdout, cursor::MoveTo(2 + i as u16, preview_y + 1)).ok();
                    execute!(stdout, SetForegroundColor(color)).ok();
                    execute!(stdout, Print(block_char)).ok();
                }
                execute!(stdout, ResetColor).ok();

                execute!(stdout, cursor::MoveTo(2, preview_y + 2)).ok();
                execute!(stdout, SetForegroundColor(color)).ok();
                execute!(stdout, Print(&format!("████████████████████"))).ok();
                execute!(stdout, ResetColor).ok();
            }
        } else {
            execute!(stdout, cursor::MoveTo(2, preview_y + 1)).ok();
            execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
            execute!(stdout, Print(s.enter_hex_valid)).ok();
            execute!(stdout, ResetColor).ok();
        }

        if let Some(ref msg) = message {
            execute!(stdout, cursor::MoveTo(2, 18)).ok();
            if message_is_error {
                execute!(stdout, SetForegroundColor(Color::Red)).ok();
            } else {
                execute!(stdout, SetForegroundColor(Color::Green)).ok();
            }
            execute!(stdout, Print(msg)).ok();
            execute!(stdout, ResetColor).ok();
        }

        execute!(stdout, cursor::MoveTo(2, 20)).ok();
        execute!(stdout, SetForegroundColor(Color::Rgb { r: 100, g: 100, b: 100 })).ok();
        execute!(stdout, Print(s.apply_help)).ok();
        execute!(stdout, ResetColor).ok();

        execute!(stdout, cursor::MoveTo(input_x + cursor_pos as u16, 11)).ok();
        stdout.flush().ok();

        if let Event::Key(KeyEvent { code, .. }) = event::read().map_err(|e| format!("Event error: {}", e))? {
            match code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    if color_input.is_empty() || !color_input.starts_with('#') || color_input.len() != 7 {
                        message = Some(s.invalid_format.to_string());
                        message_is_error = true;
                    } else if parse_hex_color(&color_input).is_none() {
                        message = Some(s.invalid_hex.to_string());
                        message_is_error = true;
                    } else {
                        match config::write_solid_color(&color_input) {
                            Ok(()) => {
                                message = Some(s.color_applied.to_string());
                                message_is_error = false;
                            }
                            Err(e) => {
                                message = Some(format!("Error: {}", e));
                                message_is_error = true;
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if cursor_pos > 0 {
                        color_input.pop();
                        cursor_pos -= 1;
                        message = None;
                    }
                }
                KeyCode::Char(c) => {
                    if color_input.len() < 7 {
                        color_input.push(c);
                        cursor_pos += 1;
                        message = None;
                    }
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode().ok();
    execute!(stdout, cursor::Show).ok();
    execute!(stdout, ResetColor).ok();
    execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).ok();
    stdout.flush().ok();
    Ok(())
}

fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::Rgb { r, g, b })
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
