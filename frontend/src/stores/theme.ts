import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  const isDark = ref(false)

  /** Read persisted preference or fall back to OS preference. */
  function initializeTheme(): void {
    const stored = localStorage.getItem('theme')

    if (stored === 'dark') {
      isDark.value = true
    } else if (stored === 'light') {
      isDark.value = false
    } else {
      isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
    }

    applyTheme()
  }

  /** Toggle between light and dark mode. */
  function toggleTheme(): void {
    isDark.value = !isDark.value
    applyTheme()
  }

  /** Apply the current theme to the document root and persist the choice. */
  function applyTheme(): void {
    const root = document.documentElement

    if (isDark.value) {
      root.classList.add('dark')
      localStorage.setItem('theme', 'dark')
    } else {
      root.classList.remove('dark')
      localStorage.setItem('theme', 'light')
    }
  }

  // Keep the DOM in sync whenever `isDark` changes reactively.
  watch(isDark, () => {
    applyTheme()
  })

  return {
    isDark,
    initializeTheme,
    toggleTheme,
  }
})