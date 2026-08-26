import { useStore } from "./store";
import NewGame from "./components/screens/NewGame";
import Dashboard from "./components/screens/Dashboard";
import SquadView from "./components/screens/SquadView";
import StandingsView from "./components/screens/StandingsView";
import FixturesView from "./components/screens/FixturesView";
import LiveMatch from "./components/screens/LiveMatch";

function Shell({ children }: { children: React.ReactNode }) {
  const { screen, setScreen, gameState, userClubId } = useStore();
  if (!gameState || !userClubId) return <>{children}</>;
  const items: { id: typeof screen; label: string }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "squad", label: "Plantilla" },
    { id: "standings", label: "Clasificación" },
    { id: "fixtures", label: "Calendario" },
    { id: "tactics", label: "Partido" },
  ];
  return (
    <div className="min-h-screen bg-fm-bg">
      <header className="sticky top-0 z-10 border-b border-fm-border bg-fm-panel/95 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-3">
          <div className="flex items-center gap-3">
            <span className="text-sm font-black tracking-tight"><span className="text-fm-accent">FM</span>27</span>
            <span className="hidden text-xs text-fm-dim sm:inline">{gameState.game_date} · {gameState.season} · {gameState.user_club_name}</span>
          </div>
          <nav className="flex gap-1">
            {items.map((it) => (
              <button key={it.id} onClick={() => setScreen(it.id)} className={`rounded-lg px-3 py-1.5 text-sm font-semibold ${screen===it.id ? "bg-fm-accent text-black" : "text-fm-dim hover:bg-fm-bg hover:text-white"}`}>{it.label}</button>
            ))}
            <button onClick={() => setScreen("newgame")} className="ml-2 rounded-lg border border-fm-border px-3 py-1.5 text-sm text-fm-dim hover:text-white">Salir</button>
          </nav>
        </div>
      </header>
      <main>{children}</main>
    </div>
  );
}

export default function App() {
  const { screen } = useStore();
  return (
    <Shell>
      {screen === "newgame" && <NewGame />}
      {screen === "dashboard" && <Dashboard />}
      {screen === "squad" && <SquadView />}
      {screen === "standings" && <StandingsView />}
      {screen === "fixtures" && <FixturesView />}
      {screen === "tactics" && <LiveMatch />}
    </Shell>
  );
}
