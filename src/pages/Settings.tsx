import { useState, useEffect } from "react";
import { useSettings, useSaveSettings } from "../hooks/useSettings";
import { UserSettings } from "../types";
import { CountrySelect } from "../components/CountrySelect";

interface SettingsUpdateProps {
  currentVersion: string | null;
  updateAvailable: boolean;
  updateVersion: string | null;
  updateDownloading: boolean;
  updateProgress: number;
  onCheckUpdate: () => void;
}

export function Settings({
  currentVersion,
  updateAvailable,
  updateVersion,
  updateDownloading,
  updateProgress,
  onCheckUpdate,
}: SettingsUpdateProps) {
  const { data: settings, isLoading } = useSettings();
  const save = useSaveSettings();

  // Local form state — mirrors the settings object
  const [form, setForm] = useState<UserSettings>({
    steam_id: null,
    steam_api_key: null,
    itad_api_key: null,
    country_code: "lk",
    min_review_score: 90,
    min_discount_percent: 50,
    sync_interval_hours: 24,
    last_synced_at: null,
    alert_threshold_percent: 50,
  });

  // Populate form when settings load
  useEffect(() => {
    if (settings) setForm(settings);
  }, [settings]);

  const handleSave = () => save.mutate(form);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <span style={{ color: "#5a5f72", fontSize: 13 }}>Loading…</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div
        style={{
          padding: "16px 20px",
          borderBottom: "1px solid #1a1d28",
          flexShrink: 0,
        }}
      >
        <h1 style={{ fontSize: 20, fontWeight: 600, color: "#e0e2e8" }}>
          Settings
        </h1>
        <p style={{ fontSize: 12, color: "#5a5f72", marginTop: 2 }}>
          Configure your Steam ID and preferences
        </p>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto" style={{ padding: "20px" }}>
        <div
          style={{
            maxWidth: 480,
            display: "flex",
            flexDirection: "column",
            gap: 24,
          }}
        >
          {/* Steam Account section */}
          <Section title="Steam Account">
            <Field
              label="Steam ID"
              description={
                <>
                  Your 17-digit Steam ID.{" "}
                  <a
                    href="https://steamcommunity.com/my/profile"
                    target="_blank"
                    rel="noreferrer"
                    style={{ color: "#3d6ef8" }}
                  >
                    Find yours here
                  </a>{" "}
                  — look at the URL, it's the number after /profiles/
                </>
              }
            >
              <input
                type="text"
                placeholder="e.g. 76561198012345678"
                value={form.steam_id ?? ""}
                onChange={(e) =>
                  setForm((f) => ({
                    ...f,
                    steam_id: e.target.value || null,
                  }))
                }
                style={{
                  width: "100%",
                  background: "#1c1e27",
                  border: "1px solid #2a2d3a",
                  borderRadius: 6,
                  padding: "8px 12px",
                  color: "#e0e2e8",
                  fontSize: 13,
                  outline: "none",
                }}
                onFocus={(e) => (e.target.style.borderColor = "#3d6ef8")}
                onBlur={(e) => (e.target.style.borderColor = "#2a2d3a")}
              />
            </Field>

            <Field
              label="Steam API Key"
              description={
                <>
                  Free key from{" "}
                  <a
                    href="https://steamcommunity.com/dev/apikey"
                    target="_blank"
                    rel="noreferrer"
                    style={{ color: "#3d6ef8" }}
                  >
                    steamcommunity.com/dev/apikey
                  </a>{" "}
                  — use "localhost" as the domain name.
                </>
              }
            >
              <input
                type="password"
                placeholder="32-character hex key"
                value={form.steam_api_key ?? ""}
                onChange={(e) =>
                  setForm((f) => ({
                    ...f,
                    steam_api_key: e.target.value || null,
                  }))
                }
                style={{
                  width: "100%",
                  background: "#1c1e27",
                  border: "1px solid #2a2d3a",
                  borderRadius: 6,
                  padding: "8px 12px",
                  color: "#e0e2e8",
                  fontSize: 13,
                  outline: "none",
                }}
                onFocus={(e) => (e.target.style.borderColor = "#3d6ef8")}
                onBlur={(e) => (e.target.style.borderColor = "#2a2d3a")}
              />
            </Field>

            <Field
              label="IsThereAnyDeal API Key"
              description={
                <>
                  Free key from{" "}
                  <a
                    href="https://isthereanydeal.com/apps/my/"
                    target="_blank"
                    rel="noreferrer"
                    style={{ color: "#3d6ef8" }}
                  >
                    isthereanydeal.com/apps/my
                  </a>{" "}
                  — register a free app to get your key.
                </>
              }
            >
              <input
                type="password"
                placeholder="ITAD API key"
                value={form.itad_api_key ?? ""}
                onChange={(e) =>
                  setForm((f) => ({
                    ...f,
                    itad_api_key: e.target.value || null,
                  }))
                }
                style={{
                  width: "100%",
                  background: "#1c1e27",
                  border: "1px solid #2a2d3a",
                  borderRadius: 6,
                  padding: "8px 12px",
                  color: "#e0e2e8",
                  fontSize: 13,
                  outline: "none",
                }}
                onFocus={(e) => (e.target.style.borderColor = "#3d6ef8")}
                onBlur={(e) => (e.target.style.borderColor = "#2a2d3a")}
              />
            </Field>

            <Field
              label="Your Region"
              description="Used for Steam regional prices. Changes take effect on next wishlist sync."
            >
              <CountrySelect
                value={form.country_code}
                onChange={(code) =>
                  setForm((f) => ({ ...f, country_code: code }))
                }
              />
            </Field>

            <InfoBox>
              Your profile visibility must be set to <strong>Public</strong> on
              Steam for wishlist fetching to work.
            </InfoBox>
          </Section>

          {/* Recommendations section */}
          <Section title="Recommendations">
            <SliderField
              label="Minimum review score"
              value={form.min_review_score}
              min={50}
              max={99}
              format={(v) => `${v}%`}
              onChange={(v) => setForm((f) => ({ ...f, min_review_score: v }))}
            />
            <SliderField
              label="Minimum discount to show"
              value={form.min_discount_percent}
              min={10}
              max={90}
              format={(v) => `${v}%`}
              onChange={(v) =>
                setForm((f) => ({ ...f, min_discount_percent: v }))
              }
            />
          </Section>

          {/* Sync section */}
          <Section title="Sync">
            <SliderField
              label="Auto-sync interval"
              value={form.sync_interval_hours}
              min={1}
              max={72}
              format={(v) => `${v}h`}
              onChange={(v) =>
                setForm((f) => ({ ...f, sync_interval_hours: v }))
              }
            />
          </Section>

          {/* Save button */}
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            <button
              onClick={handleSave}
              disabled={save.isPending}
              style={{
                background: "#3d6ef8",
                color: "#fff",
                border: "none",
                borderRadius: 6,
                padding: "8px 20px",
                fontSize: 13,
                fontWeight: 500,
                cursor: save.isPending ? "not-allowed" : "pointer",
                opacity: save.isPending ? 0.7 : 1,
              }}
            >
              {save.isPending ? "Saving…" : "Save settings"}
            </button>

            {save.isSuccess && (
              <span style={{ fontSize: 12, color: "#4ade80" }}>✓ Saved</span>
            )}
            {save.isError && (
              <span style={{ fontSize: 12, color: "#f87171" }}>
                Failed to save
              </span>
            )}
          </div>

          <Section title="About">
            <Field label="Version">
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "space-between",
                }}
              >
                <span style={{ fontSize: 13, color: "#9096a8" }}>
                  Steam Lite {currentVersion ? `v${currentVersion}` : "…"}
                </span>

                {updateAvailable ? (
                  <div
                    style={{ display: "flex", alignItems: "center", gap: 8 }}
                  >
                    {updateDownloading ? (
                      <div
                        style={{
                          display: "flex",
                          alignItems: "center",
                          gap: 8,
                        }}
                      >
                        <div
                          style={{
                            width: 80,
                            height: 4,
                            background: "#1a1d28",
                            borderRadius: 2,
                            overflow: "hidden",
                          }}
                        >
                          <div
                            style={{
                              height: "100%",
                              width: `${updateProgress}%`,
                              background: "#3d6ef8",
                              borderRadius: 2,
                              transition: "width 0.3s ease",
                            }}
                          />
                        </div>
                        <span style={{ fontSize: 12, color: "#3d6ef8" }}>
                          {updateProgress}%
                        </span>
                      </div>
                    ) : (
                      <span
                        style={{
                          fontSize: 12,
                          fontWeight: 600,
                          color: "#4ade80",
                          background: "#14291e",
                          border: "1px solid #16a34a",
                          padding: "3px 10px",
                          borderRadius: 5,
                        }}
                      >
                        v{updateVersion} available
                      </span>
                    )}
                  </div>
                ) : (
                  <button
                    onClick={onCheckUpdate}
                    style={{
                      fontSize: 12,
                      color: "#5a5f72",
                      background: "transparent",
                      border: "1px solid #2a2d3a",
                      borderRadius: 5,
                      padding: "3px 10px",
                      cursor: "pointer",
                    }}
                  >
                    Check for updates
                  </button>
                )}
              </div>
            </Field>
          </Section>
        </div>
      </div>
    </div>
  );
}

