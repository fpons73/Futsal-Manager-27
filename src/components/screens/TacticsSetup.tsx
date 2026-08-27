import { useEffect, useState } from "react";
import { api, type PreMatch } from "../../api";

interface Props { matchId: number; onStart: (snapshot: any) => void; onBack: () => void; }

const FORMATIONS = [
  { code: "3-1", label: "3-1" },
  { code: "4-0", label: "4-0" },
  { code: "2-2", label: "2-2" },
  { code: "5-0", label: "5-0" },
];

const SLOTS = [
  { role: "POR" },
  { role: "CIE" },
  { role: "ALA" },
  { role: "ALA" },
  { role: "PIV" },
];

function Slider({ label, value, onChange, min = 0, max = 100 }: { label:string; value:number; onChange:(n:number)=>void; min?:number; max?:number }) {
  return (
    <label className="flex-1">
      <div className="mb-1 flex justify-between text-[10px] uppercase tracking-wider text-fm-dim"><span>{label}</span><span className="font-mono text-fm-accent">{value}</span></div>
      <input type="range" min={min} max={max} value={value} onChange={(e)=>onChange(Number(e.target.value))} className="w-full accent-fm-accent" />
    </label>
  );
}

export default function TacticsSetup({ matchId, onStart, onBack }: Props) {
  const [pm, setPm] = useState<PreMatch | null>(null);
  const [formation, setFormation] = useState("3-1");
  const [tempo, setTempo] = useState(50);
  const [pressing, setPressing] = useState(50);
  const [defensiveLine, setDefensiveLine] = useState(50);
  const [width, setWidth] = useState(50);
  const [powerplay, setPowerplay] = useState(true);
  const [slots, setSlots] = useState<(number | null)[]>([null, null, null, null, null]);
  const [msg, setMsg] = useState<string | null>(null);
  const [starting, setStarting] = useState(false);

  useEffect(() => {
    api.getPreMatch(matchId).then((data) => {
      setPm(data);
      setFormation(data.tactics.formation);
      setTempo(data.tactics.tempo);
      setPressing(data.tactics.pressing);
      setDefensiveLine(data.tactics.defensive_line);
      setWidth(data.tactics.width);
      setPowerplay(data.tactics.powerplay_enabled);
    }).catch((e)=>setMsg(String(e)));
  }, [matchId]);

  const pick = (slotIdx: number, pid: number) => {
    // evitar duplicados en el once
    if (slots.includes(pid)) { setMsg("Ese jugador ya está en el once"); return; }
    setMsg(null);
    setSlots((s)=> s.map((v, i)=> i===slotIdx ? pid : v));
  };

  const start = async () => {
    const lineup = slots.filter((s)=> s!==null);
    if (lineup.length < 5) { setMsg("Elige el quintero inicial (5 jugadores)"); return; }
    setStarting(true);
    try {
      const snap = await api.startLiveTactics({ matchId, formation, tempo, pressing, defensiveLine, width, powerplayEnabled: powerplay, lineup });
      onStart(snap);
    } catch (e) { setMsg(String(e)); setStarting(false); }
  };

  if (!pm) return <div className="p-8 text-center text-fm-dim">Cargando tácticas… {msg && <span className="text-red-400">({msg})</span>}</div>;

  return (
    <div className="mx-auto max-w-6xl space-y-4 p-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-black">Configuración táctica</h2>
        <div className="flex items-center gap-2 text-sm">
          <span className="font-bold">{pm.home_name}</span><span className="text-fm-dim">vs</span><span className="font-bold">{pm.away_name}</span>
        </div>
      </div>
      {msg && <div className="rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-3">
        {/* Formación + controles */}
        <div className="space-y-4 rounded-xl border border-fm-border bg-fm-panel p-4">
          <div>
            <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Formación</div>
            <div className="flex gap-1">
              {FORMATIONS.map((f)=> (
                <button key={f.code} onClick={()=>setFormation(f.code)} className={`flex-1 rounded-lg px-2 py-1.5 text-sm font-bold ${formation===f.code ? "bg-fm-accent text-black" : "bg-fm-bg text-fm-dim hover:text-white"}`}>{f.label}</button>
              ))}
            </div>
          </div>

          <div className="space-y-3">
            <Slider label="Tempo" value={tempo} onChange={setTempo} />
            <Slider label="Presión" value={pressing} onChange={setPressing} />
            <Slider label="Línea defensiva" value={defensiveLine} onChange={setDefensiveLine} />
            <Slider label="Amplitud" value={width} onChange={setWidth} />
          </div>

          <label className="flex items-center gap-2 text-sm">
            <input type="checkbox" checked={powerplay} onChange={(e)=>setPowerplay(e.target.checked)} className="accent-fm-accent" />
            Activar powerplay (portero-jugador) al final si vas perdiendo
          </label>

          <div className="rounded-lg border border-fm-border bg-fm-bg p-3">
            <div className="mb-1 text-xs font-bold uppercase tracking-widest text-fm-dim">Quintero inicial (5)</div>
            <div className="space-y-1">
              {SLOTS.map((slot, i)=>(
                <div key={i} className="flex items-center justify-between gap-2 rounded bg-fm-panel px-2 py-1 text-sm">
                  <span className="w-12 text-xs font-bold text-fm-dim">{slot.role}</span>
                  <select value={slots[i] ?? ""} onChange={(e)=> pick(i, Number(e.target.value))} className="flex-1 rounded border border-fm-border bg-fm-bg px-2 py-1 text-sm">
                    <option value="">— elegir —</option>
                    {pm.squad.map((p)=><option key={p.id} value={p.id}>{p.name} ({p.position}) CA {p.ca}</option>)}
                  </select>
                </div>
              ))}
            </div>
          </div>

          <div className="flex gap-2 pt-2">
            <button onClick={start} disabled={starting} className="flex-1 rounded-lg bg-fm-accent px-4 py-2.5 font-bold text-black disabled:opacity-50">{starting ? "Empezando…" : "Empezar partido"}</button>
            <button onClick={onBack} className="rounded-lg border border-fm-border px-4 py-2.5 text-sm text-fm-dim hover:text-white">Volver</button>
          </div>
        </div>

        {/* Vista previa del campo */}
        <div className="lg:col-span-2">
          <div className="rounded-xl border border-fm-border bg-[#1b5e20] p-4">
            <div className="relative mx-auto aspect-[2/1] w-full max-w-2xl rounded-xl border-2 border-white/60 bg-[#2d8a2d]">
              {/* líneas del campo */}
              <div className="absolute inset-0 rounded-xl" style={{ boxShadow: "inset 0 0 0 2px rgba(255,255,255,.4)" }} />
              <div className="absolute left-1/2 top-2 bottom-2 w-px bg-white/50" />
              <div className="absolute left-1/2 top-1/2 h-16 w-16 -translate-x-1/2 -translate-y-1/2 rounded-full border border-white/50" />
              <div className="absolute left-2 top-1/2 -translate-y-1/2 h-24 w-24 border border-white/50" />
              <div className="absolute right-2 top-1/2 -translate-y-1/2 h-24 w-24 border border-white/50" />

              {/* jugadores en campo */}
              {slots.map((pid, i)=>{
                const p = pm.squad.find((x)=>x.id===pid);
                const col = i===0 ? "bg-amber-400" : "bg-sky-400";
                const ratio = i===0 ? 0.12 : i===1 ? 0.32 : (i<=3 ? 0.6 : 0.8);
                const y = i===0 ? 50 : i===1 ? 50 : i===2 ? 26 : i===3 ? 74 : 50;
                return (
                  <div key={i} className="absolute flex h-11 w-11 -translate-x-1/2 -translate-y-1/2 flex-col items-center justify-center rounded-full border-2 border-white text-black" style={{ left: `${ratio*100}%`, top: `${y}%`, background: col }}>
                    <span className="text-[10px] font-black leading-none">{p ? "•" : "?"}</span>
                    <span className="w-full truncate text-center text-[8px] leading-tight">{p ? p.name.split(" ")[0] : SLOTS[i].role}</span>
                  </div>
                );
              })}

              {/* parte inferior */}
              <div className="absolute bottom-3 left-1/2 -translate-x-1/2 text-[10px] uppercase tracking-widest text-white/80">{formation} · {pm.home_name}</div>
            </div>
            <p className="mt-2 text-center text-xs text-fm-dim">Arrastra la selección de cada rol en el panel para formar el quinteto. Los titulares se colocan según la formación elegida.</p>
          </div>
        </div>
      </div>
    </div>
  );
}
