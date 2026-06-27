import {
  Activity,
  BarChart3,
  ExternalLink,
  LogOut,
  Plus,
  RefreshCw,
  Save,
  ShieldCheck,
  Trash2
} from "lucide-react";
import { FormEvent, useEffect, useMemo, useState } from "react";
import {
  Campaign,
  CampaignInput,
  MaintenanceStatus,
  SessionState,
  StatsSummary,
  VersionInfo,
  completeSetup,
  createCampaign,
  disableCampaign,
  getCampaigns,
  getMaintenanceStatus,
  getSession,
  getStatsSummary,
  getVersion,
  login,
  logout,
  updateCampaign
} from "./api";

const emptyCampaign: CampaignInput = {
  name: "",
  slug: "",
  destination_url: "",
  is_active: true
};

export function App() {
  const [session, setSession] = useState<SessionState | null>(null);
  const [campaigns, setCampaigns] = useState<Campaign[]>([]);
  const [stats, setStats] = useState<StatsSummary | null>(null);
  const [version, setVersion] = useState<VersionInfo | null>(null);
  const [maintenance, setMaintenance] = useState<MaintenanceStatus | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    void loadSession();
  }, []);

  useEffect(() => {
    if (session?.authenticated) {
      void loadDashboard();
    }
  }, [session?.authenticated]);

  async function loadSession() {
    setLoading(true);
    setError(null);
    try {
      const nextSession = await getSession();
      setSession(nextSession);
    } catch (caughtError) {
      setError(errorMessage(caughtError));
    } finally {
      setLoading(false);
    }
  }

  async function loadDashboard() {
    setError(null);
    try {
      const [nextCampaigns, nextStats, nextVersion, nextMaintenance] = await Promise.all([
        getCampaigns(),
        getStatsSummary(),
        getVersion(),
        getMaintenanceStatus()
      ]);
      setCampaigns(nextCampaigns);
      setStats(nextStats);
      setVersion(nextVersion);
      setMaintenance(nextMaintenance);
    } catch (caughtError) {
      setError(errorMessage(caughtError));
    }
  }

  async function handleLogout() {
    const nextSession = await logout();
    setSession(nextSession);
    setCampaigns([]);
    setStats(null);
  }

  if (loading) {
    return <Shell status="Loading" error={error} />;
  }

  if (!session?.setup_complete) {
    return (
      <Shell status="Setup" error={error}>
        <SetupPanel
          onComplete={(nextSession) => {
            setSession(nextSession);
            void loadDashboard();
          }}
          onError={setError}
        />
      </Shell>
    );
  }

  if (!session.authenticated) {
    return (
      <Shell status="Login" error={error}>
        <LoginPanel
          onLogin={(nextSession) => {
            setSession(nextSession);
            void loadDashboard();
          }}
          onError={setError}
        />
      </Shell>
    );
  }

  return (
    <Shell status={session.username ?? "Admin"} error={error}>
      <div className="toolbar">
        <div>
          <p className="eyebrow">Ad Buy Engine</p>
          <h1>Campaign Control</h1>
        </div>
        <div className="toolbarActions">
          <button className="iconButton" type="button" onClick={() => void loadDashboard()} title="Refresh">
            <RefreshCw size={18} />
          </button>
          <button className="iconButton" type="button" onClick={() => void handleLogout()} title="Log out">
            <LogOut size={18} />
          </button>
        </div>
      </div>

      <StatusGrid stats={stats} version={version} maintenance={maintenance} />
      <CampaignWorkspace
        campaigns={campaigns}
        onChanged={() => void loadDashboard()}
        onError={setError}
      />
    </Shell>
  );
}

function Shell({
  status,
  error,
  children
}: {
  status: string;
  error: string | null;
  children?: React.ReactNode;
}) {
  return (
    <main className="appShell">
      <header className="topbar">
        <div className="brandMark">ABE</div>
        <div>
          <strong>Ad Buy Engine</strong>
          <span>{status}</span>
        </div>
      </header>
      {error ? <div className="errorBanner">{error}</div> : null}
      {children ?? null}
    </main>
  );
}

