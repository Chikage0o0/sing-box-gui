import Menu from '@/components/Menu'
import TitleBarButton from '@/components/TitleBarButton'
import { useThemeStore } from '@/stores/useThemeStore'
import { Maximize, Minus, MoonIcon, SunIcon, X } from 'lucide-react'
import { useEffect, useState } from 'react'

interface LayoutProviderProps {
    children: React.ReactNode
}

const LayoutProvider: React.FC<LayoutProviderProps> = ({ children }) => {
    const [sidebarCollapsed, setSidebarCollapsed] = useState(false)
    const { theme, toggleTheme } = useThemeStore()

    const handleMinimize = () => {}

    const handleMaximize = () => {
        // 实现最大化窗口的逻辑
        console.log('Maximize window')
    }

    const handleClose = () => {
        // 实现关闭窗口的逻辑
        console.log('Close window')
    }
    const handleResize = () => {
        if (window.innerWidth < 768) {
            setSidebarCollapsed(true)
        } else {
            setSidebarCollapsed(false)
        }
    }

    useEffect(() => {
        handleResize()
        window.addEventListener('resize', handleResize)
        return () => window.removeEventListener('resize', handleResize)
    }, [])

    return (
        <div className="flex flex-col min-h-screen bg-onehalflight-bg dark:bg-onehalfdark-bg">
            {/* 顶栏 */}
            <header className="h-10 bg-onehalflight-header text-onehalflight-text dark:bg-onehalfdark-header dark:text-onehalfdark-text flex items-center justify-between pl-4 select-none">
                {/* 左侧：图标和标题 */}
                <div className="flex items-center space-x-4">
                    <img
                        src="/assets/sing-box.svg"
                        className="w-4 h-4"
                        alt="Logo"
                    />
                    <h1 className="text-lg font-bold">SingBox</h1>
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
                        icon={<Maximize className="w-4 h-4" />}
                    />
                    <TitleBarButton
                        handleFunction={handleClose}
                        icon={<X className="w-4 h-4" />}
                    />
                </div>
            </header>

            {/* 主体内容 */}
            <div className="flex flex-1">
                {/* 侧边栏 */}
                <aside className="w-16 md:w-64 bg-onehalflight-sidebar dark:bg-onehalfdark-sidebar transition-all duration-300 ease-in-out relative">
                    {/* 侧边栏内容 */}
                    <div className="p-4">
                        <Menu sidebarCollapsed={sidebarCollapsed} />
                    </div>
                </aside>
                <main className="flex-1 p-4 overflow-auto text-onehalflight-text dark:text-onehalfdark-text">
                    {children}
                </main>
            </div>
        </div>
    )
}

export default LayoutProvider
