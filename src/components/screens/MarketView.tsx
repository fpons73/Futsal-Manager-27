import { useEffect, useState } from "react";
import { api, type MarketPlayer, type OfferRow } from "../../api";

export default function MarketView() {
  const [market, setMarket] = useState<MarketPlayer[]>([]);
  const [offers, setOffers] = useState<OfferRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [fees, setFees] = useState<Record<number, string>>({});
  const [msg, setMsg] = useState<string | null>(null);

  const load = async () => {
    setLoading(true);
    try {
      const [m, o] = await Promise.all([api.getMarket(), api.getOffers()]);
      setMarket(m);
      setOffers(o);
    } finally { setLoading(false); }
  };
  useEffect(() => { load(); }, []);

  const offer = async (p: MarketPlayer) => {
    const fee = Number(fees[p.id] || p.value);
    if (!fee || fee <= 0) { setMsg("Introduce una oferta válida"); return; }
    try {
      const res = await api.makeOffer(p.id, fee);
      setMsg(res);
      await load();
    } catch (e) { setMsg(String(e)); }
  };

  const respond = async (id: number, accept: boolean) => {
    try {
      const res = await api.respondOffer(id, accept);
      setMsg(res);
      await load();
    } catch (e) { setMsg(String(e)); }
  };

  if (loading) return <div className="p-8 text-center text-fm-dim">Cargando mercado…</div>;

  return (
    <div className="mx-auto max-w-6xl space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-black">Mercado de fichajes</h2>
        <button onClick={load} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-1.5 text-sm">Actualizar</button>
      </div>
      {msg && <div className="rounded-lg border border-fm-accent/30 bg-fm-accent/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <div className="border-b border-fm-border bg-fm-bg px-3 py-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Jugadores disponibles · {market.length}</div>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim">
              <tr><th className="px-3 py-2 text-left">Jugador</th><th className="px-2 py-2">Edad</th><th className="px-2 py-2">Pos</th><th className="px-2 py-2">CA</th><th className="px-2 py-2">Club</th><th className="px-2 py-2">Valor</th><th className="px-2 py-2">Salario</th><th className="px-2 py-2">Oferta</th><th className="px-2 py-2"></th></tr>
            </thead>
            <tbody>
              {market.map((p) => (
                <tr key={p.id} className="border-t border-fm-border hover:bg-fm-panel2">
                  <td className="px-3 py-2"><div className="font-semibold">{p.name}</div><div className="text-xs text-fm-dim">{p.nation}</div></td>
                  <td className="px-2 py-2 text-center">{p.age}</td>
                  <td className="px-2 py-2 text-center"><span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold">{p.position}</span></td>
                  <td className="px-2 py-2 text-center font-mono"><span className="font-bold">{p.ca}</span><span className="text-fm-dim">/{p.pa}</span></td>
                  <td className="px-2 py-2 text-xs">{p.club_short}</td>
                  <td className="px-2 py-2 text-right font-mono text-xs">€{Math.round(p.value).toLocaleString()}</td>
                  <td className="px-2 py-2 text-right font-mono text-xs">€{Math.round(p.wage).toLocaleString()}</td>
                  <td className="px-2 py-2"><input value={fees[p.id] ?? String(Math.round(p.value))} onChange={(e)=>setFees({...fees,[p.id]:e.target.value})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1 font-mono text-xs" /></td>
                  <td className="px-2 py-2"><button onClick={()=>offer(p)} className="rounded bg-fm-accent px-3 py-1 text-xs font-bold text-black hover:brightness-110">Ofertar</button></td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <div className="border-b border-fm-border bg-fm-bg px-3 py-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Ofertas · {offers.length}</div>
        {offers.length===0 ? <div className="p-4 text-sm text-fm-dim">Sin ofertas.</div> : (
          <div className="divide-y divide-fm-border">
            {offers.map((o)=>(
              <div key={o.id} className="flex flex-wrap items-center justify-between gap-2 px-3 py-2 text-sm">
                <span><b>{o.player_name}</b> · {o.from_club} → {o.to_club} · <span className="font-mono">€{Math.round(o.fee).toLocaleString()}</span> · <span className={`rounded px-1.5 py-0.5 text-xs ${o.status==="pending"?"bg-amber-500/20 text-amber-400":o.status==="accepted"?"bg-emerald-500/20 text-emerald-400":"bg-red-500/20 text-red-400"}`}>{o.status}</span> · {o.date}</span>
                {o.status==="pending" && (
                  <span className="flex gap-1">
                    <button onClick={()=>respond(o.id,true)} className="rounded bg-emerald-600 px-2 py-1 text-xs font-bold text-white">Aceptar</button>
                    <button onClick={()=>respond(o.id,false)} className="rounded bg-red-600 px-2 py-1 text-xs font-bold text-white">Rechazar</button>
                  </span>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
