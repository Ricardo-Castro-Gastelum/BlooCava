use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CavaConfig {
    pub gradient: bool,
    pub gradient_colors: Vec<String>,
    pub foreground: String,
    pub background: String,
}

impl Default for CavaConfig {
    fn default() -> Self {
        Self {
            gradient: false,
            gradient_colors: Vec::new(),
            foreground: String::new(),
            background: String::new(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("cava").join("config")
}

pub fn read_config() -> Result<CavaConfig, String> {
    let path = get_config_path();
    if !path.exists() {
        return Err(format!("Cava config not found at: {}", path.display()));
    }

    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
    let mut config = CavaConfig::default();
    let mut in_color_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[color]" {
            in_color_section = true;
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_color_section = false;
            continue;
        }

        if in_color_section {
            let clean = if trimmed.starts_with(';') {
                trimmed[1..].trim()
            } else {
                trimmed
            };

            if let Some(val) = clean.strip_prefix("gradient").and_then(|v| v.trim().strip_prefix('=')) {
                config.gradient = val.trim() == "1";
            } else if let Some(val) = clean.strip_prefix("foreground").and_then(|v| v.trim().strip_prefix('=')) {
                config.foreground = extract_color_value(val);
            } else if let Some(val) = clean.strip_prefix("background").and_then(|v| v.trim().strip_prefix('=')) {
                config.background = extract_color_value(val);
            } else if let Some(val) = clean.strip_prefix("gradient_color_").and_then(|v| v.trim().strip_prefix('=')) {
                config.gradient_colors.push(extract_color_value(val));
            }
        }
    }

    Ok(config)
}

fn extract_color_value(val: &str) -> String {
    let trimmed = val.trim();
    if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        trimmed[1..trimmed.len()-1].to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn write_solid_color(color: &str) -> Result<(), String> {
    let path = get_config_path();
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
    let mut new_lines = Vec::new();
    let mut in_color_section = false;
    let mut gradient_set = false;
    let mut foreground_set = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[color]" {
            in_color_section = true;
            new_lines.push(line.to_string());
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if in_color_section && !gradient_set {
                new_lines.push("gradient = 0".to_string());
                gradient_set = true;
            }
            if in_color_section && !foreground_set {
                new_lines.push(format!("foreground = '{}'", color));
                foreground_set = true;
            }
            in_color_section = false;
            new_lines.push(line.to_string());
            continue;
        }

        if in_color_section {
            let clean = if trimmed.starts_with(';') {
                trimmed[1..].trim()
            } else {
                trimmed
            };

            if clean.starts_with("gradient ") && !clean.starts_with("gradient_color") {
                new_lines.push("gradient = 0".to_string());
                gradient_set = true;
                continue;
            }

            if clean.starts_with("foreground") {
                new_lines.push(format!("foreground = '{}'", color));
                foreground_set = true;
                continue;
            }

            if clean.starts_with("gradient_color_") || clean.starts_with("gradient_count") {
                continue;
            }
        }

        new_lines.push(line.to_string());
    }

    if in_color_section && !gradient_set {
        new_lines.push("gradient = 0".to_string());
    }
    if in_color_section && !foreground_set {
        new_lines.push(format!("foreground = '{}'", color));
    }

    fs::write(&path, new_lines.join("\n")).map_err(|e| format!("Failed to write config: {}", e))?;
    reload_cava_colors();
    Ok(())
}

pub fn write_gradient(colors: &[String]) -> Result<(), String> {
    let path = get_config_path();
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
    let mut new_lines = Vec::new();
    let mut in_color_section = false;
    let mut gradient_set = false;
    let mut colors_added = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[color]" {
            in_color_section = true;
            new_lines.push(line.to_string());
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if in_color_section && !gradient_set {
                new_lines.push("gradient = 1".to_string());
                gradient_set = true;
            }
            if in_color_section && !colors_added {
                for (i, c) in colors.iter().enumerate() {
                    new_lines.push(format!("gradient_color_{} = '{}'", i + 1, c));
                }
                colors_added = true;
            }
            in_color_section = false;
            new_lines.push(line.to_string());
            continue;
        }

        if in_color_section {
            let clean = if trimmed.starts_with(';') {
                trimmed[1..].trim()
            } else {
                trimmed
            };

            if clean.starts_with("gradient ") && !clean.starts_with("gradient_color") {
                new_lines.push("gradient = 1".to_string());
                gradient_set = true;
                continue;
            }

            if clean.starts_with("gradient_color_") {
                continue;
            }

            if clean.starts_with("foreground") || clean.starts_with("background") {
                new_lines.push(line.to_string());
                continue;
            }
        }

        new_lines.push(line.to_string());
    }

    if in_color_section && !gradient_set {
        new_lines.push("gradient = 1".to_string());
    }
    if in_color_section && !colors_added {
        for (i, c) in colors.iter().enumerate() {
            new_lines.push(format!("gradient_color_{} = '{}'", i + 1, c));
        }
    }

    fs::write(&path, new_lines.join("\n")).map_err(|e| format!("Failed to write config: {}", e))?;
    reload_cava_colors();
    Ok(())
}

fn reload_cava_colors() {
    let _ = std::process::Command::new("pkill")
        .args(["-x", "-USR1", "cava"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
