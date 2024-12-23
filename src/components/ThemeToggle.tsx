import { Button } from '@nextui-org/react'
import { useThemeStore } from '@/stores/useThemeStore'
import { SunIcon, MoonIcon } from 'lucide-react'

export const ThemeToggle = () => {
    const { theme, toggleTheme } = useThemeStore()

    return (
        <Button
            isIconOnly
            variant="light"
            onPress={toggleTheme}
            aria-label="Toggle theme">
            {theme === 'light' ? <MoonIcon /> : <SunIcon />}
        </Button>
    )
}
