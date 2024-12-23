import { Routes, Route } from 'react-router'
import Home from '@/pages/Home'
import Settings from '@/pages/Settings'

const AppRoutes: React.FC = () => (
    <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/settings" element={<Settings />} />
    </Routes>
)

export default AppRoutes
