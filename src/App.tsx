import "./App.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useState } from "react";
import { NavItem, Page, Sidebar } from "./components/Sidebar";
import { Discover } from "./pages/Discover";
import { Wishlist } from "./pages/Wishlist";
import { Deals } from "./pages/Deals";
import { Settings } from "./pages/Settings";
import { TitleBar } from "./components/TitleBar";
import { useSyncListener } from "./hooks/useWishlist";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { useUpdater } from "./hooks/useUpdater";

// Create the TanStack Query client — one instance for the whole app
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Don't refetch just because the window regained focus
      // (This app is always focused — it's a desktop app)
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});

function AppShell() {
  const [currentPage, setCurrentPage] = useState<Page>("discover");
  const {
    currentVersion,
    available: updateAvailable,
    version: updateVersion,
    downloading: updateDownloading,
    progress: updateProgress,
    checkForUpdates,
  } = useUpdater();
  useSyncListener();

  useSyncListener();

  const navItems: NavItem[] = [
    { id: "discover", icon: "🔥", label: "Discover" },
    { id: "wishlist", icon: "❤️", label: "Wishlist" },
    { id: "deals", icon: "🏷️", label: "Deals" },
    {
      id: "settings",
      icon: "⚙️",
      label: "Settings",
      badge: updateAvailable, // ← green dot when update available
    },
  ];

  const pages: Record<Page, React.ReactNode> = {
    discover: (
      <ErrorBoundary>
        <Discover />
      </ErrorBoundary>
    ),
    wishlist: (
      <ErrorBoundary>
        <Wishlist />
      </ErrorBoundary>
    ),
    deals: (
      <ErrorBoundary>
        <Deals />
      </ErrorBoundary>
    ),
    settings: (
      <ErrorBoundary>
        <Settings
          currentVersion={currentVersion}
          updateAvailable={updateAvailable}
          updateVersion={updateVersion}
          updateDownloading={updateDownloading}
          updateProgress={updateProgress}
          onCheckUpdate={checkForUpdates}
        />
      </ErrorBoundary>
    ),
  };

  return (
    <div
      className="flex flex-col"
      style={{ height: "100vh", background: "#0e0f13", overflow: "hidden" }}
    >
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar
          current={currentPage}
          onChange={setCurrentPage}
          navItems={navItems}
        />
        <main className="flex-1 overflow-hidden">
          <div key={currentPage} className="page-transition h-full">
            {pages[currentPage]}
          </div>
        </main>
      </div>
    </div>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AppShell />
    </QueryClientProvider>
  );
}

export default App;
