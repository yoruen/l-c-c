import { Outlet } from 'react-router-dom';
import { Sidebar } from './Sidebar';
import { Header } from './Header';
import { CommandPalette } from '../search/CommandPalette';
import { useState } from 'react';

export function RootLayout() {
  const [commandPaletteOpen, setCommandPaletteOpen] = useState(false);

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex">
      <Sidebar />
      
      <div className="flex-1 flex flex-col min-w-0">
        <Header onOpenCommandPalette={() => setCommandPaletteOpen(true)} />
        
        <main className="flex-1 overflow-auto p-6">
          <Outlet />
        </main>
      </div>
      
      <CommandPalette 
        open={commandPaletteOpen} 
        onClose={() => setCommandPaletteOpen(false)} 
      />
    </div>
  );
}
