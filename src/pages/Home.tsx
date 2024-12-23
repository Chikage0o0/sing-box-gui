interface LogoProps {
    width?: number
    height?: number
    className?: string
}

const Logo: React.FC<LogoProps> = ({
    width = 1027,
    height = 1109,
    className = '',
}) => {
    return (
        <img
            src="/assets/sing-box.svg"
            width={width}
            height={height}
            className={className}
            alt="Logo"
        />
    )
}

const Home: React.FC = () => {
    return (
        <div
            style={{
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                height: '100vh',
            }}>
            <div>
                <Logo width={100} height={100} />
            </div>
        </div>
    )
}

export default Home
