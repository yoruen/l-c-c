import { useEffect, useState } from 'react';
import { HardDrive } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { formatBytes } from '@/lib/utils';

interface DiskInfo {
  mount: string;
  used: number;
  total: number;
  filesystem: string;
}

export function StorageWidget() {
  const [disks, setDisks] = useState<DiskInfo[]>([
    { mount: '/', used: 234 * 1024 * 1024 * 1024, total: 512 * 1024 * 1024 * 1024, filesystem: 'ext4' },
    { mount: '/home', used: 156 * 1024 * 1024 * 1024, total: 1024 * 1024 * 1024 * 1024, filesystem: 'ext4' },
  ]);

  const totalUsed = disks.reduce((acc, d) => acc + d.used, 0);
  const totalSize = disks.reduce((acc, d) => acc + d.total, 0);
  const percentage = (totalUsed / totalSize) * 100;

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">Storage</CardTitle>
        <HardDrive className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">
          {formatBytes(totalUsed)} / {formatBytes(totalSize)}
        </div>
        
        <div className="mt-3 space-y-2">
          <div className="h-2 bg-muted rounded-full overflow-hidden">
            <div
              className="h-full bg-green-500 transition-all duration-500"
              style={{ width: `${percentage}%` }}
            />
          </div>
          
          <div className="space-y-1">
            {disks.map((disk) => {
              const pct = (disk.used / disk.total) * 100;
              return (
                <div key={disk.mount} className="flex justify-between text-xs text-muted-foreground">
                  <span>{disk.mount}</span>
                  <span>{pct.toFixed(1)}%</span>
                </div>
              );
            })}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
