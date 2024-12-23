import AppRoutes from '@/routes'
import { NavigateOptions } from 'react-router'
import ThemeProvider from '@/providers/ThemeProvider'
import LayoutProvider from '@/providers/LayoutProvider'

declare module '@react-types/shared' {
    interface RouterConfig {
        routerOptions: NavigateOptions
    }
}

const App: React.FC = () => {
    return (
        <ThemeProvider>
            <LayoutProvider>
                <AppRoutes />
            </LayoutProvider>
        </ThemeProvider>
    )
}

export default App
