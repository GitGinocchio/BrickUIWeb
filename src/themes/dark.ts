// darkTheme.ts
import { type GlobalThemeOverrides } from "naive-ui"
import { colors } from "./colors"

export const dark: GlobalThemeOverrides = {
  common: {
    primaryColor: colors.dark.primary,
    primaryColorHover: colors.dark.primaryLight,
    primaryColorPressed: colors.dark.primaryGlow,
    primaryColorSuppl: colors.dark.primary,
    textColorBase: colors.dark.foreground
  },

  Button: {
    // === PRIMARY ===
    textColor: colors.dark.primaryForeground,

    // === SECONDARY ===
    colorSecondary: colors.dark.primaryDark,
    colorSecondaryHover: colors.dark.primary,
    colorSecondaryPressed: colors.dark.primaryGlow,
    paddingMedium: "0.5rem 1rem",
    paddingLarge: "0.5rem 1.75rem",
    borderRadiusMedium: "10px",
    borderRadiusLarge: "10px",
    heightMedium: "2.5rem",
    heightLarge: "3rem",
    fontWeight: "500",

    // === TERTIARY ===
    colorTertiary: colors.dark.card,
    textColorTertiary: colors.dark.cardForeground,
    colorTertiaryHover: colors.dark.secondary,
    colorTertiaryPressed: colors.dark.muted,

    // === QUATERNARY ===
    colorQuaternary: colors.dark.background,
    colorQuaternaryHover: colors.dark.primaryDark,
    colorQuaternaryPressed: colors.dark.primary,
    textColorQuaternary: colors.dark.foreground
  }
}
