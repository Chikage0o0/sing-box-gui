import { FormComponent } from '@/components/settings-form'

const Settings: React.FC = () => {
    return (
        <div className="flex w-full items-center justify-center p-6 md:p-10">
            <div className="w-full max-w-sm">
                <FormComponent />
            </div>
        </div>
    )
}

export default Settings
