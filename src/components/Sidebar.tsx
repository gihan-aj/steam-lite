export type Page = "discover" | "wishlist" | "deals" | "settings";

interface SidebarProps {
  current: Page;
  onChange: (page: Page) => void;
}

const items: { id: Page; icon: string; label: string }[] = [
  { id: "discover", icon: "🔥", label: "Discover" },
  { id: "wishlist", icon: "❤️", label: "Wishlist" },
  { id: "deals", icon: "🏷️", label: "Deals" },
  { id: "settings", icon: "⚙️", label: "Settings" },
];

export function Sidebar({ current, onChange }: SidebarProps) {
  return (
    <aside
      className="flex flex-col items-center py-3 gap-1 shrink-0"
      style={{
        width: 56,
        background: "#13141a",
        borderRight: "1px solid #1a1d28",
        paddingTop: "0.6rem"
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
      {items.map((item) => (
        <NavItem
          key={item.id}
          {...item}
          active={current === item.id}
          onClick={() => onChange(item.id)}
        />
      ))}
    </aside>
  );
}

function NavItem({
  icon,
  label,
  active,
  onClick,
}: {
  icon: string;
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      title={label}
      className="w-10 h-10 rounded-xl flex items-center justify-center
                 transition-all relative group"
      style={{
        background: active ? "#1e2540" : "transparent",
        fontSize: 18,
      }}
      onMouseEnter={(e) => {
        if (!active)
          (e.currentTarget as HTMLButtonElement).style.background = "#1a1d28";
      }}
      onMouseLeave={(e) => {
        if (!active)
          (e.currentTarget as HTMLButtonElement).style.background =
            "transparent";
      }}
    >
      {/* Active indicator bar */}
      {active && (
        <span
          className="absolute left-0 top-2 bottom-2 w-0.5 rounded-r-full"
          style={{ background: "#3d6ef8" }}
        />
      )}
      {icon}

      {/* Tooltip on hover */}
      <span
        className="absolute left-12 px-2 py-1 rounded text-xs font-medium
                   pointer-events-none opacity-0 group-hover:opacity-100
                   transition-opacity whitespace-nowrap z-50"
        style={{
          background: "#242736",
          color: "#e0e2e8",
          border: "1px solid #2a2d3a",
          fontSize: 12,
        }}
      >
        {label}
      </span>
    </button>
  );
}