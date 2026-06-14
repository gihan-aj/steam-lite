import { getCurrentWindow } from "@tauri-apps/api/window";

export function TitleBar() {
    const win = getCurrentWindow();
    const title = "Steam Lite"

    return (
      <div
        data-tauri-drag-region
        className="h-9 flex items-center justify-between px-4 shrink-0"
        style={{
          background: "#0e0f13",
          borderBottom: "1px solid #1a1d28",
          paddingLeft: "0.5rem",
          paddingRight: "0.2rem",
        }}
      >
        {/* App identity */}
        <div className="flex items-center gap-2 pointer-events-none">
          <div className="w-4 h-4 rounded" style={{ background: "#3d6ef8" }} />
          <span style={{ fontSize: 13, color: "#9096a8", fontWeight: 500 }}>
            {title}
          </span>
        </div>

        {/* Window controls — must NOT be drag region */}
        <div className="flex items-center gap-1" data-tauri-drag-region="false">
          <WinButton
            onClick={() => win.minimize()}
            hoverColor="#2a2d3a"
            title="Minimise"
          >
            <svg width="10" height="1" viewBox="0 0 10 1">
              <rect width="10" height="1" fill="currentColor" />
            </svg>
          </WinButton>
          <WinButton
            onClick={() => win.toggleMaximize()}
            hoverColor="#2a2d3a"
            title="Maximise"
          >
            <svg width="9" height="9" viewBox="0 0 9 9">
              <rect
                x="0.5"
                y="0.5"
                width="8"
                height="8"
                fill="none"
                stroke="currentColor"
                strokeWidth="1"
              />
            </svg>
          </WinButton>
          <WinButton
            onClick={() => win.close()}
            hoverColor="#c42b1c"
            title="Close"
          >
            <svg width="10" height="10" viewBox="0 0 10 10">
              <line
                x1="0"
                y1="0"
                x2="10"
                y2="10"
                stroke="currentColor"
                strokeWidth="1.2"
              />
              <line
                x1="10"
                y1="0"
                x2="0"
                y2="10"
                stroke="currentColor"
                strokeWidth="1.2"
              />
            </svg>
          </WinButton>
        </div>
      </div>
    );
}

function WinButton({
  children,
  onClick,
  hoverColor,
  title,
}: {
  children: React.ReactNode;
  onClick: () => void;
  hoverColor: string;
  title: string;
}) {
    return (
      <button
        onClick={onClick}
        title={title}
        className="w-8 h-8 flex items-center justify-center rounded transition-colors"
        style={{ color: "#9096a8", background: "transparent" }}
        onMouseEnter={(e) => {
          (e.currentTarget as HTMLButtonElement).style.background = hoverColor;
          if (hoverColor === "#c42b1c")
            (e.currentTarget as HTMLButtonElement).style.color = "#fff";
        }}
        onMouseLeave={(e) => {
          (e.currentTarget as HTMLButtonElement).style.background =
            "transparent";
          (e.currentTarget as HTMLButtonElement).style.color = "#9096a8";
        }}
      >
        {children}
      </button>
    );
}