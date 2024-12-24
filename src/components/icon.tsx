interface LogoProps {
    width?: number
    height?: number
    className?: string
}

const Logo: React.FC<LogoProps> = ({ className = '' }) => {
    return <img src="/assets/sing-box.svg" className={className} alt="Logo" />
}

export default Logo
