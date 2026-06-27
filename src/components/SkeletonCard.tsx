export function SkeletonCard() {
    return (
      <div
        style={{
          background: "#1c1e27",
          border: "1px solid #242736",
          borderRadius: 12,
          overflow: "hidden",
          minHeight: 320,
        }}
      >
        {/* Image placeholder */}
        <div
          style={{
            height: 115,
            background:
              "linear-gradient(90deg, #1a1d28 25%, #242736 50%, #1a1d28 75%)",
            backgroundSize: "200% 100%",
            animation: "shimmer 1.5s infinite",
          }}
        />

        <div
          style={{
            padding: "10px 13px",
            display: "flex",
            flexDirection: "column",
            gap: 8,
          }}
        >
          {/* Title */}
          <div
            style={{
              height: 14,
              width: "75%",
              borderRadius: 4,
              background:
                "linear-gradient(90deg, #1a1d28 25%, #242736 50%, #1a1d28 75%)",
              backgroundSize: "200% 100%",
              animation: "shimmer 1.5s infinite",
            }}
          />
          {/* Description lines */}
          <div
            style={{
              height: 11,
              width: "100%",
              borderRadius: 4,
              background:
                "linear-gradient(90deg, #1a1d28 25%, #242736 50%, #1a1d28 75%)",
              backgroundSize: "200% 100%",
              animation: "shimmer 1.5s infinite 0.1s",
            }}
          />
          <div
            style={{
              height: 11,
              width: "60%",
              borderRadius: 4,
              background:
                "linear-gradient(90deg, #1a1d28 25%, #242736 50%, #1a1d28 75%)",
              backgroundSize: "200% 100%",
              animation: "shimmer 1.5s infinite 0.2s",
            }}
          />
          {/* Price */}
          <div style={{ marginTop: "auto", paddingTop: 8 }}>
            <div
              style={{
                height: 18,
                width: "40%",
                borderRadius: 4,
                background:
                  "linear-gradient(90deg, #1a1d28 25%, #242736 50%, #1a1d28 75%)",
                backgroundSize: "200% 100%",
                animation: "shimmer 1.5s infinite 0.3s",
              }}
            />
          </div>
        </div>
      </div>
    );
}

export function SkeletonGrid({ count = 6 }: { count?: number }) {
    return (
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill, minmax(220px, 1fr))",
          gridAutoRows: "1fr",
          gap: 12,
        }}
      >
        {Array.from({ length: count }).map((_, i) => (
          <SkeletonCard key={i} />
        ))}
      </div>
    );
}