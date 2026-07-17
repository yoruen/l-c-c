import { useState } from 'react';
import { Download, Shield, AlertCircle } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

interface UpdateInfo {
  security: number;
  regular: number;
  rebootRequired: boolean;
}

export function UpdatesWidget() {
  const [updates, setUpdates] = useState<UpdateInfo>({
    security: 2,
    regular: 10,
    rebootRequired: false,
  });

  const total = updates.security + updates.regular;

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">Updates</CardTitle>
        <Download className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="flex items-baseline space-x-2">
          <span className="text-3xl font-bold">{total}</span>
          <span className="text-sm text-muted-foreground">available</span>
        </div>

        <div className="mt-3 space-y-2">
          {updates.security > 0 && (
            <div className="flex items-center gap-2 text-sm text-red-500">
              <Shield className="h-4 w-4" />
              <span>{updates.security} security updates</span>
            </div>
          )}
          
          {updates.rebootRequired && (
            <div className="flex items-center gap-2 text-sm text-yellow-500">
              <AlertCircle className="h-4 w-4" />
              <span>Reboot required</span>
            </div>
          )}

          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <span>{updates.regular} regular updates</span>
          </div>
        </div>

        <Button size="sm" className="w-full mt-3">
          Update All
        </Button>
      </CardContent>
    </Card>
  );
}
