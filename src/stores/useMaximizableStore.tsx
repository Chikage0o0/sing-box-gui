import { create } from 'zustand'
import { getCurrentWindow } from '@tauri-apps/api/window'

interface MaximizeStore {
    isMaximized: boolean
    initWindowState: () => void
    toggleMaximize: () => void
}

export const useMaximizeStore = create<MaximizeStore>((set) => ({
    isMaximized: false,

    // 初始化窗口状态
    initWindowState: async () => {
        const appWindow = getCurrentWindow()
        const maximized = await appWindow.isMaximized()
        set({ isMaximized: maximized })

        // 监听窗口大小变化
        appWindow.onResized(async () => {
            const maximized = await appWindow.isMaximized()
            set({ isMaximized: maximized })
        })
    },

    // 切换窗口状态
    toggleMaximize: async () => {
        const appWindow = getCurrentWindow()
        const isMaximized = await appWindow.isMaximized()
        if (isMaximized) {
            await appWindow.unmaximize()
        } else {
            await appWindow.maximize()
        }
        set({ isMaximized: !isMaximized })
    },
}))
