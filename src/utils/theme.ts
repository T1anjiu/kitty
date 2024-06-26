import { ref, watch } from 'vue'
import { type GlobalThemeOverrides, darkTheme, lightTheme, useOsTheme } from 'naive-ui'
import type { BuiltInGlobalTheme } from 'naive-ui/es/themes/interface'

function useTheme() {
  const osThemeRef = useOsTheme()
  const theme = ref<BuiltInGlobalTheme | null>(null)
  const primaryColor = '#5352ed'
  const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
      primaryColor,
      primaryColorHover: primaryColor,
    },
    Button: {
      textColorPrimary: 'whitesmoke',
      textColorHoverPrimary: 'whitesmoke',
      textColor: primaryColor,
    },
    Menu: {
      // itemColorHover: 'red',
      itemColorActive: primaryColor,
      itemColorActiveHover: primaryColor,
      itemTextColorActive: 'whitesmoke',
      itemTextColorActiveHover: 'white',
      borderRadius: '999px',
    },
    Switch: {
      railColorActive: primaryColor,
    },
  }

  const darkThemeOverrides: GlobalThemeOverrides = {
    common: {
      primaryColor,
      primaryColorHover: primaryColor,
    },
    Button: {
      textColorPrimary: 'whitesmoke',
      textColorHoverPrimary: 'whitesmoke',
      textColor: primaryColor,
    },
    Menu: {
      itemColorActive: primaryColor,
      itemColorActiveHover: primaryColor,
      itemColorHover: '#3b3c55',
      itemTextColor: '#5b7497',
      itemTextColorActive: 'whitesmoke',
      itemTextColorActiveHover: 'white',
      borderRadius: '999px',
    },
    Switch: {
      railColorActive: primaryColor,
    },
  }

  watch(osThemeRef, (value) => {
    if (value === 'dark') {
      document.documentElement.classList.add('dark')
      theme.value = darkTheme
      return
    }
    document.documentElement.classList.remove('dark')
    theme.value = lightTheme
  }, { immediate: true })

  return {
    theme,
    lightThemeOverrides,
    darkThemeOverrides,
  }
}

export { useTheme }
