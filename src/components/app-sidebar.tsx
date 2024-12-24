import * as React from 'react'
import { Settings2, SquareTerminal } from 'lucide-react'

import { NavMain } from '@/components/nav-main'
import { AppHeader } from '@/components/app-header'
import { Sidebar, SidebarContent, SidebarHeader } from '@/components/ui/sidebar'
import Logo from './icon'

// This is sample data.
const data = {
    sidebarHeader: {
        logo: Logo,
        title: 'Sing-Box',
    },
    navMain: [
        {
            title: '主页',
            url: '/',
            icon: SquareTerminal,
        },
        {
            title: '设置',
            url: '/settings',
            icon: Settings2,
        },
    ],
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    return (
        <Sidebar collapsible="icon" {...props}>
            <SidebarHeader>
                <AppHeader item={data.sidebarHeader} />
            </SidebarHeader>
            <SidebarContent>
                <NavMain items={data.navMain} />
            </SidebarContent>
        </Sidebar>
    )
}
