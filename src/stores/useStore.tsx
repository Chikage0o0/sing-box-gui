import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

// 1. 创建 store 时使用 typescript 类型
interface StoreState {
    count: number
    user: {
        name: string
        age: number
    } | null
    increment: () => void
    decrement: () => void
    setUser: (user: { name: string; age: number }) => void
    reset: () => void
}

// 2. 使用 middleware 增强功能
const useStore = create<StoreState>()(
    devtools(
        persist(
            (set) => ({
                // 3. 状态集中管理
                count: 0,
                user: null,

                // 4. action 使用箭头函数，便于类型推导
                increment: () => set((state) => ({ count: state.count + 1 })),
                decrement: () => set((state) => ({ count: state.count - 1 })),
                setUser: (user) => set({ user }),
                reset: () => set({ count: 0, user: null }),
            }),
            {
                name: 'app-storage', // 5. 持久化配置
                partialize: (state) => ({ user: state.user }), // 只持久化 user 信息
            }
        )
    )
)

// 6. 创建选择器，优化性能
export const useCount = () => useStore((state) => state.count)
export const useUser = () => useStore((state) => state.user)
export const useActions = () => ({
    increment: useStore((state) => state.increment),
    decrement: useStore((state) => state.decrement),
    setUser: useStore((state) => state.setUser),
    reset: useStore((state) => state.reset),
})

export default useStore
