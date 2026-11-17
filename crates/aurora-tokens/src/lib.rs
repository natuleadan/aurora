pub const TOKENS: [&str; 24] = [
    "background",
    "foreground",
    "card",
    "card-foreground",
    "popover",
    "popover-foreground",
    "primary",
    "primary-foreground",
    "secondary",
    "secondary-foreground",
    "muted",
    "muted-foreground",
    "accent",
    "accent-foreground",
    "destructive",
    "destructive-foreground",
    "border",
    "input",
    "ring",
    "chart-1",
    "chart-2",
    "chart-3",
    "chart-4",
    "chart-5",
];

const FACTORS: [f32; 8] = [0.6, 0.8, 1.0, 1.4, 1.8, 2.2, 2.6, 1.0];
const NAMES: [&str; 8] = ["sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "base"];

pub fn radius_scale(base: f32) -> [(&'static str, f32); 8] {
    let mut out = [("", 0.0_f32); 8];
    let mut i = 0;
    while i < 8 {
        out[i] = (NAMES[i], base * FACTORS[i]);
        i += 1;
    }
    out
}

pub struct Preset {
    pub name: &'static str,
    pub radius: f32,
    pub primary_light: &'static str,
    pub primary_dark: &'static str,
}

pub fn presets() -> [Preset; 5] {
    [
        Preset {
            name: "neutral",
            radius: 10.0,
            primary_light: "oklch(0.205 0 0)",
            primary_dark: "oklch(0.922 0 0)",
        },
        Preset {
            name: "stone",
            radius: 10.0,
            primary_light: "oklch(0.216 0.006 56)",
            primary_dark: "oklch(0.921 0.005 56)",
        },
        Preset {
            name: "zinc",
            radius: 10.0,
            primary_light: "oklch(0.21 0.006 286)",
            primary_dark: "oklch(0.92 0.004 286)",
        },
        Preset {
            name: "slate",
            radius: 10.0,
            primary_light: "oklch(0.208 0.042 265)",
            primary_dark: "oklch(0.929 0.013 255)",
        },
        Preset {
            name: "sand",
            radius: 14.0,
            primary_light: "oklch(0.22 0.02 70)",
            primary_dark: "oklch(0.93 0.01 70)",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{TOKENS, presets, radius_scale};

    #[test]
    fn tokens_are_unique() {
        let mut copy = TOKENS;
        copy.sort_unstable();
        let mut i = 1;
        while i < copy.len() {
            assert_ne!(copy[i - 1], copy[i]);
            i += 1;
        }
    }

    #[test]
    fn radius_scale_derives_from_base() {
        let scale = radius_scale(10.0);
        assert_eq!(scale[2].1, 10.0);
        assert_eq!(scale[0].1, 6.0);
        assert_eq!(scale[7].1, 10.0);
    }

    #[test]
    fn presets_have_both_appearances() {
        for p in presets() {
            assert!(!p.primary_light.is_empty());
            assert!(!p.primary_dark.is_empty());
        }
    }
}
