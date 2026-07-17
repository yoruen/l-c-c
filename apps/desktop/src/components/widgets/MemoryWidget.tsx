import { useEffect, useState } from 'react';
import { MemoryStick } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { formatBytes } from '@/lib/utils';

export function MemoryWidget() {
  const [used, setUsed] = useState(12.4 * 1024 * 1024 * 1024); // 12.4 GB
  const [total] = useState(32 * 1024 * 1024 * 1024); // 32 GB
  const [cached, setCached] = useState(4.2 * 1024 * 1024 * 1024);

  const percentage = (used / total) * 100;

  useEffect(() => {
    const interval = setInterval(() => {
      // Simulate memory fluctuation
      const fluctuation = (Math.random() - 0.5) * 0.5 * 1024 * 1024 * 1024;
      setUsed((prev) => Math.max(8 * 1024 * 1024 * 1024, Math.min(28 * 1024 * 1024 * 1024, prev + fluctuation)));
      setCached((prev) => Math.max(2 * 1024 * 1024 * 1024, prev + (Math.random() - 0.5) * 0.2 * 1024 * 1024 * 1024));
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  const getUsageColor = (pct: number) => {
    if (pct < 60) return 'bg-blue-500';
    if (pct < 85) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">Memory</CardTitle>
        <MemoryStick className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">
          {formatBytes(used)} / {formatBytes(total)}
        </div>
        
        <div className="mt-3 space-y-2">
          <div className="h-2 bg-muted rounded-full overflow-hidden">
            <div
              className={`h-full transition-all duration-500 ${getUsageColor(percentage)}`}
              style={{ width: `${percentage}%` }}
            />
          </div>
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>{percentage.toFixed(1)}% used</span>
            <span>{formatBytes(cached)} cached</span>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
