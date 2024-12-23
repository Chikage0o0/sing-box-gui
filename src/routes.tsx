import { Routes, Route } from 'react-router'
import Home from '@/pages/Home'

const AppRoutes: React.FC = () => (
    <Routes>
        <Route path="/" element={<Home />} />
    </Routes>
)

export default AppRoutes
