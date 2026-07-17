import { useState } from 'react';
import { Play, Square, RotateCw, Power, Search, Terminal } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface ServiceInfo {
  id: string;
  name: string;
  description: string;
  status: 'active' | 'inactive' | 'failed' | 'activating';
  enabled: boolean;
  loaded: boolean;
  uptime?: string;
}

const mockServices: ServiceInfo[] = [
  { id: '1', name: 'sshd', description: 'OpenSSH server', status: 'active', enabled: true, loaded: true, uptime: '3 days' },
  { id: '2', name: 'docker', description: 'Docker Application Container Engine', status: 'active', enabled: true, loaded: true, uptime: '3 days' },
  { id: '3', name: 'nginx', description: 'A high performance web server', status: 'inactive', enabled: false, loaded: true },
  { id: '4', name: 'postgresql', description: 'PostgreSQL database server', status: 'active', enabled: true, loaded: true, uptime: '2 days' },
  { id: '5', name: 'cups', description: 'CUPS Printing Service', status: 'inactive', enabled: true, loaded: true },
  { id: '6', name: 'bluetooth', description: 'Bluetooth service', status: 'active', enabled: true, loaded: true, uptime: '3 days' },
  { id: '7', name: 'NetworkManager', description: 'Network Manager', status: 'active', enabled: true, loaded: true, uptime: '3 days' },
  { id: '8', name: 'fail2ban', description: 'Fail2Ban Service', status: 'active', enabled: true, loaded: true, uptime: '1 week' },
];

export function ServiceManager() {
  const [search, setSearch] = useState('');
  const [activeTab, setActiveTab] = useState('all');

  const filtered = mockServices.filter((s) => {
    const matchesSearch = s.name.toLowerCase().includes(search.toLowerCase()) ||
                         s.description.toLowerCase().includes(search.toLowerCase());
    
    if (activeTab === 'active') return matchesSearch && s.status === 'active';
    if (activeTab === 'inactive') return matchesSearch && s.status === 'inactive';
    return matchesSearch;
  });

  const getStatusColor = (status: ServiceInfo['status']) => {
    switch (status) {
      case 'active': return 'text-green-500 bg-green-100';
      case 'inactive': return 'text-gray-500 bg-gray-100';
      case 'failed': return 'text-red-500 bg-red-100';
      case 'activating': return 'text-yellow-500 bg-yellow-100';
      default: return 'text-gray-500 bg-gray-100';
    }
  };

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Service Manager</h1>
          <p className="text-muted-foreground">Manage systemd services</p>
        </div>
        <Button variant="outline">
          <RotateCw className="h-4 w-4 mr-2" />
          Refresh
        </Button>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="all">All ({mockServices.length})</TabsTrigger>
          <TabsTrigger value="active">
            Active ({mockServices.filter(s => s.status === 'active').length})
          </TabsTrigger>
          <TabsTrigger value="inactive">
            Inactive ({mockServices.filter(s => s.status === 'inactive').length})
          </TabsTrigger>
        </TabsList>

        <div className="mt-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Search services..."
              className="pl-10"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
        </div>

        <TabsContent value={activeTab} className="mt-4">
          <div className="grid gap-4">
            {filtered.map((service) => (
              <Card key={service.id}>
                <CardContent className="p-4 flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-3">
                      <h3 className="font-medium">{service.name}</h3>
                      <span className={`text-xs px-2 py-0.5 rounded-full ${getStatusColor(service.status)}`}>
                        {service.status}
                      </span>
                      {service.enabled && (
                        <span className="text-xs text-muted-foreground border px-2 py-0.5 rounded-full">
                          enabled
                        </span>
                      )}
                    </div>
                    <p className="text-sm text-muted-foreground mt-1">{service.description}</p>
                    {service.uptime && (
                      <p className="text-xs text-muted-foreground mt-1">
                        Running for {service.uptime}
                      </p>
                    )}
                  </div>
                  
                  <div className="flex gap-2">
                    <Button variant="outline" size="sm">
                      <Terminal className="h-4 w-4 mr-2" />
                      Logs
                    </Button>
                    
                    {service.status === 'active' ? (
                      <>
                        <Button variant="outline" size="sm">
                          <RotateCw className="h-4 w-4 mr-2" />
                          Restart
                        </Button>
                        <Button variant="destructive" size="sm">
                          <Square className="h-4 w-4 mr-2" />
                          Stop
                        </Button>
                      </>
                    ) : (
                      <Button size="sm">
                        <Play className="h-4 w-4 mr-2" />
                        Start
                      </Button>
                    )}
                    
                    <Button variant="outline" size="sm">
                      <Power className="h-4 w-4 mr-2" />
                      {service.enabled ? 'Disable' : 'Enable'}
                    </Button>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
