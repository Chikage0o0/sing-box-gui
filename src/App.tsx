import AppRoutes from '@/routes'
import { NavigateOptions } from 'react-router'
import { ThemeToggle } from '@/components/ThemeToggle'
import { ThemeProvider } from '@/providers/ThemeProvider'

declare module '@react-types/shared' {
    interface RouterConfig {
        routerOptions: NavigateOptions
    }
}

const App: React.FC = () => {
    return (
        <ThemeProvider>
            <div className="fixed top-4 right-4">
                <ThemeToggle />
            </div>
            <AppRoutes />
        </ThemeProvider>
    )
}

export default App
