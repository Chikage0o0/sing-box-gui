import { create } from 'zustand'
import { z } from 'zod'
import { invoke } from '@tauri-apps/api/core'

// 定义表单数据类型
export const formSchema = z.object({
    server: z.object({
        subscribe_url: z.string().url('请输入正确的URL'),
    }),
    client: z.object({
        auto_start: z.boolean(),
        silent_start: z.boolean(),
    }),
})

type SettingData = z.infer<typeof formSchema>

interface SettingStore {
    settingData: SettingData
    setSetting: (data: SettingData) => void
    initSetting: () => Promise<void>
}

// 创建store
export const useSettingStore = create<SettingStore>((set) => ({
    settingData: {
        server: {
            subscribe_url: '',
        },
        client: {
            auto_start: false,
            silent_start: false,
        },
    },
    setSetting: (data) => set({ settingData: data }),
    initSetting: async () => {
        // 这里可以初始化表单数据
        try {
            let setting: SettingData = await invoke('get_setting')
            console.log(setting)
            if (setting) {
                set({ settingData: setting })
            }
        } catch (error) {
            console.error(error)
        }
    },
}))
