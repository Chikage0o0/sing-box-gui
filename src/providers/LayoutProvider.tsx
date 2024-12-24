import { AppSidebar } from '@/components/app-sidebar'
import TitleBarButton from '@/components/title-bar-button'
import {
    SidebarInset,
    SidebarProvider,
    SidebarTrigger,
} from '@/components/ui/sidebar'
import { useMaximizeStore } from '@/stores/useMaximizableStore'
import { useThemeStore } from '@/stores/useThemeStore'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Maximize, Minimize, Minus, MoonIcon, SunIcon, X } from 'lucide-react'
import { useEffect } from 'react'

interface LayoutProviderProps {
    children: React.ReactNode
}

const LayoutProvider: React.FC<LayoutProviderProps> = ({ children }) => {
    const { theme, toggleTheme } = useThemeStore()
    const { isMaximized, toggleMaximize, initWindowState } = useMaximizeStore()

    const handleMinimize = async () => {
        const appWindow = getCurrentWindow()
        await appWindow.minimize()
    }

    const handleMaximize = () => {
        toggleMaximize()
    }

    useEffect(() => {
        initWindowState()
    }, [])

    const handleClose = () => {
        // 实现关闭窗口的逻辑
        console.log('Close window')
    }

    return (
        <SidebarProvider>
            <AppSidebar />
            <SidebarInset>
                <header className="h-12 flex items-center justify-between px-4 select-none">
                    <div className="flex items-center">
                        <SidebarTrigger className="h-9 w-9" />
                    </div>
                    <div className="w-full h-full" data-tauri-drag-region />
                    {/* 右侧：窗口控制按钮 */}
                    <div className="flex items-center">
                        <TitleBarButton
                            handleFunction={toggleTheme}
                            icon={
                                theme === 'light' ? (
                                    <MoonIcon className="w-4 h-4" />
                                ) : (
                                    <SunIcon className="w-4 h-4" />
                                )
                            }
                        />
                        <TitleBarButton
                            handleFunction={handleMinimize}
                            icon={<Minus className="w-4 h-4" />}
                        />
                        <TitleBarButton
                            handleFunction={handleMaximize}
                            icon={
                                isMaximized ? (
                                    <Minimize className="w-4 h-4" />
                                ) : (
                                    <Maximize className="w-4 h-4" />
                                )
                            }
                        />
                        <TitleBarButton
                            handleFunction={handleClose}
                            icon={<X className="w-4 h-4" />}
                        />
                    </div>
                </header>

                <main className="h-full">{children}</main>
            </SidebarInset>
        </SidebarProvider>
    )
}

export default LayoutProvider
