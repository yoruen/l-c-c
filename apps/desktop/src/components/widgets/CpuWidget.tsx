import { useEffect, useState } from 'react';
import { Cpu, Activity } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function CpuWidget() {
  const [usage, setUsage] = useState(0);
  const [cores, setCores] = useState(8);
  const [frequency, setFrequency] = useState(3.6);

  useEffect(() => {
    // Simulate real-time CPU data
    const interval = setInterval(() => {
      setUsage(Math.floor(Math.random() * 40) + 20);
      setFrequency(3.2 + Math.random() * 0.8);
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const getUsageColor = (pct: number) => {
    if (pct < 50) return 'bg-green-500';
    if (pct < 80) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">CPU Usage</CardTitle>
        <Cpu className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="flex items-baseline space-x-2">
          <span className="text-3xl font-bold">{usage.toFixed(1)}%</span>
          <Activity className={`h-4 w-4 ${usage > 80 ? 'text-red-500 animate-pulse' : 'text-green-500'}`} />
        </div>
        
        <div className="mt-3 space-y-2">
          <div className="h-2 bg-muted rounded-full overflow-hidden">
            <div
              className={`h-full transition-all duration-500 ${getUsageColor(usage)}`}
              style={{ width: `${usage}%` }}
            />
          </div>
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>{cores} cores</span>
            <span>{frequency.toFixed(2)} GHz</span>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
