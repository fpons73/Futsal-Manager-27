import { useEffect, useState } from "react";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

type Staff = { id: number; first_name: string; last_name: string; common_name: string; nation: string; nation_id: number; role: string; club_id: number | null; club_name: string | null; tactical: number; man_management: number; judging: number; motivating: number; working_youngsters: number; physio_level: number; wage_weekly: number };
type Player = { id: number; first_name: string; last_name: string; common_name: string; nation: string; nation_id: number; club: string; club_id: number | null; position: string; ca: number; pa: number; age: number; foot: string };

const ROLES = ["coach","assistant","scout","physio"];

export default function ClubEditor({ club, nations, onClose }: { club: any; nations: any[]; onClose: () => void }) {
  const [crest, setCrest] = useState<string | null>(club.crest_path ?? null);
  const [coaches, setCoaches] = useState<Staff[]>([]);
  const [coachId, setCoachId] = useState<number | null>(club.coach_id ?? null);
  const [staff, setStaff] = useState<Staff[]>([]);
  const [squad, setSquad] = useState<Player[]>([]);
  const [allPlayers, setAllPlayers] = useState<Player[]>([]);
  const [msg, setMsg] = useState<string | null>(null);
  const [search, setSearch] = useState("");

  const refresh = async () => {
    try {
      const [c, st, sq, all] = await Promise.all([
        invoke<Staff[]>("editor_list_coaches"),
        invoke<Staff[]>("editor_list_staff", { clubId: club.id }),
        invoke<Player[]>("editor_list_players_by_club", { clubId: club.id }),
        invoke<Player[]>("editor_list_players", { limit: 2000 }),
      ]);
      setCoaches(c); setStaff(st); setSquad(sq); setAllPlayers(all);
    } catch (e) { setMsg(String(e)); }
  };
  useEffect(() => { refresh(); }, [club.id]);

  const onCrestPick = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = async () => {
      const dataUrl = reader.result as string;
      const base64 = dataUrl.split(",")[1] || "";
      const ext = (dataUrl.match(/data:image\/(\w+)/)?.[1] || "png").toLowerCase();
      try {
        const path = await invoke<string>("editor_set_crest", { clubId: club.id, data: base64, ext });
        setCrest(path);
        setMsg("Escudo actualizado");
      } catch (err) { setMsg(String(err)); }
    };
    reader.readAsDataURL(file);
  };

  const setCoach = async (id: number | null) => {
    try { await invoke("editor_set_coach", { clubId: club.id, staffId: id }); setCoachId(id); setMsg("Entrenador asignado"); refresh(); } catch (e){ setMsg(String(e)); }
  };

  const addStaff = async (role: string) => {
    const first = prompt("Nombre del " + role); if (!first) return;
    const last = prompt("Apellidos"); if (!last) return;
    const nid = nations[0]?.id ?? 1;
    try {
      await invoke("editor_create_staff", { first, last, nationId: nid, role, clubId: club.id, tactical: 10, manManagement: 12, judging: 12, motivating: 10, workingYoungsters: 10, physioLevel: 10, wageWeekly: 600 });
      refresh(); setMsg("Staff añadido");
    } catch (e){ setMsg(String(e)); }
  };
  const delStaff = async (id: number) => {
    if (!confirm("¿Borrar staff?")) return;
    try { await invoke("editor_delete_staff", { id }); refresh(); } catch (e){ setMsg(String(e)); }
  };

  const addPlayer = async (pid: number) => {
    try { await invoke("editor_assign_player", { playerId: pid, clubId: club.id }); refresh(); setMsg("Jugador añadido a la plantilla"); } catch (e){ setMsg(String(e)); }
  };
  const removePlayer = async (pid: number) => {
    try { await invoke("editor_release_player", { playerId: pid }); refresh(); setMsg("Jugador liberado"); } catch (e){ setMsg(String(e)); }
  };

  const freePlayers = allPlayers.filter((p) => (p.club_id ?? null) !== club.id && (!search || (p.common_name || p.first_name + " " + p.last_name).toLowerCase().includes(search.toLowerCase())));

  return (
    <div className="rounded-xl border border-sky-500/30 bg-fm-panel p-4">
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-lg font-black">Editando {club.name} <span className="text-fm-dim">(ID {club.id})</span></h3>
        <button onClick={onClose} className="rounded-lg border border-fm-border px-3 py-1 text-sm text-fm-dim hover:text-white">Cerrar</button>
      </div>
      {msg && <div className="mb-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-2">
        {/* Escudo */}
        <section className="rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Escudo</div>
          <div className="flex items-center gap-3">
            {crest ? (
              <img src={convertFileSrc(crest)} alt="escudo" className="h-16 w-16 rounded-lg border border-fm-border object-contain" />
            ) : (
              <div className="flex h-16 w-16 items-center justify-center rounded-lg border border-fm-border bg-fm-panel text-2xl">{club.short_name?.[0]}</div>
            )}
            <label className="cursor-pointer rounded-lg bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">
              Elegir escudo…
              <input type="file" accept="image/*" className="hidden" onChange={onCrestPick} />
            </label>
          </div>
          {club.short_name && <div className="mt-2 text-xs text-fm-dim">Corto: {club.short_name}</div>}
        </section>

        {/* Entrenador */}
        <section className="rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Entrenador</div>
          <div className="flex flex-wrap items-center gap-2">
            <select value={coachId ?? ""} onChange={(e)=>setCoach(e.target.value ? Number(e.target.value) : null)} className="flex-1 rounded border border-fm-border bg-fm-panel px-2 py-1.5 text-sm">
              <option value="">— Sin entrenador —</option>
              {coaches.map((c)=> <option key={c.id} value={c.id}>{c.common_name} {c.club_name ? `(${c.club_name})` : "(libre)"}</option>)}
            </select>
            <button onClick={() => addStaff("coach")} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Nuevo</button>
          </div>
          {coaches.find((c)=>c.id===coachId) && (
            <div className="mt-2 text-xs text-fm-dim">
              Táctica {coaches.find((c)=>c.id===coachId)!.tactical} · Gestión {coaches.find((c)=>c.id===coachId)!.man_management} · Motivación {coaches.find((c)=>c.id===coachId)!.motivating}
            </div>
          )}
        </section>
      </div>

      {/* Staff */}
      <section className="mt-4 rounded-lg border border-fm-border bg-fm-bg p-3">
        <div className="mb-2 flex items-center justify-between">
          <div className="text-xs font-bold uppercase tracking-widest text-fm-dim">Cuerpo técnico ({staff.length})</div>
          <div className="flex gap-1">
            {ROLES.map((r)=> (
              <button key={r} onClick={()=>addStaff(r)} className="rounded bg-fm-panel2 px-2 py-1 text-xs font-semibold text-fm-dim hover:text-white">+ {r}</button>
            ))}
          </div>
        </div>
        {staff.length===0 ? <div className="text-sm text-fm-dim">Sin staff. Usa los botones para añadir.</div> : (
          <div className="grid gap-1 sm:grid-cols-2">
            {staff.map((s)=>(
              <div key={s.id} className="flex items-center justify-between rounded bg-fm-panel px-2 py-1.5 text-sm">
                <span><b>{s.common_name}</b> <span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold uppercase">{s.role}</span></span>
                <button onClick={()=>delStaff(s.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button>
              </div>
            ))}
          </div>
        )}
      </section>

      {/* Plantilla */}
      <section className="mt-4 rounded-lg border border-fm-border bg-fm-bg p-3">
        <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Plantilla ({squad.length} jugadores)</div>
        <div className="mb-2 flex gap-2">
          <input placeholder="Buscar jugador libre para fichar…" value={search} onChange={(e)=>setSearch(e.target.value)} className="flex-1 rounded border border-fm-border bg-fm-panel px-2 py-1.5 text-sm" />
        </div>
        {search && freePlayers.length>0 && (
          <div className="mb-2 max-h-40 overflow-auto rounded border border-fm-border bg-fm-panel">
            {freePlayers.slice(0, 20).map((p)=>(
              <div key={p.id} className="flex items-center justify-between border-b border-fm-border px-2 py-1 text-sm hover:bg-fm-panel2">
                <span>{p.common_name} <span className="text-fm-dim">({p.position}, CA {p.ca}, {p.club || "libre"})</span></span>
                <button onClick={()=>addPlayer(p.id)} className="rounded bg-fm-accent px-2 py-0.5 text-xs font-bold text-black">Fichar</button>
              </div>
            ))}
          </div>
        )}
        {squad.length===0 ? <div className="text-sm text-fm-dim">Sin jugadores. Busca arriba para fichar.</div> : (
          <div className="grid gap-1 sm:grid-cols-2">
            {squad.map((p)=>(
              <div key={p.id} className="flex items-center justify-between rounded bg-fm-panel px-2 py-1.5 text-sm">
                <span>{p.common_name} <span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold">{p.position}</span> <span className="font-mono text-xs text-fm-dim">CA {p.ca}</span></span>
                <button onClick={()=>removePlayer(p.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Quitar</button>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
}
