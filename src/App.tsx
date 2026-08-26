import { useEffect, useState } from "react";
import { useStore } from "./store";
import { api } from "./api";
import NewGame from "./components/screens/NewGame";
import Dashboard from "./components/screens/Dashboard";
import SquadView from "./components/screens/SquadView";
import StandingsView from "./components/screens/StandingsView";
import FixturesView from "./components/screens/FixturesView";
import LiveMatch from "./components/screens/LiveMatch";
import MarketView from "./components/screens/MarketView";
import InboxView from "./components/screens/InboxView";
import TrainingView from "./components/screens/TrainingView";
import FinanceView from "./components/screens/FinanceView";

function Shell({ children }: { children: React.ReactNode }) {
  const { screen, setScreen, gameState, userClubId } = useStore();
  const [unread, setUnread] = useState(0);

  useEffect(() => {
    if (!gameState || !userClubId) return;
    const t = setInterval(async () => {
      try { const inbox = await api.getInbox(); setUnread(inbox.filter((m)=>m.is_read===0).length); } catch {}
    }, 4000);
    api.getInbox().then((inbox)=> setUnread(inbox.filter((m)=>m.is_read===0).length)).catch(()=>{});
    return () => clearInterval(t);
  }, [gameState, userClubId]);

  if (!gameState || !userClubId) return <>{children}</>;

  const items: { id: typeof screen; label: string; badge?: number }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "squad", label: "Plantilla" },
    { id: "standings", label: "Clasificación" },
    { id: "fixtures", label: "Calendario" },
    { id: "tactics", label: "Partido" },
    { id: "market", label: "Mercado" },
    { id: "training", label: "Entreno" },
    { id: "finance", label: "Finanzas" },
    { id: "inbox", label: "Buzón", badge: unread },
  ];

  return (
    <div className="min-h-screen bg-fm-bg">
      <header className="sticky top-0 z-10 border-b border-fm-border bg-fm-panel/95 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-4 py-3 lg:px-6">
          <div className="flex items-center gap-3">
            <span className="text-sm font-black tracking-tight"><span className="text-fm-accent">FM</span>27</span>
            <span className="hidden text-xs text-fm-dim md:inline">{gameState.game_date} · {gameState.season} · {gameState.user_club_name}</span>
          </div>
          <nav className="flex flex-wrap gap-1">
            {items.map((it) => (
              <button key={it.id} onClick={() => setScreen(it.id)} className={`relative rounded-lg px-2.5 py-1.5 text-xs font-semibold lg:text-sm ${screen===it.id ? "bg-fm-accent text-black" : "text-fm-dim hover:bg-fm-bg hover:text-white"}`}>
                {it.label}
                {it.badge ? <span className="absolute -right-1 -top-1 rounded-full bg-red-500 px-1 py-0 text-[10px] font-bold text-white">{it.badge}</span> : null}
              </button>
            ))}
            <button onClick={() => setScreen("newgame")} className="ml-1 rounded-lg border border-fm-border px-2.5 py-1.5 text-xs text-fm-dim hover:text-white">Salir</button>
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
      {screen === "market" && <MarketView />}
      {screen === "inbox" && <InboxView />}
      {screen === "training" && <TrainingView />}
      {screen === "finance" && <FinanceView />}
    </Shell>
  );
}
