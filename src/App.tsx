import "./App.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useState } from "react";
import { Page, Sidebar } from "./components/Sidebar";
import { Discover } from "./pages/Discover";
import { Wishlist } from "./pages/Wishlist";
import { Deals } from "./pages/Deals";
import { Settings } from "./pages/Settings";
import { TitleBar } from "./components/TitleBar";

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

  const pages: Record<Page, React.ReactNode> = {
    discover: <Discover />,
    wishlist: <Wishlist />,
    deals: <Deals />,
    settings: <Settings />,
  };

  return (
    <div
      className="flex flex-col"
      style={{ height: "100vh", background: "#0e0f13", overflow: "hidden" }}
    >
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar current={currentPage} onChange={setCurrentPage} />
        <main className="flex-1 overflow-hidden">{pages[currentPage]}</main>
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
