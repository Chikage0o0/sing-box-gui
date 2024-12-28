import Logo from '@/components/icon'
import { toast } from '@/hooks/use-toast'
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'

const bgColor = (status: string) => {
    switch (status) {
        case 'success':
            return 'bg-primary'
        case 'error':
            return 'bg-destructive'
        default:
            return 'bg-secondary'
    }
}

const Home: React.FC = () => {
    type Status = 'loading' | 'success' | 'error' | 'idle'

    const [status, setStatus] = useState<Status>('idle')

    const onLogoClick = async () => {
        if (
            status === 'success' ||
            status === 'loading' ||
            status === 'error'
        ) {
            // stop
            try {
                await invoke('stop')
                setStatus('idle')
            } catch (error: any) {
                console.error(error)
                toast({
                    title: '停止失败',
                    description: error,
                    variant: 'destructive',
                })
            }
        }

        if (status === 'error' || status === 'idle') {
            // start
            try {
                await invoke('start')
                setStatus('loading')
            } catch (error: any) {
                console.error(error)
                setStatus('error')
                toast({
                    title: '启动失败',
                    description: error,
                    variant: 'destructive',
                })
            }
        }
    }

    const getStatus = async () => {
        try {
            const status = await invoke('get_status')
            setStatus(status as Status)
        } catch (error: any) {
            console.error(error)
            setStatus('error')
        }
    }

    useEffect(() => {
        // 挂载时获取状态，并且设置1s定时器获取，在组件卸载时清除定时器
        getStatus()
        const timer = setInterval(() => {
            getStatus()
        }, 1000)
        return () => {
            clearInterval(timer)
        }
    }, [])

    return (
        <div className="flex items-center justify-center h-full">
            <div
                className={`w-36 h-36 rounded-full flex items-center justify-center  ${bgColor(
                    status
                )}`}
                onClick={onLogoClick}>
                <Logo className="w-28 h-28" />
            </div>
        </div>
    )
}

export default Home
