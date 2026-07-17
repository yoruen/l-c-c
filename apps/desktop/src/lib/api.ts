import { invoke } from '@tauri-apps/api/core';

// System Info Types
export interface SystemInfo {
  distribution: string;
  version: string;
  codename: string;
  hostname: string;
  kernel: string;
  uptime_seconds: number;
}

// Plugin Types
export interface PluginMetadata {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  capabilities: string[];
  health: 'Healthy' | 'Degraded' | 'Unhealthy' | 'Unknown';
}

// Dashboard Data Types
export interface DashboardData {
  cpu_percent: number;
  memory_percent: number;
  disk_usage: Record<string, number>;
  network: {
    download_mbps: number;
    upload_mbps: number;
  };
  updates_available: number;
  active_services: number;
  security_alerts: number;
}

// Search Types
export interface SearchResult {
  id: string;
  title: string;
  subtitle?: string;
  category: string;
  score: number;
}

// Widget Types
export interface WidgetDefinition {
  id: string;
  name: string;
  description: string;
  plugin_id: string;
}

// API Functions

export async function getSystemInfo(): Promise<SystemInfo> {
  return invoke('get_system_info');
}

export async function getPlugins(): Promise<PluginMetadata[]> {
  return invoke('list_plugins');
}

export async function search(query: string, limit: number = 20): Promise<Array<[string, SearchResult]>> {
  return invoke('search', { query, limit });
}

export async function getWidgets(): Promise<Array<[string, WidgetDefinition]>> {
  return invoke('get_widgets');
}

export async function executeCommand(
  pluginId: string,
  command: string,
  args: Record<string, unknown>
): Promise<unknown> {
  return invoke('execute_command', { pluginId, command, args });
}

export async function getDashboardData(): Promise<DashboardData> {
  return invoke('get_dashboard_data');
}
