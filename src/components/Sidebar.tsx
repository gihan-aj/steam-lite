export type Page = "discover" | "wishlist" | "deals" | "settings";

export interface NavItem {
  id: Page;
  icon: string;
  label: string;
  badge?: boolean;
}

interface SidebarProps {
  current: Page;
  onChange: (page: Page) => void;
  navItems: NavItem[];
}

const items: { id: Page; icon: string; label: string }[] = [
  { id: "discover", icon: "🔥", label: "Discover" },
  { id: "wishlist", icon: "❤️", label: "Wishlist" },
  { id: "deals", icon: "🏷️", label: "Deals" },
  { id: "settings", icon: "⚙️", label: "Settings" },
];

export function Sidebar({ current, onChange, navItems }: SidebarProps) {
  return (
    <aside
      className="flex flex-col items-center py-3 gap-1 shrink-0"
      style={{
        width: 56,
        background: "#13141a",
        borderRight: "1px solid #1a1d28",
        paddingTop: "0.6rem",
      }}
    >
      {/* Logo mark */}
      <div
        className="w-9 h-9 rounded-lg flex items-center justify-center mb-3"
        style={{ background: "#3d6ef8" }}
      >
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="7" cy="7" r="6" stroke="white" strokeWidth="1.5" />
          <path
            d="M4 7h6M7 4v6"
            stroke="white"
            strokeWidth="1.5"
            strokeLinecap="round"
          />
        </svg>
      </div>

      {/* Nav items */}
      {navItems.map((item) => (
        <NavItemButton
          key={item.id}
          item={item}
          active={current === item.id}
          onClick={() => onChange(item.id)}
        />
      ))}
    </aside>
  );
}

function NavItemButton({
  item,
  active,
  onClick,
}: {
  item: NavItem;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      title={item.label}
      style={{
        width: 40,
        height: 40,
        borderRadius: 10,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        background: active ? "#1e2540" : "transparent",
        border: "none",
        fontSize: 18,
        cursor: "pointer",
        position: "relative",
        transition: "background 0.1s",
      }}
      onMouseEnter={(e) => {
        if (!active)
          (e.currentTarget as HTMLElement).style.background = "#1a1d28";
      }}
      onMouseLeave={(e) => {
        if (!active)
          (e.currentTarget as HTMLElement).style.background = "transparent";
      }}
    >
      {/* Active indicator */}
      {active && (
        <span
          style={{
            position: "absolute",
            left: 0,
            top: 8,
            bottom: 8,
            width: 2,
            borderRadius: "0 2px 2px 0",
            background: "#3d6ef8",
          }}
        />
      )}

      {item.icon}

      {/* Badge dot — update notification etc. */}
      {item.badge && (
        <span
          style={{
            position: "absolute",
            top: 4,
            right: 4,
            width: 7,
            height: 7,
            borderRadius: "50%",
            background: "#4ade80",
            border: "2px solid #13141a",
          }}
        />
      )}

      {/* Tooltip */}
      <span
        style={{
          position: "absolute",
          left: 48,
          background: "#242736",
          border: "1px solid #2a2d3a",
          color: "#e0e2e8",
          fontSize: 12,
          padding: "3px 8px",
          borderRadius: 6,
          whiteSpace: "nowrap",
          pointerEvents: "none",
          opacity: 0,
          transition: "opacity 0.15s",
          zIndex: 100,
        }}
        className="nav-tooltip"
      >
        {item.label}
      </span>
    </button>
  );
}