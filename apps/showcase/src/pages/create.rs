use nsh_core::prelude::*;
use serde_json::Value;

use crate::ui;

const STYLES: [&str; 3] = ["Nova", "Vega", "Orbit"];
const BASES: [&str; 5] = ["Neutral", "Stone", "Zinc", "Slate", "Sand"];
const THEMES: [&str; 3] = ["System", "Light", "Dark"];
const CHARTS: [&str; 4] = ["Neutral", "Blue", "Green", "Amber"];
const HEADINGS: [&str; 3] = ["Inter", "Serif", "Mono"];
const FONTS: [&str; 3] = ["Inter", "System", "Serif"];
const ICONS: [&str; 3] = ["Lucide", "Tabler", "Phosphor"];
const RADII: [&str; 4] = ["Default", "Compact", "Round", "None"];
const MENUS: [&str; 3] = ["Solid", "Subtle", "Outline"];
const ACCENTS: [&str; 3] = ["Subtle", "Muted", "Strong"];

const COLOR_TOKENS: [&str; 12] = [
    "background",
    "foreground",
    "card",
    "popover",
    "primary",
    "secondary",
    "muted",
    "accent",
    "destructive",
    "border",
    "input",
    "ring",
];

const COLOR_LABELS: [&str; 5] = ["Neutral", "Stone", "Warm", "Cool", "Deep"];

const COLOR_VALUES: [[&str; 5]; 12] = [
    [
        "oklch(1 0 0)",
        "oklch(0.985 0 0)",
        "oklch(0.99 0.005 80)",
        "oklch(0.99 0.005 250)",
        "oklch(0.98 0 0)",
    ],
    [
        "oklch(0.145 0 0)",
        "oklch(0.15 0.005 60)",
        "oklch(0.16 0.01 70)",
        "oklch(0.15 0.01 250)",
        "oklch(0.12 0 0)",
    ],
    [
        "oklch(1 0 0)",
        "oklch(0.985 0 0)",
        "oklch(0.99 0.005 80)",
        "oklch(0.985 0.005 250)",
        "oklch(0.97 0 0)",
    ],
    [
        "oklch(1 0 0)",
        "oklch(0.985 0 0)",
        "oklch(0.99 0.005 80)",
        "oklch(0.985 0.005 250)",
        "oklch(0.97 0 0)",
    ],
    [
        "oklch(0.205 0 0)",
        "oklch(0.22 0.01 60)",
        "oklch(0.24 0.03 70)",
        "oklch(0.22 0.04 250)",
        "oklch(0.18 0 0)",
    ],
    [
        "oklch(0.97 0 0)",
        "oklch(0.965 0.005 60)",
        "oklch(0.96 0.01 70)",
        "oklch(0.96 0.01 250)",
        "oklch(0.94 0 0)",
    ],
    [
        "oklch(0.97 0 0)",
        "oklch(0.965 0.005 60)",
        "oklch(0.96 0.01 70)",
        "oklch(0.96 0.01 250)",
        "oklch(0.94 0 0)",
    ],
    [
        "oklch(0.97 0 0)",
        "oklch(0.965 0.005 60)",
        "oklch(0.96 0.01 70)",
        "oklch(0.96 0.01 250)",
        "oklch(0.94 0 0)",
    ],
    [
        "oklch(0.577 0.245 27)",
        "oklch(0.58 0.22 40)",
        "oklch(0.6 0.18 60)",
        "oklch(0.58 0.2 20)",
        "oklch(0.52 0.22 27)",
    ],
    [
        "oklch(0.922 0 0)",
        "oklch(0.92 0.005 60)",
        "oklch(0.91 0.01 70)",
        "oklch(0.91 0.01 250)",
        "oklch(0.88 0 0)",
    ],
    [
        "oklch(0.922 0 0)",
        "oklch(0.92 0.005 60)",
        "oklch(0.91 0.01 70)",
        "oklch(0.91 0.01 250)",
        "oklch(0.88 0 0)",
    ],
    [
        "oklch(0.708 0 0)",
        "oklch(0.71 0.005 60)",
        "oklch(0.7 0.01 70)",
        "oklch(0.7 0.01 250)",
        "oklch(0.65 0 0)",
    ],
];

fn preset_command(values: &[usize; 10], colors: &[usize; 12]) -> String {
    let pick = |options: &[&str], index: usize| -> String {
        options[index % options.len()].to_ascii_lowercase()
    };
    let mut command = format!(
        "--preset a{} --style {} --base {} --theme {} --chart {} --heading {} --font {} --icons {} --radius {} --menu {} --accent {}",
        values[0] + 1,
        pick(&STYLES, values[0]),
        pick(&BASES, values[1]),
        pick(&THEMES, values[2]),
        pick(&CHARTS, values[3]),
        pick(&HEADINGS, values[4]),
        pick(&FONTS, values[5]),
        pick(&ICONS, values[6]),
        pick(&RADII, values[7]),
        pick(&MENUS, values[8]),
        pick(&ACCENTS, values[9]),
    );
    for (index, token) in COLOR_TOKENS.iter().enumerate() {
        command.push_str(&format!(
            " --{token} {}",
            pick(&COLOR_LABELS, colors[index])
        ));
    }
    command
}

fn apply_theme(colors: &[usize; 12], radius: usize) {
    let mut json: Value = match serde_json::from_str(include_str!("../../theme.json")) {
        Ok(value) => value,
        Err(_) => return,
    };
    for (index, token) in COLOR_TOKENS.iter().enumerate() {
        let value = Value::String(COLOR_VALUES[index][colors[index] % 5].to_string());
        json["cssVars"]["light"][*token] = value.clone();
        json["cssVars"]["dark"][*token] = value;
    }
    let radius_values = ["0.625rem", "0.375rem", "1rem", "0rem"];
    json["cssVars"]["theme"]["radius"] = Value::String(radius_values[radius % 4].to_string());
    if let Ok(theme) = nsh_base::Theme::from_json(&json.to_string()) {
        set_tokens(theme.into_tokens());
    }
}

