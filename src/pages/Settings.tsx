import { useState } from 'react'

const Settings: React.FC = () => {
    type Status = 'loading' | 'success' | 'error' | 'idle'

    const [status, setStatus] = useState<Status>('idle')

    return <div className="flex items-center justify-center h-full"></div>
}

export default Settings
