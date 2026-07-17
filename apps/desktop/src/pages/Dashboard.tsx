import { useQuery } from '@tanstack/react-query';
import { CpuWidget } from '@/components/widgets/CpuWidget';
import { MemoryWidget } from '@/components/widgets/MemoryWidget';
import { StorageWidget } from '@/components/widgets/StorageWidget';
import { UpdatesWidget } from '@/components/widgets/UpdatesWidget';
import { NetworkWidget } from '@/components/widgets/NetworkWidget';
import { getSystemInfo, getDashboardData } from '@/lib/api';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Activity, Shield, Server, AlertCircle } from 'lucide-react';

export function Dashboard() {
  const { data: systemInfo, isLoading: isLoadingSystem } = useQuery({
    queryKey: ['system-info'],
    queryFn: getSystemInfo,
  });

  const { data: dashboardData, isLoading: isLoadingDashboard } = useQuery({
    queryKey: ['dashboard-data'],
    queryFn: getDashboardData,
    refetchInterval: 5000, // Refresh every 5 seconds
  });

  if (isLoadingSystem || isLoadingDashboard) {
    return (
      <div className="p-6 space-y-6">
        <div className="h-8 w-64 bg-muted rounded animate-pulse" />
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {[...Array(5)].map((_, i) => (
            <div key={i} className="h-32 bg-muted rounded animate-pulse" />
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground mt-1">
          {systemInfo?.distribution} {systemInfo?.version} • {systemInfo?.hostname} • Kernel {systemInfo?.kernel}
        </p>
      </div>

      {/* Widget Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <CpuWidget />
        <MemoryWidget />
        <StorageWidget />
        <UpdatesWidget />
        <NetworkWidget />
      </div>

      {/* Additional Info Cards */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Services</CardTitle>
            <Server className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{dashboardData?.active_services || 0}</div>
            <p className="text-xs text-muted-foreground">systemd units running</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Security Status</CardTitle>
            <Shield className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="flex items-center gap-2">
              {dashboardData?.security_alerts === 0 ? (
                <>
                  <div className="h-2 w-2 rounded-full bg-green-500" />
                  <span className="text-green-500 font-medium">Protected</span>
                </>
              ) : (
                <>
                  <AlertCircle className="h-4 w-4 text-red-500" />
                  <span className="text-red-500 font-medium">{dashboardData?.security_alerts} alerts</span>
                </>
              )}
            </div>
            <p className="text-xs text-muted-foreground mt-1">Last scan: 2 hours ago</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">System Activity</CardTitle>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-sm text-muted-foreground space-y-1">
              <p>System uptime: 3 days, 12 hours</p>
              <p>Last update: 2 days ago</p>
              <p>Pending tasks: 0</p>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
