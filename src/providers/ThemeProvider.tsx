import { useThemeStore } from '@/stores/useThemeStore'
import { useEffect } from 'react'

interface ThemeProviderProps {
    children: React.ReactNode
}

const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
    const { theme, setTheme } = useThemeStore()

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

    return <div className="blur-background">{children}</div>
}

export default ThemeProvider
