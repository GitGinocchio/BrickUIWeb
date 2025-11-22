// colors.ts

export const colors = {
  light: {
    // Primary
    primary: "#cb4153",            // colore principale
    primaryDark: "#b73a49",        // versione più scura per hover/active
    primaryLight: "#e06c7c",       // versione più chiara per hover/focus
    primaryGlow: "#f399a5",        // glow/effetto luce
    primaryForeground: "#ffffff",   // testo su primary

    // Secondary / Neutrals
    background: "#ffffff",
    foreground: "#1a1a1a",
    card: "#fefefe",
    cardForeground: "#1a1a1a",
    secondary: "#f5f5f5",
    secondaryForeground: "#1a1a1a",
    muted: "#eaeaea",
    mutedForeground: "#5a5a5a",
    border: "#e0e0e0",
    input: "#e6e6e6",

    // Tertiary
    tertiary: "#ffffff",
    tertiaryForeground: "#727272",
    tertiaryPressed: "#757373ff",
    focus: "#fff",

    // Accent
    accent: "#fce4e8",
    accentForeground: "#cb4153",

    // Destructive / Error
    destructive: "#f04f4f",
    destructiveForeground: "#ffffff",

    // Sidebar
    sidebarBackground: "#fafafa",
    sidebarForeground: "#434551",
    sidebarPrimary: "#1a1a1a",
    sidebarPrimaryForeground: "#fafafa",
    sidebarAccent: "#f4f4f4",
    sidebarAccentForeground: "#1a1a1a",
    sidebarBorder: "#e0e0e0",
    sidebarRing: "#cb4153",

    // Gradients
    gradientHero: "linear-gradient(135deg, #cb4153, #e06c7c)",
    gradientSubtle: "linear-gradient(180deg, #ffffff, #f5f5f5)",

    // Shadows
    shadowBrick: "0 8px 24px -4px rgba(203,65,83,0.2)",
    shadowHover: "0 12px 32px -4px rgba(203,65,83,0.3)"
  },

  dark: {
    // Primary
    primary: "#cb4153",
    primaryDark: "#a0323f",
    primaryLight: "#d96c78",
    primaryGlow: "#e3878f",
    primaryForeground: "#ffffff",

    // Secondary / Neutrals
    background: "#121212",
    foreground: "#fafafa",
    card: "#1e1e1e",
    cardForeground: "#fafafa",
    secondary: "#262626",
    secondaryForeground: "#fafafa",
    muted: "#1a1a1a",
    mutedForeground: "#a6a6a6",
    border: "#333333",
    input: "#333333",

    // Accent
    accent: "#3c0f12",
    accentForeground: "#cb4153",

    // Destructive / Error
    destructive: "#802626",
    destructiveForeground: "#fafafa",

    // Sidebar
    sidebarBackground: "#1a1a1a",
    sidebarForeground: "#f4f4f4",
    sidebarPrimary: "#4d6ef1",
    sidebarPrimaryForeground: "#ffffff",
    sidebarAccent: "#292929",
    sidebarAccentForeground: "#f4f4f4",
    sidebarBorder: "#292929",
    sidebarRing: "#cb4153",

    // Gradients
    gradientHero: "linear-gradient(135deg, #cb4153, #d96c78)",
    gradientSubtle: "linear-gradient(180deg, #121212, #1e1e1e)",

    // Shadows
    shadowBrick: "0 8px 24px -4px rgba(203,65,83,0.3)",
    shadowHover: "0 12px 32px -4px rgba(203,65,83,0.4)"
  }
}

export type ColorTheme = typeof colors.light
