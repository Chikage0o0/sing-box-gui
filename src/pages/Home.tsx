import { useState } from 'react'

interface LogoProps {
    width?: number
    height?: number
    className?: string
}

const Logo: React.FC<LogoProps> = ({ className = '' }) => {
    return <img src="/assets/sing-box.svg" className={className} alt="Logo" />
}

const bgColor = (status: string) => {
    switch (status) {
        case 'loading':
            return 'bg-yellow-200'
        case 'success':
            return 'bg-green-200'
        case 'error':
            return 'bg-red-200'
        case 'idle':
        default:
            return 'bg-gray-200'
    }
}

const Home: React.FC = () => {
    type Status = 'loading' | 'success' | 'error' | 'idle'

    const [status, setStatus] = useState<Status>('idle')

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
