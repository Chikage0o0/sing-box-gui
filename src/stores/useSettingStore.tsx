import { create } from 'zustand'
import { z } from 'zod'
import { invoke } from '@tauri-apps/api/core'
import { toast } from '@/hooks/use-toast'

// 定义表单数据类型
export const formSchema = z.object({
    server: z.object({
        subscribe_url: z.string().url('请输入正确的URL'),
    }),
    client: z.object({
        log_level: z.enum(['tracing', 'debug', 'info', 'warning', 'error']),
        auto_start: z.boolean(),
        silent_start: z.boolean(),
    }),
})

type SettingData = z.infer<typeof formSchema>

interface SettingStore {
    settingData: SettingData
    setSetting: (data: Partial<SettingData>) => Promise<void>
    initSetting: () => Promise<void>
}

// 创建store
export const useSettingStore = create<SettingStore>((set, get) => ({
    settingData: {
        server: {
            subscribe_url: '',
        },
        client: {
            auto_start: false,
            silent_start: false,
            log_level: 'info',
        },
    },
    setSetting: async (new_data) => {
        // 更新store中的值
        set((state) => {
            return {
                settingData: {
                    ...state.settingData,
                    ...new_data,
                },
            }
        })
        // 这里可以将数据提交给后端
        try {
            await invoke('set_setting', { setting: get().settingData })
            toast({
                title: '保存设置成功',
            })
        } catch (error: any) {
            console.error(error)
            toast({
                title: '保存设置失败',
                description: error,
                variant: 'destructive',
            })
        }
    },
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
