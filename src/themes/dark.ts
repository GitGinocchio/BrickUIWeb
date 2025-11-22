// darkTheme.ts
import { type GlobalThemeOverrides } from "naive-ui"
import { colors } from "./colors" // Assumendo che colors.ts sia nella stessa cartella

export const dark: GlobalThemeOverrides = {
  common: {
    primaryColor: colors.dark.primary,
    primaryColorHover: colors.dark.primaryLight,
    primaryColorPressed: colors.dark.primaryGlow,
    primaryColorSuppl: colors.dark.primary,
    textColorBase: colors.dark.foreground
  },

  Button: {
    // === SECONDARY ===
    colorSecondary: colors.dark.primaryDark,
    colorSecondaryHover: colors.dark.primary,
    paddingMedium: "0.5rem 1rem 0.5rem 1rem",
    paddingLarge: "0.5rem 1.75rem 0.5rem 1.75rem",
    borderRadiusMedium: "10px",
    borderRadiusLarge: "10px",
    heightMedium: "2.5rem",
    heightLarge: "3rem",
    fontWeight: "500",

    // === Tertiary ===
    colorTertiary: colors.light.background,
    textColorTertiary: colors.light.tertiary,
    colorTertiaryHover: colors.light.tertiaryForeground,
    colorTertiaryPressed: colors.light.tertiaryPressed,
  

    // === Quaternary ===
    colorQuaternaryHover: colors.dark.primaryDark,
    colorQuaternaryPressed: colors.dark.primary,
    color: colors.dark.foreground,
    

    // === Default ===
    textColor: colors.dark.foreground,
  }
}
