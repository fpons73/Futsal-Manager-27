import { useEffect, useState } from "react";
import { api, type PlayerRow } from "../../api";
import { useStore } from "../../store";

export default function SquadView() {
  const { userClubId, clubs } = useStore();
  const [players, setPlayers] = useState<PlayerRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!userClubId) { setLoading(false); setError("No hay club seleccionado"); return; }
    setLoading(true);
    setError(null);
    api.getSquad(userClubId).then((p)=>{
      setPlayers(p);
      if (p.length===0) setError(`Club ${userClubId} (${clubs.find(c=>c.id===userClubId)?.name ?? "?"}) sin jugadores — verifica DB`);
    }).catch((e)=> setError(String(e))).finally(()=>setLoading(false));
  }, [userClubId]);

  if (loading) return <div className="p-8 text-center text-fm-dim">Cargando plantilla…</div>;
  if (error) return <div className="p-8 text-center"><div className="text-amber-400">{error}</div><button onClick={()=> userClubId && api.getSquad(userClubId).then(setPlayers)} className="mt-2 rounded bg-fm-accent px-3 py-1 text-sm font-bold text-black">Reintentar</button></div>;
  if (!players.length) return <div className="p-8 text-center text-fm-dim">Sin jugadores. (Club {userClubId})</div>;

  return (
    <div className="mx-auto max-w-6xl p-6">
      <h2 className="mb-4 text-xl font-black">Plantilla <span className="font-normal text-fm-dim">({players.length} jugadores)</span></h2>
      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim">
              <tr>
                <th className="px-3 py-2 text-left">Jugador</th>
                <th className="px-2 py-2">Pos</th>
                <th className="px-2 py-2">Edad</th>
                <th className="px-2 py-2">Nac</th>
                <th className="px-2 py-2">CA</th>
                <th className="px-2 py-2">PA</th>
                <th className="px-2 py-2">Cond</th>
                <th className="px-2 py-2">Salario</th>
                <th className="px-2 py-2">PAS</th>
                <th className="px-2 py-2">FIN</th>
                <th className="px-2 py-2">REG</th>
                <th className="px-2 py-2">ENT</th>
                <th className="px-2 py-2">RIT</th>
              </tr>
            </thead>
            <tbody>
              {players.map((p) => (
                <tr key={p.id} className="border-t border-fm-border hover:bg-fm-panel2">
                  <td className="px-3 py-2 font-semibold">{p.common_name} <span className="font-normal text-fm-dim">({p.first_name} {p.last_name})</span></td>
                  <td className="px-2 py-2 text-center"><span className={`rounded px-1.5 py-0.5 text-xs font-bold ${p.position==="POR" ? "bg-amber-500/20 text-amber-400" : p.position==="PIV" ? "bg-red-500/20 text-red-400" : p.position==="CIE" ? "bg-sky-500/20 text-sky-400" : "bg-emerald-500/20 text-emerald-400"}`}>{p.position}</span></td>
                  <td className="px-2 py-2 text-center">{p.age}</td>
                  <td className="px-2 py-2 text-center text-xs">{p.nation}</td>
                  <td className="px-2 py-2 text-center font-mono font-bold">{p.ca}</td>
                  <td className="px-2 py-2 text-center font-mono text-fm-dim">{p.pa}</td>
                  <td className="px-2 py-2 text-center"><span className={`rounded px-1.5 py-0.5 text-xs ${p.condition>80 ? "bg-emerald-500/20 text-emerald-400" : p.condition>60 ? "bg-amber-500/20 text-amber-400" : "bg-red-500/20 text-red-400"}`}>{p.condition}%</span></td>
                  <td className="px-2 py-2 text-right font-mono text-xs">€{Math.round(p.wage).toLocaleString()}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.passing}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.finishing}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.dribbling}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.tackling}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.reflexes}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
