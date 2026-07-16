import { createBrowserRouter } from 'react-router-dom';
import { RootLayout } from './components/layout/RootLayout';
import { Dashboard } from './pages/Dashboard';
import { SoftwareCenter } from './pages/SoftwareCenter';
import { Services } from './pages/Services';
import { Processes } from './pages/Processes';
import { Storage } from './pages/Storage';
import { Network } from './pages/Network';
import { Security } from './pages/Security';
import { Logs } from './pages/Logs';
import { Settings } from './pages/Settings';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <RootLayout />,
    children: [
      { index: true, element: <Dashboard /> },
      { path: 'software', element: <SoftwareCenter /> },
      { path: 'services', element: <Services /> },
      { path: 'processes', element: <Processes /> },
      { path: 'storage', element: <Storage /> },
      { path: 'network', element: <Network /> },
      { path: 'security', element: <Security /> },
      { path: 'logs', element: <Logs /> },
      { path: 'settings', element: <Settings /> },
    ],
  },
]);
