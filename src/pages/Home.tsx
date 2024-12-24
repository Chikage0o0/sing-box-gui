import Logo from '@/components/icon'
import { useState } from 'react'

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

    const [status] = useState<Status>('loading')

    return (
        <div className="flex items-center justify-center h-full">
            <div
                className={`w-36 h-36 rounded-full flex items-center justify-center  ${bgColor(
                    status
                )}`}>
                <Logo className="w-28 h-28" />
            </div>
        </div>
    )
}

export default Home
