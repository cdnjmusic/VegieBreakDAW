//! Minimal SVG-to-Painter icon renderer for Vega Break Studio.
//! Handles the simple SVG shapes in branding/icons/ without external crates.
//! Rendered via egui::Painter so no new dependency is required.

use eframe::egui;

/// Parse a hex color like "#00f0ff" or "rgb(0,240,255)".
fn parse_color(s: &str) -> egui::Color32 {
    let s = s.trim();
    if s.starts_with('#') {
        let hex = &s[1..];
        if let Ok(v) = u32::from_str_radix(hex, 16) {
            return egui::Color32::from_rgba_unmultiplied(
                ((v >> 16) & 0xFF) as u8,
                ((v >> 8) & 0xFF) as u8,
                (v & 0xFF) as u8,
                255,
            );
        }
    }
    egui::Color32::WHITE
}

/// Extract attribute value from an XML element string, e.g. `fill="#00f0ff"`.
fn attr(elem: &str, name: &str) -> Option<String> {
    let pat = format!("{}=", name);
    let start = elem.find(&pat)?;
    let start = start + pat.len();
    let rest = &elem[start..];
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let end = rest[1..].find(quote)?;
    Some(rest[1..1 + end].to_string())
}

/// Split points attribute like "5,3 19,12 5,21" into Vec<(f32, f32)>.
fn parse_points(s: &str) -> Vec<egui::Pos2> {
    s.split_whitespace()
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.split(',').collect();
            if parts.len() == 2 {
                let x = parts[0].trim().parse::<f32>().ok()?;
                let y = parts[1].trim().parse::<f32>().ok()?;
                Some(egui::Pos2::new(x, y))
            } else {
                None
            }
        })
        .collect()
}

