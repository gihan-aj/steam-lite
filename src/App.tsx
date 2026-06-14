import "./App.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Home } from "./pages/Home";

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

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Home />
    </QueryClientProvider>
  );
}

export default App;