function SetupPanel({
  onComplete,
  onError
}: {
  onComplete: (session: SessionState) => void;
  onError: (message: string | null) => void;
}) {
  const [setupSecret, setSetupSecret] = useState("");
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("");
  const [trackingDomain, setTrackingDomain] = useState("");
  const [saving, setSaving] = useState(false);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSaving(true);
    onError(null);

    try {
      const nextSession = await completeSetup({
        setup_secret: setupSecret,
        username,
        password,
        tracking_domain: trackingDomain.trim() ? trackingDomain.trim() : null
      });
      onComplete(nextSession);
    } catch (caughtError) {
      onError(errorMessage(caughtError));
    } finally {
      setSaving(false);
    }
  }

  return (
    <section className="centerPanel">
      <div className="panelHeader">
        <ShieldCheck size={24} />
        <h1>First Run Setup</h1>
      </div>
      <form className="formGrid" onSubmit={(event) => void handleSubmit(event)}>
        <label>
          Setup Secret
          <input
            value={setupSecret}
            onChange={(event) => setSetupSecret(event.target.value)}
            required
            autoComplete="one-time-code"
          />
        </label>
        <label>
          Admin Username
          <input
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            required
            minLength={3}
            autoComplete="username"
          />
        </label>
        <label>
          Admin Password
          <input
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            required
            minLength={12}
            autoComplete="new-password"
          />
        </label>
        <label>
          Tracking Domain
          <input
            value={trackingDomain}
            onChange={(event) => setTrackingDomain(event.target.value)}
            placeholder="track.example.com"
            autoComplete="url"
          />
        </label>
        <button className="primaryButton" type="submit" disabled={saving}>
          <Save size={18} />
          {saving ? "Saving" : "Save Setup"}
        </button>
      </form>
    </section>
  );
}

function LoginPanel({
  onLogin,
  onError
}: {
  onLogin: (session: SessionState) => void;
  onError: (message: string | null) => void;
}) {
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("");

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    onError(null);
    try {
      onLogin(await login({ username, password }));
    } catch (caughtError) {
      onError(errorMessage(caughtError));
    }
  }

  return (
    <section className="centerPanel compactPanel">
      <div className="panelHeader">
        <ShieldCheck size={24} />
        <h1>Admin Login</h1>
      </div>
      <form className="formGrid" onSubmit={(event) => void handleSubmit(event)}>
        <label>
          Username
          <input
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            required
            autoComplete="username"
          />
        </label>
        <label>
          Password
          <input
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            required
            autoComplete="current-password"
          />
        </label>
        <button className="primaryButton" type="submit">
          <ShieldCheck size={18} />
          Log In
        </button>
      </form>
    </section>
  );
}

function StatusGrid({
  stats,
  version,
  maintenance
}: {
  stats: StatsSummary | null;
  version: VersionInfo | null;
  maintenance: MaintenanceStatus | null;
}) {
  return (
    <section className="statusGrid">
      <MetricTile icon={<Activity size={20} />} label="Clicks" value={stats?.total_clicks ?? 0} />
      <MetricTile icon={<BarChart3 size={20} />} label="Active" value={stats?.active_campaigns ?? 0} />
      <MetricTile icon={<ShieldCheck size={20} />} label="Version" value={version?.version ?? "0.1.0"} />
      <MetricTile icon={<RefreshCw size={20} />} label="Updates" value={maintenance?.update_channel ?? "github"} />
    </section>
  );
}

