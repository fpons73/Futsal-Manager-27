import { useEffect, useState } from "react";
import { api, type FinanceRow } from "../../api";

export default function FinanceView() {
  const [fin, setFin] = useState<FinanceRow | null>(null);
  const [injuries, setInjuries] = useState<[number,string,string,string,string][]>([]);
  const [loading, setLoading] = useState(true);

  const load = async () => {
    setLoading(true);
    try {
      const [f, inj] = await Promise.all([api.getFinance(), api.getInjuries()]);
      setFin(f);
      setInjuries(inj);
    } finally { setLoading(false); }
  };
  useEffect(()=>{ load(); },[]);

  if (loading) return <div className="p-8 text-center text-fm-dim">Cargando finanzas…</div>;
  if (!fin) return <div className="p-8 text-center text-fm-dim">Sin datos.</div>;

  const fmt = (n:number) => `€${Math.round(n).toLocaleString()}`;

  return (
    <div className="mx-auto max-w-6xl space-y-6 p-6">
      <h2 className="text-xl font-black">Finanzas · {fin.club_name}</h2>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <div className={`rounded-xl border p-4 ${fin.balance < 0 ? "border-red-500/30 bg-red-500/10" : "border-fm-border bg-fm-panel"}`}>
          <div className="text-xs uppercase tracking-widest text-fm-dim">Balance</div>
          <div className={`text-2xl font-black ${fin.balance < 0 ? "text-red-400" : "text-emerald-400"}`}>{fmt(fin.balance)}</div>
          <div className="text-xs text-fm-dim">Mensual {fmt(fin.monthly_balance)}</div>
        </div>
        <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
          <div className="text-xs uppercase tracking-widest text-fm-dim">Presupuesto fichajes</div>
          <div className="text-xl font-bold">{fmt(fin.transfer_budget)}</div>
          <div className="text-xs text-fm-dim">Disponible para ofertas</div>
        </div>
        <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
          <div className="text-xs uppercase tracking-widest text-fm-dim">Salarios</div>
          <div className="text-xl font-bold">{fmt(fin.total_wages)}<span className="text-sm font-normal text-fm-dim">/sem</span></div>
          <div className="text-xs text-fm-dim">Presupuesto {fmt(fin.wage_budget)}/sem · {fin.total_wages > fin.wage_budget ? "⚠️ Excedido" : "OK"}</div>
        </div>
        <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
          <div className="text-xs uppercase tracking-widest text-fm-dim">Ingresos</div>
          <div className="text-sm">Patrocinio {fmt(fin.sponsorship)}</div>
          <div className="text-sm">Taquilla {fmt(fin.ticket_income)}</div>
          <div className="text-sm">Premios {fmt(fin.prize_money)}</div>
        </div>
      </div>

      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <div className="border-b border-fm-border bg-fm-bg px-3 py-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Lesionados · {injuries.length}</div>
        {injuries.length===0 ? <div className="p-4 text-sm text-fm-dim">Sin lesionados.</div> : (
          <div className="divide-y divide-fm-border">
            {injuries.map(([id,name,type,ret,injDate])=>(
              <div key={id} className="flex items-center justify-between px-3 py-2 text-sm">
                <span className="font-semibold">{name}</span>
                <span className="text-fm-dim">{type} · hasta {ret} (desde {injDate})</span>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="rounded-xl border border-fm-border bg-amber-500/10 p-4 text-sm">
        <b>Cómo funciona:</b> cada lunes se descuentan salarios y se suma patrocinio (€15k base). Cada partido en casa genera taquilla (65-90% aforo × €12). Balance negativo genera aviso en la bandeja.
      </div>
    </div>
  );
}
