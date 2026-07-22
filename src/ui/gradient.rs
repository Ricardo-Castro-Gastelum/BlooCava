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

#[derive(Debug, Clone)]
struct ColorSlot {
    value: String,
    cursor_pos: usize,
}

impl ColorSlot {
    fn new() -> Self {
        Self {
            value: String::new(),
            cursor_pos: 0,
        }
    }
}

pub fn run_gradient_ui(lang: Lang) -> Result<(), String> {
    let s = lang.strings();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;

    let mut selected_count = 2;
    let mut colors: Vec<ColorSlot> = vec![ColorSlot::new(); 5];
    let mut active_color = 0;
    let mut message: Option<String> = None;
    let mut message_is_error = false;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).ok();

        execute!(stdout, cursor::MoveTo(2, 1)).ok();
        print_bloo_cava(&mut stdout);

        execute!(stdout, cursor::MoveTo(2, 13)).ok();
        execute!(stdout, SetForegroundColor(Color::Rgb { r: 150, g: 150, b: 150 })).ok();
        execute!(stdout, Print(s.gradient_mode)).ok();
        execute!(stdout, ResetColor).ok();

        execute!(stdout, cursor::MoveTo(2, 15)).ok();
        execute!(stdout, SetForegroundColor(Color::White)).ok();
        execute!(stdout, Print(s.color_count)).ok();
        execute!(stdout, ResetColor).ok();

        for i in 2..=5 {
            let x = 24 + ((i - 2) as u16 * 10);
            execute!(stdout, cursor::MoveTo(x, 15)).ok();
            if i == selected_count {
                execute!(stdout, SetForegroundColor(Color::White)).ok();
                execute!(stdout, Print(format!("[{}]", i))).ok();
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
                execute!(stdout, Print(format!(" {} ", i))).ok();
            }
            execute!(stdout, ResetColor).ok();
        }

        let input_start_y = 17;
        for i in 0..selected_count {
            let y = input_start_y + (i as u16 * 2);
            let slot = &colors[i];

            execute!(stdout, cursor::MoveTo(2, y)).ok();
            execute!(stdout, SetForegroundColor(Color::White)).ok();
            execute!(stdout, Print(format!("Color {}: ", i + 1))).ok();
            execute!(stdout, ResetColor).ok();

            let input_x = 12;
            execute!(stdout, cursor::MoveTo(input_x, y)).ok();

            if i == active_color {
                execute!(stdout, SetForegroundColor(Color::White)).ok();
            } else if !slot.value.is_empty() && slot.value.starts_with('#') && slot.value.len() == 7 {
                if let Some(color) = parse_hex_color(&slot.value) {
                    execute!(stdout, SetForegroundColor(color)).ok();
                }
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
            }

            execute!(stdout, Print(&slot.value)).ok();
            execute!(stdout, ResetColor).ok();
        }

        let preview_y = input_start_y + (selected_count as u16 * 2) + 1;
        execute!(stdout, cursor::MoveTo(2, preview_y)).ok();
        execute!(stdout, SetForegroundColor(Color::White)).ok();
        execute!(stdout, Print(s.preview)).ok();
        execute!(stdout, ResetColor).ok();

        let valid_colors: Vec<Color> = colors[..selected_count]
            .iter()
            .filter_map(|c| {
                if c.value.starts_with('#') && c.value.len() == 7 {
                    parse_hex_color(&c.value)
                } else {
                    None
                }
            })
            .collect();

        if valid_colors.len() == selected_count {
            for i in 0..20 {
                let t = i as f64 / 19.0;
                let color = interpolate_colors(&valid_colors, t);
                let block_char = "\u{2588}";
                execute!(stdout, cursor::MoveTo(2 + i as u16, preview_y + 1)).ok();
                execute!(stdout, SetForegroundColor(color)).ok();
                execute!(stdout, Print(block_char)).ok();
            }
            execute!(stdout, ResetColor).ok();

            execute!(stdout, cursor::MoveTo(2, preview_y + 2)).ok();
            for i in 0..20 {
                let t = i as f64 / 19.0;
                let color = interpolate_colors(&valid_colors, t);
                execute!(stdout, cursor::MoveTo(2 + i as u16, preview_y + 2)).ok();
                execute!(stdout, SetForegroundColor(color)).ok();
                execute!(stdout, Print("\u{2584}")).ok();
            }
            execute!(stdout, ResetColor).ok();
        } else {
            execute!(stdout, cursor::MoveTo(2, preview_y + 1)).ok();
            execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
            execute!(stdout, Print(s.enter_all_hex)).ok();
            execute!(stdout, ResetColor).ok();
        }

        if let Some(ref msg) = message {
            execute!(stdout, cursor::MoveTo(2, preview_y + 4)).ok();
            if message_is_error {
                execute!(stdout, SetForegroundColor(Color::Red)).ok();
            } else {
                execute!(stdout, SetForegroundColor(Color::Green)).ok();
            }
            execute!(stdout, Print(msg)).ok();
            execute!(stdout, ResetColor).ok();
        }

        execute!(stdout, cursor::MoveTo(2, preview_y + 6)).ok();
        execute!(stdout, SetForegroundColor(Color::Rgb { r: 100, g: 100, b: 100 })).ok();
        execute!(stdout, Print(s.gradient_help)).ok();
        execute!(stdout, ResetColor).ok();

        let slot = &colors[active_color];
        let input_x = 12;
        execute!(stdout, cursor::MoveTo(input_x + slot.cursor_pos as u16, input_start_y + (active_color as u16 * 2))).ok();
        stdout.flush().ok();

        if let Event::Key(KeyEvent { code, .. }) = event::read().map_err(|e| format!("Event error: {}", e))? {
            match code {
                KeyCode::Esc => break,
                KeyCode::Tab => {
                    active_color = (active_color + 1) % selected_count;
                    message = None;
                }
                KeyCode::BackTab => {
                    active_color = if active_color == 0 {
                        selected_count - 1
                    } else {
                        active_color - 1
                    };
                    message = None;
                }
                KeyCode::Up => {
                    if selected_count < 5 {
                        selected_count += 1;
                        message = None;
                    }
                }
                KeyCode::Down => {
                    if selected_count > 2 {
                        selected_count -= 1;
                        if active_color >= selected_count {
                            active_color = selected_count - 1;
                        }
                        message = None;
                    }
                }
                KeyCode::Enter => {
                    let mut all_valid = true;
                    for i in 0..selected_count {
                        let c = &colors[i];
                        if c.value.is_empty() || !c.value.starts_with('#') || c.value.len() != 7 || parse_hex_color(&c.value).is_none() {
                            all_valid = false;
                            message = Some(s.invalid_color.replace("{}", &(i + 1).to_string()));
                            message_is_error = true;
                            active_color = i;
                            break;
                        }
                    }

                    if all_valid {
                        let color_values: Vec<String> = colors[..selected_count]
                            .iter()
                            .map(|c| c.value.clone())
                            .collect();

                        match config::write_gradient(&color_values) {
                            Ok(()) => {
                                message = Some(s.gradient_applied.to_string());
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
                    let slot = &mut colors[active_color];
                    if slot.cursor_pos > 0 {
                        slot.value.pop();
                        slot.cursor_pos -= 1;
                        message = None;
                    }
                }
                KeyCode::Char(c) => {
                    let slot = &mut colors[active_color];
                    if slot.value.len() < 7 {
                        slot.value.push(c);
                        slot.cursor_pos += 1;
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

fn interpolate_colors(colors: &[Color], t: f64) -> Color {
    if colors.len() == 1 {
        return colors[0];
    }

    let segment = t * (colors.len() - 1) as f64;
    let idx = segment.floor() as usize;
    let frac = segment - idx as f64;

    let c1 = &colors[idx.min(colors.len() - 1)];
    let c2 = &colors[(idx + 1).min(colors.len() - 1)];

    let (r1, g1, b1) = color_to_rgb(c1);
    let (r2, g2, b2) = color_to_rgb(c2);

    let r = (r1 as f64 + (r2 as f64 - r1 as f64) * frac) as u8;
    let g = (g1 as f64 + (g2 as f64 - g1 as f64) * frac) as u8;
    let b = (b1 as f64 + (b2 as f64 - b1 as f64) * frac) as u8;

    Color::Rgb { r, g, b }
}

fn color_to_rgb(c: &Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb { r, g, b } => (*r, *g, *b),
        _ => (0, 0, 0),
    }
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
