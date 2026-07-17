import { useEffect, useState } from 'react';
import { Wifi, Download, Upload } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function NetworkWidget() {
  const [downloadSpeed, setDownloadSpeed] = useState(12.5);
  const [uploadSpeed, setUploadSpeed] = useState(3.2);
  const [totalDownload, setTotalDownload] = useState(45.2 * 1024 * 1024 * 1024);
  const [totalUpload, setTotalUpload] = useState(12.8 * 1024 * 1024 * 1024);

  useEffect(() => {
    const interval = setInterval(() => {
      setDownloadSpeed(Math.max(0, 12.5 + (Math.random() - 0.5) * 10));
      setUploadSpeed(Math.max(0, 3.2 + (Math.random() - 0.5) * 3));
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const formatSpeed = (speed: number) => {
    if (speed >= 1000) {
      return `${(speed / 1000).toFixed(2)} Gbps`;
    }
    return `${speed.toFixed(2)} Mbps`;
  };

  const formatData = (bytes: number) => {
    const gb = bytes / (1024 * 1024 * 1024);
    if (gb >= 1000) {
      return `${(gb / 1000).toFixed(2)} TB`;
    }
    return `${gb.toFixed(2)} GB`;
  };

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">Network</CardTitle>
        <Wifi className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <Download className="h-4 w-4" />
              <span>Download</span>
            </div>
            <div className="text-xl font-bold">{formatSpeed(downloadSpeed)}</div>
            <div className="text-xs text-muted-foreground">
              Total: {formatData(totalDownload)}
            </div>
          </div>
          
          <div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <Upload className="h-4 w-4" />
              <span>Upload</span>
            </div>
            <div className="text-xl font-bold">{formatSpeed(uploadSpeed)}</div>
            <div className="text-xs text-muted-foreground">
              Total: {formatData(totalUpload)}
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
