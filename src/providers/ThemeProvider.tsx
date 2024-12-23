import { NextUIProvider } from '@nextui-org/react'
import { useThemeStore } from '../stores/useThemeStore'
import { useEffect } from 'react'
import { useHref, useNavigate } from 'react-router'

interface ThemeProviderProps {
    children: React.ReactNode
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
    const { theme, setTheme } = useThemeStore()
    const navigate = useNavigate()

    useEffect(() => {
        // 同步主题到 document
        document.documentElement.className = theme
    }, [theme])

    useEffect(() => {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
        const handleChange = (e: MediaQueryListEvent) => {
            setTheme(e.matches ? 'dark' : 'light')
        }
        mediaQuery.addEventListener('change', handleChange)
        return () => {
            mediaQuery.removeEventListener('change', handleChange)
        }
    }, [setTheme])

    return (
        <NextUIProvider navigate={navigate} useHref={useHref}>
            <div
                className={`${theme} text-foreground bg-background min-h-screen`}>
                {children}
            </div>
        </NextUIProvider>
    )
}
