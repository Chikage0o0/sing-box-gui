import { HomeIcon, SettingsIcon } from 'lucide-react'

const MenuItems = [
    {
        title: 'Home',
        icon: HomeIcon,
        link: '/',
    },
    {
        title: 'Settings',
        icon: SettingsIcon,
        link: '/settings',
    },
]

interface MenuProps {
    sidebarCollapsed: boolean
}

const Menu: React.FC<MenuProps> = ({ sidebarCollapsed }) => {
    if (sidebarCollapsed) {
        return (
            <div className="space-y-4">
                {MenuItems.map((item, index) => (
                    <div className="w-8 h-8 bg-onehalflight-sidebar-item dark:bg-onehalfdark-sidebar-item rounded-full flex items-center justify-center">
                        <item.icon className="w-4 h-4" />
                    </div>
                ))}
            </div>
        )
    } else {
        return (
            <div className="space-y-4">
                {MenuItems.map((item, index) => (
                    <div className="h-8 bg-onehalflight-sidebar-item dark:bg-onehalfdark-sidebar-item rounded flex items-center px-4">
                        <span>
                            <item.icon className="w-4 h-4" />
                        </span>
                    </div>
                ))}
            </div>
        )
    }
}

export default Menu