/// Parse path data like "M4 12l8-8 8 8-8 8z" into drawing commands.
fn parse_path_d(d: &str) -> Vec<PathCmd> {
    let mut cmds = Vec::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut cur = String::new();
    for ch in d.chars() {
        if ch.is_ascii_alphabetic() {
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
            cur.push(ch);
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }

    let mut pos = egui::Pos2::ZERO;
    let mut start_pos = egui::Pos2::ZERO;

    for token in &tokens {
        let ch = token.chars().next().unwrap_or(' ');
        let rest = &token[1..].trim();
        match ch {
            'M' => {
                let pts = parse_points(rest);
                if let Some(p) = pts.first() {
                    pos = *p;
                    start_pos = *p;
                    cmds.push(PathCmd::MoveTo(pos));
                }
            }
            'L' => {
                let pts = parse_points(rest);
                if let Some(p) = pts.first() {
                    pos = *p;
                    cmds.push(PathCmd::LineTo(pos));
                }
            }
            'l' => {
                // relative line: split by whitespace, take pairs
                let nums: Vec<f32> = rest
                    .split_whitespace()
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect();
                for i in (0..nums.len()).step_by(2) {
                    if i + 1 < nums.len() {
                        pos = egui::Pos2::new(pos.x + nums[i], pos.y + nums[i + 1]);
                        cmds.push(PathCmd::LineTo(pos));
                    }
                }
            }
            'Z' | 'z' => {
                cmds.push(PathCmd::CloseTo(start_pos));
            }
            _ => {}
        }
    }

    cmds
}

enum PathCmd {
    MoveTo(egui::Pos2),
    LineTo(egui::Pos2),
    CloseTo(egui::Pos2),
}

/// Render a single SVG element onto a Painter.
pub fn render_svg_element(painter: &egui::Painter, rect: egui::Rect, svg: &str) {
    // Find all element tags within the SVG content.
    let mut start = 0;
    while let Some(tag_start) = svg[start..].find('<') {
        let tag_start = start + tag_start;
        if let Some(tag_end) = svg[tag_start..].find('>') {
            let tag_end = tag_start + tag_end;
            let elem = &svg[tag_start..=tag_end];

            // Skip non-element tags (<?xml, <!DOCTYPE, comments, closing </svg>)
            if elem.starts_with("</") || elem.starts_with("<?") || elem.starts_with("<!") {
                start = tag_end + 1;
                continue;
            }

            let color = attr(elem, "fill")
                .map(|c| parse_color(&c))
                .unwrap_or(egui::Color32::WHITE);

            if let Some(tag_name) = elem.split_whitespace().next() {
                let tag_name = tag_name.trim_start_matches('<').trim();

                match tag_name {
                    "rect" => {
                        let x: f32 = attr(elem, "x").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let y: f32 = attr(elem, "y").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let w: f32 = attr(elem, "width").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let h: f32 = attr(elem, "height").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let local_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(w, h));
                        let screen_rect = fit_rect(local_rect, rect);
                        painter.rect_filled(screen_rect, 0.0, color);
                    }
                    "circle" => {
                        let cx: f32 = attr(elem, "cx").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let cy: f32 = attr(elem, "cy").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let r: f32 = attr(elem, "r").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let local_center = egui::pos2(cx, cy);
                        let screen_center = fit_point(local_center, rect);
                        let screen_r = r * rect.width() / 24.0; // SVG viewBox is 24x24
                        painter.circle_filled(screen_center, screen_r, color);
                    }
                    "polygon" => {
                        if let Some(pts_str) = attr(elem, "points") {
                            let local_pts = parse_points(&pts_str);
                            let screen_pts: Vec<egui::Pos2> = local_pts
                                .iter()
                                .map(|p| fit_point(*p, rect))
                                .collect();
                            if screen_pts.len() >= 3 {
                                painter.add(egui::Shape::convex_polygon(
                                    screen_pts,
                                    color,
                                    egui::Stroke::NONE,
                                ));
                            }
                        }
                    }
                    "line" => {
                        let x1: f32 = attr(elem, "x1").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let y1: f32 = attr(elem, "y1").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let x2: f32 = attr(elem, "x2").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let y2: f32 = attr(elem, "y2").and_then(|v| v.parse().ok()).unwrap_or(0.0);
                        let stroke_w: f32 = attr(elem, "stroke-width").and_then(|v| v.parse().ok()).unwrap_or(1.0);
                        let stroke_color = attr(elem, "stroke").map(|c| parse_color(&c)).unwrap_or(color);
                        let p1 = fit_point(egui::pos2(x1, y1), rect);
                        let p2 = fit_point(egui::pos2(x2, y2), rect);
                        painter.line_segment([p1, p2], egui::Stroke::new(stroke_w * rect.width() / 24.0, stroke_color));
                    }
                    "path" => {
                        if let Some(d) = attr(elem, "d") {
                            let cmds = parse_path_d(&d);
                            if cmds.is_empty() {
                                continue;
                            }
                            let mut points = Vec::new();
                            let mut closed = false;
                            for cmd in &cmds {
                                match cmd {
                                    PathCmd::MoveTo(p) => {
                                        if !points.is_empty() {
                                            // flush previous polygon
                                            if points.len() >= 3 {
                                                painter.add(egui::Shape::convex_polygon(
                                                    points.iter().map(|p| fit_point(*p, rect)).collect(),
                                                    color,
                                                    egui::Stroke::NONE,
                                                ));
                                            }
                                            points.clear();
                                        }
                                        points.push(fit_point(*p, rect));
                                    }
                                    PathCmd::LineTo(p) => {
                                        points.push(fit_point(*p, rect));
                                    }
                                    PathCmd::CloseTo(start) => {
                                        points.push(fit_point(*start, rect));
                                        closed = true;
                                    }
                                }
                            }
                            if closed || points.len() >= 3 {
                                painter.add(egui::Shape::convex_polygon(
                                    points.iter().map(|p| fit_point(*p, rect)).collect(),
                                    color,
                                    egui::Stroke::NONE,
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
            start = tag_end + 1;
        } else {
            break;
        }
    }
}

/// Fit an SVG point (in 24x24 viewBox space) to screen rect.
fn fit_point(p: egui::Pos2, rect: egui::Rect) -> egui::Pos2 {
    egui::pos2(
        rect.left() + (p.x / 24.0) * rect.width(),
        rect.top() + (p.y / 24.0) * rect.height(),
    )
}

/// Fit an SVG rect to screen rect.
fn fit_rect(r: egui::Rect, view: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(fit_point(r.min, view), fit_point(r.max, view) - fit_point(r.min, view))
}

/// Load an SVG file content from the branding/icons directory.
pub fn load_svg(icon_name: &str) -> Option<String> {
    let path = std::path::Path::new("branding/icons").join(format!("{}.svg", icon_name));
    std::fs::read_to_string(path).ok()
}

/// Draw an SVG icon centered in the given rect.
pub fn draw_icon(painter: &egui::Painter, rect: egui::Rect, icon_name: &str) {
    if let Some(svg) = load_svg(icon_name) {
        // Inflate rect slightly for comfortable icon sizing
        let icon_rect = rect.shrink(2.0);
        render_svg_element(painter, icon_rect, &svg);
    }
}
