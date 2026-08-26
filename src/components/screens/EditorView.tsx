import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ClubEditor from "../editor/ClubEditor";
import PlayerEditor from "../editor/PlayerEditor";
import StaffEditor from "../editor/StaffEditor";
import NationEditor from "../editor/NationEditor";

type Tab = "clubs" | "players" | "staff" | "nations" | "competitions";

const emptyClub = { id: 0, name: "", short: "", nation: 1, city: "", stadium: "", cap: 2000, rep: 600, c1: "#0f4c3a", c2: "#ffffff" };
const emptyPlayer = { id: 0, first: "", last: "", nation: 1, club: "", ca: 80, pa: 120, pos: "ALA" };
const emptyStaff = { id: 0, first: "", last: "", nation: 1, role: "assistant", club: "", tactical: 10, manManagement: 12, judging: 12, motivating: 10, workingYoungsters: 10, physioLevel: 10, wage: 600 };
const emptyNation = { id: 0, name: "", conf: 1, rep: 500, level: 50 };
const emptyComp = { id: 0, name: "", nation: "", tier: "", teams: 16, season: "2026/2027" };

export default function EditorView() {
  const [tab, setTab] = useState<Tab>("clubs");
  const [data, setData] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(false);
  const [msg, setMsg] = useState<string | null>(null);

  const [newClub, setNewClub] = useState(emptyClub);
  const [newPlayer, setNewPlayer] = useState(emptyPlayer);
  const [newStaff, setNewStaff] = useState(emptyStaff);
  const [newNation, setNewNation] = useState(emptyNation);
  const [newComp, setNewComp] = useState(emptyComp);

  const [editingClub, setEditingClub] = useState<any | null>(null);
  const [editingPlayer, setEditingPlayer] = useState<any | null>(null);
  const [editingStaff, setEditingStaff] = useState<any | null>(null);
  const [editingNation, setEditingNation] = useState<any | null>(null);

  const [confeds, setConfeds] = useState<any[]>([]);
  const [nations, setNations] = useState<any[]>([]);
  const [clubs, setClubs] = useState<any[]>([]);

  const load = async (t: Tab) => {
    setLoading(true);
    try {
      if (t === "nations") setData(await invoke<any[]>("editor_list_nations"));
      else if (t === "clubs") setData(await invoke<any[]>("editor_list_clubs"));
      else if (t === "players") setData(await invoke<any[]>("editor_list_players", { limit: 2000 }));
      else if (t === "staff") setData(await invoke<any[]>("editor_list_staff"));
      else if (t === "competitions") setData(await invoke<any[]>("editor_list_competitions"));
      setMsg(null);
    } catch (e) { setMsg(String(e)); }
    finally { setLoading(false); }
  };

  useEffect(() => { load(tab); }, [tab]);
  useEffect(() => {
    invoke<number>("editor_init")
      .then(async () => {
        const [c, n, cl] = await Promise.all([
          invoke<any[]>("editor_list_confederations_full").catch(()=>[]),
          invoke<any[]>("editor_list_nations").catch(()=>[]),
          invoke<any[]>("editor_list_clubs").catch(()=>[]),
        ]);
        setConfeds(c); setNations(n); setClubs(cl);
        load(tab);
      })
      .catch(()=>{});
  }, [tab]);

  const filtered = data.filter((r:any) => {
    if (!search) return true;
    const s = search.toLowerCase();
    return JSON.stringify(Object.values(r)).toLowerCase().includes(s);
  });
  const setSearchAndClear = (q:string) => setSearch(q);

  const inEdit = (state:any) => state.id !== 0;

  // ---- Clubs ----
  const saveClub = async () => {
    try {
      if (newClub.id) await invoke("editor_update_club", { id: newClub.id, name: newClub.name, shortName: newClub.short, nationId: newClub.nation, city: newClub.city, stadium: newClub.stadium, capacity: newClub.cap, reputation: newClub.rep, c1: newClub.c1, c2: newClub.c2 });
      else await invoke("editor_create_club", { name: newClub.name, shortName: newClub.short, nationId: newClub.nation, city: newClub.city, stadium: newClub.stadium, capacity: newClub.cap, reputation: newClub.rep, c1: newClub.c1, c2: newClub.c2 });
      setMsg(newClub.id ? "Club actualizado" : "Club creado");
      setNewClub(emptyClub); load("clubs");
    }catch(e){ setMsg(String(e)); }
  };
  const delClub = async (id:number) => { if(!confirm("¿Borrar club?")) return; try{ await invoke("editor_delete_club",{id}); load("clubs"); }catch(e){ setMsg(String(e)); } };

  // ---- Players ----
  const savePlayer = async () => {
    try {
      if (newPlayer.id) await invoke("editor_update_player", { id: newPlayer.id, first: newPlayer.first, last: newPlayer.last, nationId: newPlayer.nation, clubId: newPlayer.club ? Number(newPlayer.club) : null, ca: newPlayer.ca, pa: newPlayer.pa, pos: newPlayer.pos });
      else await invoke("editor_create_player", { first: newPlayer.first, last: newPlayer.last, nationId: newPlayer.nation, clubId: newPlayer.club ? Number(newPlayer.club) : null, ca: newPlayer.ca, pa: newPlayer.pa, pos: newPlayer.pos });
      setMsg(newPlayer.id ? "Jugador actualizado" : "Jugador creado");
      setNewPlayer(emptyPlayer); load("players");
    }catch(e){ setMsg(String(e)); }
  };
  const delPlayer = async (id:number) => { if(!confirm("¿Borrar jugador?")) return; try{ await invoke("editor_delete_player",{id}); load("players"); }catch(e){ setMsg(String(e)); } };

  // ---- Staff ----
  const saveStaff = async () => {
    try {
      if (newStaff.id) await invoke("editor_update_staff", { id: newStaff.id, first: newStaff.first, last: newStaff.last, nationId: newStaff.nation, role: newStaff.role, clubId: newStaff.club ? Number(newStaff.club) : null, tactical: newStaff.tactical, manManagement: newStaff.manManagement, judging: newStaff.judging, motivating: newStaff.motivating, workingYoungsters: newStaff.workingYoungsters, physioLevel: newStaff.physioLevel, wageWeekly: newStaff.wage });
      else await invoke("editor_create_staff", { first: newStaff.first, last: newStaff.last, nationId: newStaff.nation, role: newStaff.role, clubId: newStaff.club ? Number(newStaff.club) : null, tactical: newStaff.tactical, manManagement: newStaff.manManagement, judging: newStaff.judging, motivating: newStaff.motivating, workingYoungsters: newStaff.workingYoungsters, physioLevel: newStaff.physioLevel, wageWeekly: newStaff.wage });
      setMsg(newStaff.id ? "Staff actualizado" : "Staff creado");
      setNewStaff(emptyStaff); load("staff");
    }catch(e){ setMsg(String(e)); }
  };
  const delStaff = async (id:number) => { if(!confirm("¿Borrar staff?")) return; try{ await invoke("editor_delete_staff",{id}); load("staff"); }catch(e){ setMsg(String(e)); } };

  // ---- Nations ----
  const saveNation = async () => {
    try {
      if (newNation.id) await invoke("editor_update_nation", { id: newNation.id, name: newNation.name, confederationId: newNation.conf, reputation: newNation.rep, futsalLevel: newNation.level });
      else await invoke("editor_create_nation", { name: newNation.name, confederationId: newNation.conf, reputation: newNation.rep, futsalLevel: newNation.level });
      setMsg(newNation.id ? "Nación actualizada" : "Nación creada");
      setNewNation(emptyNation); load("nations");
    }catch(e){ setMsg(String(e)); }
  };
  const delNation = async (id:number) => { if(!confirm("¿Borrar nación?")) return; try{ await invoke("editor_delete_nation",{id}); load("nations"); }catch(e){ setMsg(String(e)); } };

  // ---- Competitions ----
  const saveComp = async () => {
    try {
      if (newComp.id) await invoke("editor_update_competition", { id: newComp.id, name: newComp.name, nationId: newComp.nation ? Number(newComp.nation) : null, tier: newComp.tier ? Number(newComp.tier) : null, totalTeams: newComp.teams, season: newComp.season });
      else await invoke("editor_create_competition", { name: newComp.name, nationId: newComp.nation ? Number(newComp.nation) : null, tier: newComp.tier ? Number(newComp.tier) : null, totalTeams: newComp.teams, season: newComp.season });
      setMsg(newComp.id ? "Competición actualizada" : "Competición creada");
      setNewComp(emptyComp); load("competitions");
    }catch(e){ setMsg(String(e)); }
  };
  const delComp = async (id:number) => { if(!confirm("¿Borrar competición?")) return; try{ await invoke("editor_delete_competition",{id}); load("competitions"); }catch(e){ setMsg(String(e)); } };

  return (
    <div className="mx-auto max-w-7xl space-y-4 p-6">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <h2 className="text-xl font-black">Editor — Base de Datos</h2>
        <span className="rounded-full bg-fm-accent px-3 py-1 text-xs font-bold text-black">37 competiciones · 26 naciones</span>
      </div>

      <div className="flex flex-wrap gap-1">
        {(["clubs","players","staff","nations","competitions"] as Tab[]).map((t)=>(
          <button key={t} onClick={()=>{ setTab(t); setSearch(""); }} className={`rounded-lg px-3 py-1.5 text-sm font-semibold ${tab===t ? "bg-fm-accent text-black" : "bg-fm-panel border border-fm-border text-fm-dim"}`}>{t.toUpperCase()}</button>
        ))}
      </div>

      {/* Buscador global de la pestaña */}
      <div className="flex items-center gap-2 rounded-lg border border-fm-border bg-fm-panel px-3 py-2">
        <span className="text-fm-dim">🔍</span>
        <input value={search} onChange={(e)=>setSearchAndClear(e.target.value)} placeholder={`Buscar en ${tab}… (nombre, nación, posición, etc.)`} className="w-full bg-transparent text-sm outline-none" />
        {filtered.length !== data.length && <span className="shrink-0 text-xs text-fm-dim">{filtered.length}/{data.length}</span>}
      </div>

      {msg && <div className="rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      {loading ? <div className="p-8 text-center text-fm-dim">Cargando…</div> : (
        <>
          {tab==="clubs" && (
            <div className="space-y-4">
              {editingClub && <ClubEditor club={editingClub} nations={nations} onClose={()=>{ setEditingClub(null); setNewClub(emptyClub); load("clubs"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newClub) ? "Datos del club · ID " + newClub.id : "Nuevo club"}</div>
                <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-4">
                  <input placeholder="Nombre" value={newClub.name} onChange={(e)=>setNewClub({...newClub,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Corto (3-4)" value={newClub.short} onChange={(e)=>setNewClub({...newClub,short:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newClub.nation} onChange={(e)=>setNewClub({...newClub,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
                  </select>
                  <input placeholder="Ciudad" value={newClub.city} onChange={(e)=>setNewClub({...newClub,city:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Pabellón" value={newClub.stadium} onChange={(e)=>setNewClub({...newClub,stadium:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Capacidad" value={newClub.cap} onChange={(e)=>setNewClub({...newClub,cap:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Rep" value={newClub.rep} onChange={(e)=>setNewClub({...newClub,rep:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveClub} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newClub) ? "Guardar" : "Crear club (+12 jugadores)"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Club</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Pabellón</th><th className="px-2 py-2">Rep</th><th className="px-2 py-2">Plant.</th><th></th></tr></thead>
                    <tbody>{filtered.map((c:any)=><tr key={c.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5"><span className="inline-flex items-center gap-2"><span className="flex h-7 w-7 items-center justify-center rounded bg-fm-bg text-xs font-bold text-white" style={{background:c.primary_color}}>{c.short_name}</span><span className="font-semibold">{c.name}</span></span></td>
                      <td className="px-2 py-1.5 text-xs">{c.nation}</td><td className="px-2 py-1.5 text-xs">{c.stadium}</td><td className="px-2 py-1.5 text-center font-mono">{c.reputation}</td><td className="px-2 py-1.5 text-center font-mono">{c.squad_count}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>{ setNewClub({ id: c.id, name: c.name, short: c.short_name, nation: c.nation_id, city: c.city ?? "", stadium: c.stadium ?? "", cap: c.capacity ?? 2000, rep: c.reputation, c1: c.primary_color ?? "#0f4c3a", c2: c.secondary_color ?? "#ffffff" }); setEditingClub(c); }} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delClub(c.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="players" && (
            <div className="space-y-4">
              {editingPlayer && <PlayerEditor player={editingPlayer} nations={nations} onClose={()=>{ setEditingPlayer(null); load("players"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newPlayer) ? "Editar jugador · ID " + newPlayer.id : "Nuevo jugador"}</div>
                <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-5">
                  <input placeholder="Nombre" value={newPlayer.first} onChange={(e)=>setNewPlayer({...newPlayer,first:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Apellidos" value={newPlayer.last} onChange={(e)=>setNewPlayer({...newPlayer,last:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newPlayer.nation} onChange={(e)=>setNewPlayer({...newPlayer,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
                  </select>
                  <select value={newPlayer.pos} onChange={(e)=>setNewPlayer({...newPlayer,pos:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option>POR</option><option>CIE</option><option>ALA</option><option>PIV</option><option>UNI</option></select>
                  <input placeholder="Club ID" value={newPlayer.club} onChange={(e)=>setNewPlayer({...newPlayer,club:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="CA" value={newPlayer.ca} onChange={(e)=>setNewPlayer({...newPlayer,ca:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="PA" value={newPlayer.pa} onChange={(e)=>setNewPlayer({...newPlayer,pa:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={savePlayer} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newPlayer) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Jugador</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Club</th><th className="px-2 py-2">Pos</th><th className="px-2 py-2">CA/PA</th><th></th></tr></thead>
                    <tbody>{filtered.map((p:any)=><tr key={p.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold">{p.common_name}</td><td className="px-2 py-1.5 text-xs">{p.nation}</td><td className="px-2 py-1.5 text-xs">{p.club || "-"}</td><td className="px-2 py-1.5 text-center text-xs">{p.position}</td><td className="px-2 py-1.5 text-center font-mono">{p.ca}/{p.pa}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setEditingPlayer(p)} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delPlayer(p.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="staff" && (
            <div className="space-y-4">
              {editingStaff && <StaffEditor staff={editingStaff} nations={nations} clubs={clubs} onClose={()=>{ setEditingStaff(null); load("staff"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newStaff) ? "Editar staff · ID " + newStaff.id : "Nuevo staff"}</div>
                <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-6">
                  <input placeholder="Nombre" value={newStaff.first} onChange={(e)=>setNewStaff({...newStaff,first:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Apellidos" value={newStaff.last} onChange={(e)=>setNewStaff({...newStaff,last:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newStaff.nation} onChange={(e)=>setNewStaff({...newStaff,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">{nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}</select>
                  <select value={newStaff.role} onChange={(e)=>setNewStaff({...newStaff,role:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option>coach</option><option>assistant</option><option>scout</option><option>physio</option></select>
                  <select value={newStaff.club} onChange={(e)=>setNewStaff({...newStaff,club:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option value="">Libre</option>{clubs.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}</select>
                  <input type="number" placeholder="Salario" value={newStaff.wage} onChange={(e)=>setNewStaff({...newStaff,wage:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveStaff} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newStaff) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Staff</th><th className="px-2 py-2">Rol</th><th className="px-2 py-2">Club</th><th className="px-2 py-2">Salario</th><th></th></tr></thead>
                    <tbody>{filtered.map((s:any)=><tr key={s.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold">{s.first_name} {s.last_name}</td><td className="px-2 py-1.5 text-center"><span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold">{s.role}</span></td><td className="px-2 py-1.5 text-xs">{s.club_name || "libre"}</td><td className="px-2 py-1.5 text-right font-mono text-xs">€{Math.round(s.wage_weekly).toLocaleString()}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setEditingStaff(s)} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delStaff(s.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="nations" && (
            <div className="space-y-4">
              {editingNation && <NationEditor nation={editingNation} confeds={confeds} onClose={()=>{ setEditingNation(null); load("nations"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newNation) ? "Editar nación · ID " + newNation.id : "Nueva nación"}</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newNation.name} onChange={(e)=>setNewNation({...newNation,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newNation.conf} onChange={(e)=>setNewNation({...newNation,conf:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">{confeds.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}</select>
                  <input type="number" placeholder="Rep" value={newNation.rep} onChange={(e)=>setNewNation({...newNation,rep:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Nivel" value={newNation.level} onChange={(e)=>setNewNation({...newNation,level:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveNation} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newNation) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">País</th><th className="px-2 py-2">Federación</th><th className="px-2 py-2">Rep</th><th className="px-2 py-2">Nivel</th><th></th></tr></thead>
                    <tbody>{filtered.map((n:any)=><tr key={n.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold">{n.name}</td><td className="px-2 py-1.5 text-xs">{n.confederation}</td><td className="px-2 py-1.5 text-center font-mono">{n.reputation}</td><td className="px-2 py-1.5 text-center font-mono">{n.futsal_level}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setEditingNation(n)} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delNation(n.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="competitions" && (
            <div className="space-y-4">
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newComp) ? "Editar competición · ID " + newComp.id : "Nueva competición"}</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newComp.name} onChange={(e)=>setNewComp({...newComp,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newComp.nation} onChange={(e)=>setNewComp({...newComp,nation:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option value="">Internacional</option>{nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}</select>
                  <input placeholder="Tier" value={newComp.tier} onChange={(e)=>setNewComp({...newComp,tier:e.target.value})} className="w-16 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Equipos" value={newComp.teams} onChange={(e)=>setNewComp({...newComp,teams:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Temporada" value={newComp.season} onChange={(e)=>setNewComp({...newComp,season:e.target.value})} className="w-32 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveComp} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newComp) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Competición</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Tier</th><th className="px-2 py-2">Equipos</th><th className="px-2 py-2">Temp.</th><th></th></tr></thead>
                    <tbody>{filtered.map((c:any)=><tr key={c.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold">{c.name}</td><td className="px-2 py-1.5 text-xs">{c.nation || "—"}</td><td className="px-2 py-1.5 text-center">{c.tier ?? "—"}</td><td className="px-2 py-1.5 text-center">{c.total_teams ?? "—"}</td><td className="px-2 py-1.5 text-xs">{c.season}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setNewComp({ id: c.id, name: c.name, nation: c.nation_id ? String(c.nation_id) : "", tier: c.tier ? String(c.tier) : "", teams: c.total_teams ?? 16, season: c.season })} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delComp(c.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}
