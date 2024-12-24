import { Button } from '@/components/ui/button'

interface TitleBarButtonProps {
    handleFunction: () => void
    icon: React.ReactNode
}

const TitleBarButton: React.FC<TitleBarButtonProps> = ({
    handleFunction,
    icon,
}) => {
    return (
        <Button size="icon" variant="ghost" onClick={handleFunction}>
            {icon}
        </Button>
    )
}

export default TitleBarButton
