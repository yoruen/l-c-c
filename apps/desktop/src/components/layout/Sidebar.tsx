import { NavLink } from 'react-router-dom';
import { 
  LayoutDashboard, 
  Package, 
  Settings as SettingsIcon, 
  Activity,
  HardDrive,
  Network,
  Shield,
  FileText,
  Cpu,
  Terminal
} from 'lucide-react';
import { motion } from 'framer-motion';

const navItems = [
  { to: '/', icon: LayoutDashboard, label: 'Dashboard' },
  { to: '/software', icon: Package, label: 'Software' },
  { to: '/services', icon: SettingsIcon, label: 'Services' },
  { to: '/processes', icon: Activity, label: 'Processes' },
  { to: '/storage', icon: HardDrive, label: 'Storage' },
  { to: '/network', icon: Network, label: 'Network' },
  { to: '/security', icon: Shield, label: 'Security' },
  { to: '/logs', icon: FileText, label: 'Logs' },
];

export function Sidebar() {
  return (
    <motion.aside 
      initial={{ x: -100, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      className="w-64 bg-slate-900 border-r border-slate-800 flex flex-col"
    >
      <div className="p-6 flex items-center gap-3">
        <div className="w-10 h-10 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center">
          <Cpu className="w-6 h-6 text-white" />
        </div>
        <div>
          <h1 className="font-bold text-lg leading-tight">Linux Control</h1>
          <p className="text-xs text-slate-400">Center</p>
        </div>
      </div>
      
      <nav className="flex-1 px-3 space-y-1">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? 'bg-blue-500/10 text-blue-400 border border-blue-500/20'
                  : 'text-slate-400 hover:text-slate-100 hover:bg-slate-800'
              }`
            }
          >
            <item.icon className="w-5 h-5" />
            {item.label}
          </NavLink>
        ))}
      </nav>
      
      <div className="p-4 border-t border-slate-800">
        <button className="w-full flex items-center gap-3 px-3 py-2 text-sm text-slate-400 hover:text-slate-100 transition-colors">
          <Terminal className="w-5 h-5" />
          Terminal
        </button>
      </div>
    </motion.aside>
  );
}