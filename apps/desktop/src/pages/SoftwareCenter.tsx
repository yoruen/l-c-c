import { useState } from 'react';
import { Search, Package, Download, Trash2, RotateCw, Check } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface PackageInfo {
  id: string;
  name: string;
  version: string;
  description: string;
  size: string;
  installed: boolean;
  repository: string;
}

const mockPackages: PackageInfo[] = [
  { id: '1', name: 'firefox', version: '120.0', description: 'Mozilla Firefox web browser', size: '75 MB', installed: true, repository: 'main' },
  { id: '2', name: 'code', version: '1.85.0', description: 'Visual Studio Code editor', size: '320 MB', installed: true, repository: 'main' },
  { id: '3', name: 'docker', version: '24.0.7', description: 'Container runtime', size: '150 MB', installed: false, repository: 'main' },
  { id: '4', name: 'nodejs', version: '20.10.0', description: 'JavaScript runtime', size: '45 MB', installed: true, repository: 'main' },
  { id: '5', name: 'rust', version: '1.74.0', description: 'Rust programming language', size: '800 MB', installed: false, repository: 'main' },
  { id: '6', name: 'gimp', version: '2.10.36', description: 'GNU Image Manipulation Program', size: '120 MB', installed: false, repository: 'universe' },
  { id: '7', name: 'vlc', version: '3.0.20', description: 'Media player', size: '45 MB', installed: true, repository: 'main' },
  { id: '8', name: 'postgresql', version: '16.1', description: 'Database server', size: '65 MB', installed: false, repository: 'main' },
];

export function SoftwareCenter() {
  const [search, setSearch] = useState('');
  const [activeTab, setActiveTab] = useState('all');

  const filtered = mockPackages.filter((p) => {
    const matchesSearch = p.name.toLowerCase().includes(search.toLowerCase()) ||
                         p.description.toLowerCase().includes(search.toLowerCase());
    
    if (activeTab === 'installed') return matchesSearch && p.installed;
    if (activeTab === 'updates') return matchesSearch && !p.installed; // Mock: not installed = update available
    return matchesSearch;
  });

  const installedCount = mockPackages.filter(p => p.installed).length;
  const availableCount = mockPackages.filter(p => !p.installed).length;

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Software Center</h1>
          <p className="text-muted-foreground">Manage packages and applications</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline">
            <RotateCw className="h-4 w-4 mr-2" />
            Refresh
          </Button>
          <Button>
            <Download className="h-4 w-4 mr-2" />
            Update All
          </Button>
        </div>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="all">All ({mockPackages.length})</TabsTrigger>
          <TabsTrigger value="installed">Installed ({installedCount})</TabsTrigger>
          <TabsTrigger value="updates">Updates ({availableCount})</TabsTrigger>
        </TabsList>

        <div className="mt-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Search packages..."
              className="pl-10"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
        </div>

        <TabsContent value={activeTab} className="mt-4">
          <div className="grid gap-4">
            {filtered.map((pkg) => (
              <Card key={pkg.id}>
                <CardContent className="p-4 flex items-center justify-between">
                  <div className="flex items-center gap-4">
                    <div className="h-12 w-12 rounded-lg bg-muted flex items-center justify-center">
                      <Package className="h-6 w-6 text-muted-foreground" />
                    </div>
                    <div>
                      <div className="flex items-center gap-2">
                        <h3 className="font-medium">{pkg.name}</h3>
                        <span className="text-xs text-muted-foreground">v{pkg.version}</span>
                        {pkg.installed && (
                          <span className="text-xs bg-green-100 text-green-800 px-2 py-0.5 rounded-full">
                            Installed
                          </span>
                        )}
                      </div>
                      <p className="text-sm text-muted-foreground">{pkg.description}</p>
                      <p className="text-xs text-muted-foreground mt-1">
                        {pkg.size} • {pkg.repository}
                      </p>
                    </div>
                  </div>
                  <div className="flex gap-2">
                    {pkg.installed ? (
                      <>
                        <Button variant="outline" size="sm">
                          <RotateCw className="h-4 w-4 mr-2" />
                          Reinstall
                        </Button>
                        <Button variant="destructive" size="sm">
                          <Trash2 className="h-4 w-4 mr-2" />
                          Remove
                        </Button>
                      </>
                    ) : (
                      <Button size="sm">
                        <Download className="h-4 w-4 mr-2" />
                        Install
                      </Button>
                    )}
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
