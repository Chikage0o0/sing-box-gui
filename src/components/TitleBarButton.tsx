import { Button } from '@nextui-org/react'

interface TitleBarButtonProps {
    handleFunction: () => void
    icon: React.ReactNode
}

const TitleBarButton: React.FC<TitleBarButtonProps> = ({
    handleFunction,
    icon,
}) => {
    return (
        <Button
            isIconOnly
            size="sm"
            radius="sm"
            variant="light"
            onPress={handleFunction}>
            {icon}
        </Button>
    )
}

export default TitleBarButton
