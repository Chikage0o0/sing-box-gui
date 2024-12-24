import AppRoutes from '@/routes'
import ThemeProvider from '@/providers/ThemeProvider'
import LayoutProvider from '@/providers/LayoutProvider'

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
