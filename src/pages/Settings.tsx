import { LoginForm } from '@/components/settings-form'

const Settings: React.FC = () => {
    return (
        <div className="flex w-full items-center justify-center p-6 md:p-10">
            <div className="w-full max-w-sm">
                <LoginForm />
            </div>
        </div>
    )
}

export default Settings