function MetricTile({
  icon,
  label,
  value
}: {
  icon: React.ReactNode;
  label: string;
  value: string | number;
}) {
  return (
    <div className="metricTile">
      <div className="metricIcon">{icon}</div>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function CampaignWorkspace({
  campaigns,
  onChanged,
  onError
}: {
  campaigns: Campaign[];
  onChanged: () => void;
  onError: (message: string | null) => void;
}) {
  const [draft, setDraft] = useState<CampaignInput>(emptyCampaign);
  const [editingId, setEditingId] = useState<string | null>(null);
  const editingCampaign = useMemo(
    () => campaigns.find((campaign) => campaign.id === editingId) ?? null,
    [campaigns, editingId]
  );

  useEffect(() => {
    if (editingCampaign) {
      setDraft({
        name: editingCampaign.name,
        slug: editingCampaign.slug,
        destination_url: editingCampaign.destination_url,
        is_active: editingCampaign.is_active
      });
    }
  }, [editingCampaign]);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    onError(null);
    try {
      if (editingId) {
        await updateCampaign(editingId, draft);
      } else {
        await createCampaign(draft);
      }
      setDraft(emptyCampaign);
      setEditingId(null);
      onChanged();
    } catch (caughtError) {
      onError(errorMessage(caughtError));
    }
  }

  async function handleDisable(campaignId: string) {
    onError(null);
    try {
      await disableCampaign(campaignId);
      onChanged();
    } catch (caughtError) {
      onError(errorMessage(caughtError));
    }
  }

  return (
    <section className="workspaceGrid">
      <form className="campaignForm" onSubmit={(event) => void handleSubmit(event)}>
        <div className="panelHeader">
          <Plus size={22} />
          <h2>{editingId ? "Edit Campaign" : "New Campaign"}</h2>
        </div>
        <label>
          Name
          <input
            value={draft.name}
            onChange={(event) => setDraft({ ...draft, name: event.target.value })}
            required
          />
        </label>
        <label>
          Slug
          <input
            value={draft.slug}
            onChange={(event) => setDraft({ ...draft, slug: event.target.value })}
            required
            pattern="[A-Za-z0-9_-]{3,}"
          />
        </label>
        <label>
          Destination URL
          <input
            value={draft.destination_url}
            onChange={(event) => setDraft({ ...draft, destination_url: event.target.value })}
            required
            type="url"
          />
        </label>
        <label className="toggleRow">
          <input
            type="checkbox"
            checked={draft.is_active}
            onChange={(event) => setDraft({ ...draft, is_active: event.target.checked })}
          />
          Active
        </label>
        <button className="primaryButton" type="submit">
          <Save size={18} />
          {editingId ? "Save Campaign" : "Create Campaign"}
        </button>
      </form>

      <div className="campaignTableShell">
        <div className="tableHeader">
          <h2>Campaigns</h2>
          <span>{campaigns.length}</span>
        </div>
        <div className="campaignTable" role="table">
          <div className="campaignRow tableHead" role="row">
            <span>Name</span>
            <span>Tracking Link</span>
            <span>Status</span>
            <span>Actions</span>
          </div>
          {campaigns.map((campaign) => (
            <div className="campaignRow" role="row" key={campaign.id}>
              <button
                className="linkButton"
                type="button"
                onClick={() => setEditingId(campaign.id)}
              >
                {campaign.name}
              </button>
              <a href={`/c/${campaign.slug}`} target="_blank" rel="noreferrer">
                /c/{campaign.slug}
                <ExternalLink size={14} />
              </a>
              <span className={campaign.is_active ? "statusActive" : "statusPaused"}>
                {campaign.is_active ? "Active" : "Paused"}
              </span>
              <button
                className="iconButton dangerButton"
                type="button"
                onClick={() => void handleDisable(campaign.id)}
                title="Disable campaign"
              >
                <Trash2 size={16} />
              </button>
            </div>
          ))}
          {campaigns.length === 0 ? <div className="emptyState">No campaigns</div> : null}
        </div>
      </div>
    </section>
  );
}

function errorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  return "Request failed";
}
