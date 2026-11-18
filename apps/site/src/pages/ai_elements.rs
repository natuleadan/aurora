use nsh_core::prelude::*;

const AI_ELEMENT_NAMES: [&str; 20] = [
    "Chain of Thought",
    "Chat",
    "Checkpoint",
    "Confirmation",
    "Context",
    "File Tree",
    "Inline Citation",
    "Message",
    "Model Selector",
    "Plan",
    "Prompt Input",
    "Queue",
    "Reasoning",
    "Sources",
    "Stack Trace",
    "Subagent",
    "Suggestion",
    "Task",
    "Terminal",
    "Test Results",
];

pub fn ai_elements() -> impl IntoElement {
    view()
        .p_6()
        .gap_4()
        .max_w(px(900.0))
        .child(
            text("AI Elements")
                .text_size(px(28.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(
            text("20 specialized components designed specifically for agentic harnesses and interactive AI interfaces.")
                .text_size(px(14.0))
                .text_color(move || nsh_base::theme().muted_foreground),
        )
        .child(
            view()
                .gap_2()
                .children(
                    AI_ELEMENT_NAMES.iter().map(|name| {
                        aurora_components::card(
                            name,
                            "Deterministic agent primitive with accessible keyboard and focus states.",
                        )
                    }),
                ),
        )
}