// ─── Small reusable sub-components ───────────────────────

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <h2
        style={{
          fontSize: 11,
          fontWeight: 600,
          color: "#5a5f72",
          textTransform: "uppercase",
          letterSpacing: "0.08em",
          marginBottom: 12,
        }}
      >
        {title}
      </h2>
      <div
        style={{
          background: "#1c1e27",
          border: "1px solid #242736",
          borderRadius: 10,
          padding: "4px 0",
          display: "flex",
          flexDirection: "column",
          gap: 0,
        }}
      >
        {children}
      </div>
    </div>
  );
}

function Field({
  label,
  description,
  children,
}: {
  label: string;
  description?: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <div style={{ padding: "12px 16px", borderBottom: "1px solid #1a1d28" }}>
      <label
        style={{
          display: "block",
          fontSize: 13,
          color: "#e0e2e8",
          marginBottom: 6,
          fontWeight: 500,
        }}
      >
        {label}
      </label>
      {description && (
        <p style={{ fontSize: 11, color: "#5a5f72", marginBottom: 8 }}>
          {description}
        </p>
      )}
      {children}
    </div>
  );
}

function SliderField({
  label,
  value,
  min,
  max,
  format,
  onChange,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  format: (v: number) => string;
  onChange: (v: number) => void;
}) {
  return (
    <div
      style={{
        padding: "12px 16px",
        borderBottom: "1px solid #1a1d28",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        gap: 16,
      }}
    >
      <span style={{ fontSize: 13, color: "#e0e2e8", flexShrink: 0 }}>
        {label}
      </span>
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <input
          type="range"
          min={min}
          max={max}
          value={value}
          onChange={(e) => onChange(Number(e.target.value))}
          style={{ accentColor: "#3d6ef8", width: 100 }}
        />
        <span
          style={{
            fontSize: 12,
            color: "#e0e2e8",
            width: 32,
            textAlign: "right",
          }}
        >
          {format(value)}
        </span>
      </div>
    </div>
  );
}

function InfoBox({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        margin: "8px 12px",
        padding: "8px 12px",
        background: "#1a2235",
        border: "1px solid #1e3a5f",
        borderRadius: 6,
        fontSize: 12,
        color: "#7aa2cc",
        lineHeight: 1.5,
      }}
    >
      ℹ️ {children}
    </div>
  );
}
