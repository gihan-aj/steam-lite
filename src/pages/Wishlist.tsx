export function Wishlist() {
  return (
    <div className="flex flex-col items-center justify-center h-full gap-3">
      <span style={{ fontSize: 48 }}>❤️</span>
      <h2 style={{ fontSize: 18, color: "#e0e2e8", fontWeight: 500 }}>
        Wishlist
      </h2>
      <p style={{ fontSize: 13, color: "#5a5f72" }}>
        Coming soon — add your Steam ID in Settings first
      </p>
    </div>
  );
}
