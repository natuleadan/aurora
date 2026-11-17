//! The preset designer: a dynamic panel whose controls update the preview and
//! generate a reusable command.

use nsh_core::prelude::*;

use crate::ui;

const STYLES: [&str; 3] = ["Nova", "Vega", "Orbit"];
const BASES: [&str; 5] = ["Neutral", "Stone", "Zinc", "Slate", "Sand"];
const THEMES: [&str; 3] = ["System", "Light", "Dark"];
const CHARTS: [&str; 4] = ["Neutral", "Blue", "Green", "Amber"];
const HEADINGS: [&str; 3] = ["Inter", "Serif", "Mono"];
const FONTS: [&str; 3] = ["Inter", "System", "Serif"];
const ICONS: [&str; 3] = ["Lucide", "Tabler", "Phosphor"];
const RADII: [&str; 3] = ["Default", "Compact", "Round"];
const MENUS: [&str; 3] = ["Solid", "Subtle", "Outline"];
const ACCENTS: [&str; 3] = ["Subtle", "Muted", "Strong"];

/// Builds the command from the current selection. Pure so it can be tested.
fn preset_command(values: &[usize; 10]) -> String {
    let pick = |options: &[&str], index: usize| -> String {
        options[index % options.len()].to_ascii_lowercase()
    };
    format!(
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
    )
}

#[allow(clippy::too_many_arguments)]
fn current_command(
    style: Signal<usize>,
    base: Signal<usize>,
    theme: Signal<usize>,
    chart: Signal<usize>,
    heading: Signal<usize>,
    font: Signal<usize>,
    icons: Signal<usize>,
    radius: Signal<usize>,
    menu: Signal<usize>,
    accent: Signal<usize>,
) -> String {
    preset_command(&[
        style.get(),
        base.get(),
        theme.get(),
        chart.get(),
        heading.get(),
        font.get(),
        icons.get(),
        radius.get(),
        menu.get(),
        accent.get(),
    ])
}

/// The two-step designer page.
pub fn create() -> impl IntoElement {
    let style = signal(0_usize);
    let base = signal(0_usize);
    let theme = signal(0_usize);
    let chart = signal(0_usize);
    let heading = signal(0_usize);
    let font = signal(0_usize);
    let icons = signal(0_usize);
    let radius = signal(0_usize);
    let menu = signal(0_usize);
    let accent = signal(0_usize);
    let step = signal(0_usize);
    let code = signal(false);
    let shuffle = signal(0_usize);

    view()
        .flex_1()
        .gap_5()
        .child(ui::section_title(
            "New project",
            "Shape a preset, preview it, then take the command.",
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
                .child(control("Style", &STYLES, style))
                .child(control("Base Color", &BASES, base))
                .child(control("Theme", &THEMES, theme))
                .child(control("Chart Color", &CHARTS, chart)),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(control("Heading", &HEADINGS, heading))
                .child(control("Font", &FONTS, font))
                .child(control("Icon Library", &ICONS, icons))
                .child(control("Radius", &RADII, radius)),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(control("Menu", &MENUS, menu))
                .child(control("Menu Accent", &ACCENTS, accent)),
        )
        .child(preview(current_command(
            style, base, theme, chart, heading, font, icons, radius, menu, accent,
        )))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(action("Open Preset", move || code.update(|v| *v = !*v)))
                .child(action("Shuffle", move || {
                    let n = shuffle.get() + 1;
                    shuffle.set(n);
                    style.set((n * 3) % STYLES.len());
                    base.set((n * 7) % BASES.len());
                    chart.set((n * 11) % CHARTS.len());
                    heading.set((n * 5) % HEADINGS.len());
                    font.set((n * 13) % FONTS.len());
                    icons.set((n * 17) % ICONS.len());
                    radius.set((n * 19) % RADII.len());
                    menu.set((n * 23) % MENUS.len());
                    accent.set((n * 29) % ACCENTS.len());
                }))
                .child(action("Get Code", move || {
                    set_clipboard(current_command(
                        style, base, theme, chart, heading, font, icons, radius, menu, accent,
                    ))
                })),
        )
        .child(when(
            move || code.get(),
            move || {
                code_panel(current_command(
                    style, base, theme, chart, heading, font, icons, radius, menu, accent,
                ))
            },
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
        .child(
            text(label)
                .text_size(px(12.0))
                .text_color(move || nsh_base::theme().primary_foreground),
        )
}

fn control(
    label: &'static str,
    options: &'static [&'static str],
    value: Signal<usize>,
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
                .on_press(move || value.update(|v| *v = (*v + 1) % options.len()))
                .child(text(move || options[value.get() % options.len()]).text_size(px(13.0))),
        )
}

fn action(label: &'static str, on_press: impl Fn() + 'static) -> impl IntoElement {
    view()
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || nsh_base::theme().border)
        .hover(|s| s.bg(nsh_base::theme().muted))
        .on_press(on_press)
        .child(text(label).text_size(px(13.0)))
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
            "Controls here follow the shared tokens, not a fixed palette.",
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
        .child(meta("Search for a command to run..."))
        .child(meta(command))
}

fn meta(label: impl Into<String>) -> impl IntoElement {
    text(label.into())
        .text_size(px(12.0))
        .text_color(move || nsh_base::theme().muted_foreground)
}

#[cfg(test)]
mod tests {
    use super::preset_command;

    #[test]
    fn preset_command_is_stable_and_contains_every_control() {
        let values = [0_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let first = preset_command(&values);
        assert_eq!(first, preset_command(&values));
        assert!(first.starts_with("--preset a1"));
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
            assert!(first.contains(flag), "missing {flag}");
        }
    }
}
