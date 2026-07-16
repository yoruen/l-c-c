import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { motion } from 'framer-motion';
import { 
  Cpu, 
  HardDrive, 
  Network, 
  Shield, 
  Package, 
  Activity,
  AlertCircle
} from 'lucide-react';
import { AreaChart, Area, ResponsiveContainer, XAxis, YAxis, Tooltip } from 'recharts';

interface DashboardData {
  cpu_percent: number;
  memory_percent: number;
  disk_usage: Record<string, number>;
  network: { download_mbps: number; upload_mbps: number };
  updates_available: number;
  active_services: number;
  security_alerts: number;
}

const mockHistoryData = Array.from({ length: 20 }, (_, i) => ({
  time: i,
  cpu: 30 + Math.random() * 40,
  memory: 50 + Math.random() * 20,
}));

function MetricCard({ 
  title, 
  value, 
  subtitle, 
  icon: Icon, 
  color,
  trend 
}: { 
  title: string; 
  value: string; 
  subtitle?: string;
  icon: React.ElementType;
  color: string;
  trend?: 'up' | 'down' | 'neutral';
}) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="bg-slate-900/50 border border-slate-800 rounded-xl p-6 hover:border-slate-700 transition-colors"
    >
      <div className="flex items-start justify-between">
        <div>
          <p className="text-sm font-medium text-slate-400">{title}</p>
          <p className="text-2xl font-bold mt-1">{value}</p>
          {subtitle && (
            <p className={`text-sm mt-1 ${
              trend === 'up' ? 'text-emerald-400' : 
              trend === 'down' ? 'text-rose-400' : 'text-slate-500'
            }`}>
              {subtitle}
            </p>
          )}
        </div>
        <div className={`p-3 rounded-lg bg-${color}-500/10`}>
          <Icon className={`w-6 h-6 text-${color}-400`} />
        </div>
      </div>
    </motion.div>
  );
}

export function Dashboard() {
  const { data: dashboardData } = useQuery({
    queryKey: ['dashboard'],
    queryFn: () => invoke<DashboardData>('get_dashboard_data'),
    refetchInterval: 5000,
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Dashboard</h2>
          <p className="text-slate-400">System overview and real-time metrics</p>
        </div>
        <div className="flex items-center gap-2 text-sm text-slate-400">
          <span className="w-2 h-2 bg-emerald-500 rounded-full animate-pulse" />
          System Healthy
        </div>
      </div>

      {/* Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard
          title="CPU Usage"
          value={`${dashboardData?.cpu_percent.toFixed(1) ?? '--'}%`}
          subtitle="8 cores active"
          icon={Cpu}
          color="blue"
          trend="neutral"
        />
        <MetricCard
          title="Memory"
          value={`${dashboardData?.memory_percent.toFixed(1) ?? '--'}%`}
          subtitle="12.4 GB / 32 GB"
          icon={Activity}
          color="purple"
          trend="neutral"
        />
        <MetricCard
          title="Storage"
          value={`${Object.values(dashboardData?.disk_usage ?? {})[0]?.toFixed(0) ?? '--'}%`}
          subtitle="/ partition"
          icon={HardDrive}
          color="amber"
          trend="up"
        />
        <MetricCard
          title="Network"
          value={`${dashboardData?.network.download_mbps.toFixed(1) ?? '--'} ↓`}
          subtitle={`${dashboardData?.network.upload_mbps.toFixed(1) ?? '--'} Mbps ↑`}
          icon={Network}
          color="emerald"
          trend="neutral"
        />
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <motion.div 
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="lg:col-span-2 bg-slate-900/50 border border-slate-800 rounded-xl p-6"
        >
          <h3 className="font-semibold mb-4">System Resources</h3>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={mockHistoryData}>
                <defs>
                  <linearGradient id="cpuGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#3b82f6" stopOpacity={0}/>
                  </linearGradient>
                  <linearGradient id="memGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#a855f7" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#a855f7" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <XAxis dataKey="time" hide />
                <YAxis hide />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1e293b', 
                    border: '1px solid #334155',
                    borderRadius: '8px'
                  }}
                />
                <Area 
                  type="monotone" 
                  dataKey="cpu" 
                  stroke="#3b82f6" 
                  fillOpacity={1} 
                  fill="url(#cpuGradient)" 
                />
                <Area 
                  type="monotone" 
                  dataKey="memory" 
                  stroke="#a855f7" 
                  fillOpacity={1} 
                  fill="url(#memGradient)" 
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </motion.div>

        <motion.div 
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="space-y-4"
        >
          <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-6">
            <div className="flex items-center gap-3 mb-4">
              <Package className="w-5 h-5 text-blue-400" />
              <h3 className="font-semibold">Updates</h3>
            </div>
            <div className="flex items-baseline gap-2">
              <span className="text-3xl font-bold">{dashboardData?.updates_available ?? 0}</span>
              <span className="text-slate-400">packages available</span>
            </div>
            <button className="mt-4 w-full py-2 bg-blue-500 hover:bg-blue-600 rounded-lg text-sm font-medium transition-colors">
              Review Updates
            </button>
          </div>

          <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-6">
            <div className="flex items-center gap-3 mb-4">
              <Shield className="w-5 h-5 text-emerald-400" />
              <h3 className="font-semibold">Security</h3>
            </div>
            {dashboardData?.security_alerts === 0 ? (
              <div className="flex items-center gap-2 text-emerald-400">
                <Shield className="w-5 h-5" />
                <span>No security alerts</span>
              </div>
            ) : (
              <div className="flex items-center gap-2 text-amber-400">
                <AlertCircle className="w-5 h-5" />
                <span>{dashboardData?.security_alerts} alerts</span>
              </div>
            )}
          </div>

          <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-6">
            <div className="flex items-center gap-3 mb-4">
              <SettingsIcon className="w-5 h-5 text-purple-400" />
              <h3 className="font-semibold">Services</h3>
            </div>
            <div className="flex items-baseline gap-2">
              <span className="text-3xl font-bold">{dashboardData?.active_services ?? 0}</span>
              <span className="text-slate-400">active</span>
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
