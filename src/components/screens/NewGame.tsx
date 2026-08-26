import { useEffect, useState } from "react";
import { api, type ClubRow } from "../../api";
import { useStore } from "../../store";

export default function NewGame() {
  const { setScreen, setGameState, setClubs, setCompetitions, setSelectedComp, setUserClub } = useStore();
  const [clubs, setLocalClubs] = useState<ClubRow[]>([]);
  const [filter, setFilter] = useState<"all" | "España" | "Brasil" | "Portugal">("all");
  const [loading, setLoading] = useState(true);
  const [creating, setCreating] = useState<number | null>(null);

  useEffect(() => {
    api.newGame().then((res) => {
      setLocalClubs(res.clubs);
      setLoading(false);
    }).catch(() => setLoading(false));
  }, []);

  const start = async (clubId: number) => {
    setCreating(clubId);
    try {
      const res = await api.newGame(clubId);
      const state = await api.getGameState();
      setClubs(res.clubs);
      setCompetitions(res.competitions);
      setGameState(state);
      setUserClub(clubId);
      const comp = res.competitions.find((c) => {
        const club = res.clubs.find((x) => x.id === clubId);
        return club ? c.nation === club.nation : false;
      });
      setSelectedComp(comp ? comp.id : res.competitions[0]?.id ?? null);
      setScreen("dashboard");
    } catch (e) {
      alert(String(e));
    } finally {
      setCreating(null);
    }
  };

  if (loading) return <div className="p-12 text-center text-fm-dim">Generando mundo…</div>;

  const filtered = filter === "all" ? clubs : clubs.filter((c) => c.nation === filter);

  const groups: Record<string, ClubRow[]> = { "España": [], "Brasil": [], "Portugal": [] };
  filtered.forEach((c) => { if (groups[c.nation]) groups[c.nation].push(c); });

  return (
    <div className="mx-auto max-w-6xl p-6">
      <div className="mb-8 text-center">
        <h1 className="text-4xl font-black tracking-tight"><span className="text-fm-accent">FUTSAL</span> MANAGER 27</h1>
        <p className="mt-2 text-fm-dim">Elige tu club para comenzar la temporada 2026/27</p>
        <div className="mt-4 flex justify-center gap-2">
          {(["all","España","Brasil","Portugal"] as const).map((f) => (
            <button key={f} onClick={() => setFilter(f)} className={`rounded-full px-4 py-1.5 text-sm font-semibold ${filter===f ? "bg-fm-accent text-black" : "bg-fm-panel border border-fm-border text-fm-dim hover:text-white"}`}>{f==="all" ? "Todas" : f}</button>
          ))}
        </div>
      </div>

      {Object.entries(groups).map(([nation, list]) => list.length>0 && (
        <div key={nation} className="mb-8">
          <h2 className="mb-3 flex items-center gap-2 text-sm font-bold uppercase tracking-widest text-fm-dim"><span className="h-px w-6 bg-fm-border"/> {nation} <span className="rounded bg-fm-panel2 px-2 py-0.5 text-xs normal-case">{list.length} clubes</span></h2>
          <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
            {list.map((c) => (
              <button key={c.id} onClick={() => start(c.id)} disabled={creating!==null} className="group flex items-center gap-3 rounded-xl border border-fm-border bg-fm-panel p-3 text-left transition hover:border-fm-accent/50 hover:bg-fm-panel2 disabled:opacity-60">
                <span className="flex h-10 w-10 items-center justify-center rounded-lg text-xs font-black text-white" style={{ background: c.primary_color }}>{c.short_name}</span>
                <span className="min-w-0 flex-1">
                  <span className="block truncate text-sm font-semibold">{c.name}</span>
                  <span className="block text-xs text-fm-dim">Rep {c.reputation}</span>
                </span>
                <span className="text-fm-dim group-hover:text-fm-accent">{creating===c.id ? "…" : "→"}</span>
              </button>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
