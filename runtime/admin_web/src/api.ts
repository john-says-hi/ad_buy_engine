export interface SetupStatus {
  setup_complete: boolean;
}

export interface SessionState {
  authenticated: boolean;
  username: string | null;
  setup_complete: boolean;
}

export interface Campaign {
  id: string;
  name: string;
  slug: string;
  destination_url: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface StatsSummary {
  total_clicks: number;
  active_campaigns: number;
}

export interface VersionInfo {
  name: string;
  version: string;
}

export interface MaintenanceStatus {
  service: string;
  database: string;
  update_channel: string;
}

export interface CampaignInput {
  name: string;
  slug: string;
  destination_url: string;
  is_active: boolean;
}

export interface SetupInput {
  setup_secret: string;
  username: string;
  password: string;
  tracking_domain: string | null;
}

export interface LoginInput {
  username: string;
  password: string;
}

export async function getSetupStatus(): Promise<SetupStatus> {
  return requestJson<SetupStatus>("/api/setup/status");
}

export async function completeSetup(input: SetupInput): Promise<SessionState> {
  return requestJson<SessionState>("/api/setup/complete", {
    method: "POST",
    body: JSON.stringify(input)
  });
}

export async function login(input: LoginInput): Promise<SessionState> {
  return requestJson<SessionState>("/api/auth/login", {
    method: "POST",
    body: JSON.stringify(input)
  });
}

export async function logout(): Promise<SessionState> {
  return requestJson<SessionState>("/api/auth/logout", {
    method: "POST",
    body: JSON.stringify({})
  });
}

export async function getSession(): Promise<SessionState> {
  return requestJson<SessionState>("/api/session");
}

export async function getCampaigns(): Promise<Campaign[]> {
  return requestJson<Campaign[]>("/api/campaigns");
}

export async function createCampaign(input: CampaignInput): Promise<Campaign> {
  return requestJson<Campaign>("/api/campaigns", {
    method: "POST",
    body: JSON.stringify(input)
  });
}

export async function updateCampaign(id: string, input: CampaignInput): Promise<Campaign> {
  return requestJson<Campaign>(`/api/campaigns/${id}`, {
    method: "PUT",
    body: JSON.stringify(input)
  });
}

export async function disableCampaign(id: string): Promise<Campaign> {
  return requestJson<Campaign>(`/api/campaigns/${id}`, {
    method: "DELETE"
  });
}

export async function getStatsSummary(): Promise<StatsSummary> {
  return requestJson<StatsSummary>("/api/stats/summary");
}

export async function getVersion(): Promise<VersionInfo> {
  return requestJson<VersionInfo>("/api/version");
}

export async function getMaintenanceStatus(): Promise<MaintenanceStatus> {
  return requestJson<MaintenanceStatus>("/api/maintenance/status");
}

async function requestJson<T>(path: string, init: RequestInit = {}): Promise<T> {
  const response = await fetch(path, {
    credentials: "include",
    headers: {
      "content-type": "application/json",
      ...init.headers
    },
    ...init
  });

  const responseText = await response.text();
  const parsedBody = responseText ? JSON.parse(responseText) : {};

  if (!response.ok) {
    throw new Error(parsedBody.error ?? `Request failed with ${response.status}`);
  }

  return parsedBody as T;
}
