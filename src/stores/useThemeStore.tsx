import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface ThemeStore {
    theme: 'light' | 'dark'
    toggleTheme: () => void
    setTheme: (theme: 'light' | 'dark') => void
}

export const useThemeStore = create<ThemeStore>()(
    persist(
        (set) => ({
            theme:
                typeof window !== 'undefined' &&
                window.matchMedia('(prefers-color-scheme: dark)').matches
                    ? 'dark'
                    : 'light',
            toggleTheme: () =>
                set((state) => ({
                    theme: state.theme === 'light' ? 'dark' : 'light',
                })),
            setTheme: (theme) => set({ theme }),
        }),
        {
            name: 'theme-storage',
        }
    )
)