pub fn create() -> impl IntoElement {
    let style = signal(0_usize);
    let base = signal(0_usize);
    let scheme = signal(0_usize);
    let chart = signal(0_usize);
    let heading = signal(0_usize);
    let font = signal(0_usize);
    let icons = signal(0_usize);
    let radius = signal(0_usize);
    let menu = signal(0_usize);
    let accent = signal(0_usize);
    let colors: [Signal<usize>; 12] = std::array::from_fn(|_| signal(0_usize));
    let step = signal(0_usize);
    let code = signal(false);
    let shuffle = signal(0_usize);

    let values = [
        style, base, scheme, chart, heading, font, icons, radius, menu, accent,
    ];
    let command = preset_command(
        &values.map(|signal| signal.get()),
        &colors.map(|signal| signal.get()),
    );

    let mut color_rows = view().flex_row().items_center().gap_3();
    for (index, token) in COLOR_TOKENS.iter().enumerate() {
        let value = colors[index];
        color_rows = color_rows.child(control(token, &COLOR_LABELS, value, move || {
            apply_theme(&colors.map(|signal| signal.get()), radius.get())
        }));
    }

    view()
        .flex_1()
        .gap_5()
        .child(ui::section_title(
            "New project",
            "Tune every token, preview it, then take the command.",
        ))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(step_chip("01", step, 0))
                .child(step_chip("02", step, 1)),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(control("Style", &STYLES, style, || {}))
                .child(control("Base Color", &BASES, base, || {}))
                .child(control("Appearance", &THEMES, scheme, || {}))
                .child(control("Chart Color", &CHARTS, chart, || {})),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(control("Heading", &HEADINGS, heading, || {}))
                .child(control("Font", &FONTS, font, || {}))
                .child(control("Icon Library", &ICONS, icons, || {}))
                .child(control("Radius", &RADII, radius, move || {
                    apply_theme(&colors.map(|v| v.get()), radius.get())
                })),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(control("Menu", &MENUS, menu, || {}))
                .child(control("Menu Accent", &ACCENTS, accent, || {})),
        )
        .child(ui::section_title(
            "Color tokens",
            "Every theme variable is editable.",
        ))
        .child(color_rows)
        .child(preview(command.clone()))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(ui::button(
                    "Open Preset",
                    ui::ButtonStyle::Outline,
                    move || code.update(|v| *v = !*v),
                ))
                .child(ui::button("Shuffle", ui::ButtonStyle::Outline, move || {
                    let n = shuffle.get() + 1;
                    shuffle.set(n);
                    style.set((n * 3) % STYLES.len());
                    base.set((n * 7) % BASES.len());
                    radius.set((n * 11) % RADII.len());
                    for (index, value) in colors.iter().enumerate() {
                        value.set((n * (index + 3)) % 5);
                    }
                    apply_theme(&colors.map(|v| v.get()), radius.get());
                }))
                .child(ui::button(
                    "Get Code",
                    ui::ButtonStyle::Primary,
                    move || {
                        set_clipboard(preset_command(
                            &values.map(|v| v.get()),
                            &colors.map(|v| v.get()),
                        ));
                    },
                )),
        )
        .child(when(
            move || code.get(),
            move || code_panel(command.clone()),
        ))
}

fn step_chip(label: &'static str, step: Signal<usize>, index: usize) -> impl IntoElement {
    view()
        .px_3()
        .py_1()
        .rounded(px(6.0))
        .bg(move || {
            if step.get() == index {
                nsh_base::theme().primary
            } else {
                nsh_base::theme().muted
            }
        })
        .on_press(move || step.set(index))
        .child(text(label).text_size(px(12.0)))
}

fn control(
    label: &'static str,
    options: &'static [&'static str],
    value: Signal<usize>,
    on_change: impl Fn() + 'static,
) -> impl IntoElement {
    view()
        .gap_1()
        .child(
            text(label)
                .text_size(px(11.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            view()
                .px_3()
                .py_2()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || nsh_base::theme().border)
                .on_press(move || {
                    value.update(|v| *v = (*v + 1) % options.len());
                    on_change();
                })
                .child(text(move || options[value.get() % options.len()]).text_size(px(13.0))),
        )
}

fn preview(command: String) -> impl IntoElement {
    view()
        .gap_3()
        .child(
            text("Live preview")
                .text_size(px(15.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(ui::card(
            "A surface in your theme",
            "Controls here follow the shared tokens.",
        ))
        .child(
            text(command)
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}

fn code_panel(command: String) -> impl IntoElement {
    view()
        .p_4()
        .rounded(px(8.0))
        .bg(move || nsh_base::theme().muted)
        .gap_2()
        .child(
            text("Command Palette")
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(text("Search for a command to run...").text_size(px(12.0)))
        .child(text(command).text_size(px(12.0)))
}

#[cfg(test)]
mod tests {
    use super::{COLOR_TOKENS, preset_command};

    #[test]
    fn command_contains_every_control() {
        let values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let colors = [0; 12];
        let command = preset_command(&values, &colors);
        assert!(command.starts_with("--preset a1"));
        for flag in [
            "--style",
            "--base",
            "--theme",
            "--chart",
            "--heading",
            "--font",
            "--icons",
            "--radius",
            "--menu",
            "--accent",
        ] {
            assert!(command.contains(flag));
        }
        for token in COLOR_TOKENS {
            assert!(command.contains(&format!("--{token}")));
        }
    }
}
