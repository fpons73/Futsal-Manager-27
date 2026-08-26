import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Tab = "nations" | "clubs" | "players" | "competitions" | "stadiums";

export default function EditorView() {
  const [tab, setTab] = useState<Tab>("clubs");
  const [data, setData] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [msg, setMsg] = useState<string | null>(null);

  // Forms
  const [newNation, setNewNation] = useState({ name: "", conf: 1, rep: 500, level: 50 });
  const [newClub, setNewClub] = useState({ name: "", short: "", nation: 1, city: "", stadium: "", cap: 2000, rep: 600, c1: "#0f4c3a", c2: "#ffffff" });
  const [newPlayer, setNewPlayer] = useState({ first: "", last: "", nation: 1, club: "", ca: 80, pa: 120, pos: "ALA" });
  const [newComp, setNewComp] = useState({ name: "", nation: "", tier: "", teams: 16, season: "2026/2027" });
  const [confeds, setConfeds] = useState<any[]>([]);
  const [nations, setNations] = useState<any[]>([]);

  const load = async (t: Tab) => {
    setLoading(true);
    try {
      if (t === "nations") setData(await invoke<any[]>("editor_list_nations"));
      else if (t === "clubs") setData(await invoke<any[]>("editor_list_clubs"));
      else if (t === "players") setData(await invoke<any[]>("editor_list_players", { limit: 100 }));
      else if (t === "competitions") setData(await invoke<any[]>("editor_list_competitions"));
      else if (t === "stadiums") setData(await invoke<any[]>("editor_list_stadiums"));
      setMsg(null);
    } catch (e) { setMsg(String(e)); }
    finally { setLoading(false); }
  };

  useEffect(() => { load(tab); }, [tab]);
  useEffect(() => {
    // Asegurar que la BD exista (auto-seed si vacía) y luego cargar listas
    invoke<number>("editor_init")
      .then(async () => {
        const [c, n] = await Promise.all([
          invoke<any[]>("editor_list_confederations").catch(()=>[]),
          invoke<any[]>("editor_list_nations").catch(()=>[]),
        ]);
        setConfeds(c);
        setNations(n);
        load(tab);
      })
      .catch(()=>{});
  }, [tab]);

  const createNation = async () => {
    try { await invoke("editor_create_nation", { name: newNation.name, confederationId: newNation.conf, reputation: newNation.rep, futsalLevel: newNation.level }); setMsg("Nación creada"); load("nations"); } catch(e){ setMsg(String(e)); }
  };
  const delNation = async (id:number) => { if(!confirm("¿Borrar nación?")) return; try{ await invoke("editor_delete_nation",{id}); load("nations"); }catch(e){ setMsg(String(e)); } };
  const createClub = async () => {
    try{ await invoke("editor_create_club", { name: newClub.name, shortName: newClub.short, nationId: newClub.nation, city: newClub.city, stadium: newClub.stadium, capacity: newClub.cap, reputation: newClub.rep, c1: newClub.c1, c2: newClub.c2 }); setMsg("Club creado"); load("clubs"); }catch(e){ setMsg(String(e)); }
  };
  const delClub = async (id:number) => { if(!confirm("¿Borrar club? También borrará contratos/plantilla asociada")) return; try{ await invoke("editor_delete_club",{id}); load("clubs"); }catch(e){ setMsg(String(e)); } };
  const createPlayer = async () => {
    try{ await invoke("editor_create_player", { first: newPlayer.first, last: newPlayer.last, nationId: newPlayer.nation, clubId: newPlayer.club ? Number(newPlayer.club) : null, ca: newPlayer.ca, pa: newPlayer.pa, pos: newPlayer.pos }); setMsg("Jugador creado"); load("players"); }catch(e){ setMsg(String(e)); }
  };
  const delPlayer = async (id:number) => { if(!confirm("¿Borrar jugador?")) return; try{ await invoke("editor_delete_player",{id}); load("players"); }catch(e){ setMsg(String(e)); } };
  const createComp = async () => {
    try{ await invoke("editor_create_competition", { name: newComp.name, nationId: newComp.nation ? Number(newComp.nation) : null, tier: newComp.tier ? Number(newComp.tier) : null, totalTeams: newComp.teams, season: newComp.season }); setMsg("Competición creada"); load("competitions"); }catch(e){ setMsg(String(e)); }
  };
  const delComp = async (id:number) => { if(!confirm("¿Borrar competición?")) return; try{ await invoke("editor_delete_competition",{id}); load("competitions"); }catch(e){ setMsg(String(e)); } };

  return (
    <div className="mx-auto max-w-6xl space-y-4 p-6">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <h2 className="text-xl font-black">Editor — Base de Datos</h2>
        <span className="rounded-full bg-fm-accent px-3 py-1 text-xs font-bold text-black">PRD completo: 26 competiciones</span>
      </div>
      <p className="text-sm text-fm-dim">Crea, edita y elimina países, clubes, jugadores y competiciones. Los cambios se aplican a la partida actual (fichero en %APPDATA%/FutsalManager27). Para lanzar con todas las competiciones del PRD, usa “Nueva partida” tras editar.</p>

      <div className="flex flex-wrap gap-1">
        {(["clubs","players","nations","competitions","stadiums"] as Tab[]).map((t)=>(
          <button key={t} onClick={()=>setTab(t)} className={`rounded-lg px-3 py-1.5 text-sm font-semibold ${tab===t ? "bg-fm-accent text-black" : "bg-fm-panel border border-fm-border text-fm-dim"}`}>{t.toUpperCase()}</button>
        ))}
      </div>

      {msg && <div className="rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      {loading ? <div className="p-8 text-center text-fm-dim">Cargando…</div> : (
        <>
          {tab==="nations" && (
            <div className="space-y-3">
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Nueva nación</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newNation.name} onChange={(e)=>setNewNation({...newNation,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newNation.conf} onChange={(e)=>setNewNation({...newNation,conf:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    {confeds.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}
                  </select>
                  <input type="number" placeholder="Rep 0-1000" value={newNation.rep} onChange={(e)=>setNewNation({...newNation,rep:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Nivel 0-100" value={newNation.level} onChange={(e)=>setNewNation({...newNation,level:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={createNation} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Crear</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <table className="w-full text-sm">
                  <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">ID</th><th className="px-2 py-2 text-left">Nombre</th><th className="px-2 py-2">Confed.</th><th className="px-2 py-2">Rep</th><th className="px-2 py-2">Nivel</th><th></th></tr></thead>
                  <tbody>{data.map((n:any)=><tr key={n.id} className="border-t border-fm-border hover:bg-fm-panel2"><td className="px-2 py-1.5 font-mono">{n.id}</td><td className="px-2 py-1.5 font-semibold">{n.name}</td><td className="px-2 py-1.5 text-center">{n.confederation}</td><td className="px-2 py-1.5 text-center">{n.reputation}</td><td className="px-2 py-1.5 text-center">{n.futsal_level}</td><td className="px-2 py-1.5 text-right"><button onClick={()=>delNation(n.id)} className="rounded bg-red-600 px-2 py-1 text-xs text-white">Borrar</button></td></tr>)}</tbody>
                </table>
              </div>
            </div>
          )}

          {tab==="clubs" && (
            <div className="space-y-3">
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Nuevo club</div>
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
                  <input placeholder="Color1 #hex" value={newClub.c1} onChange={(e)=>setNewClub({...newClub,c1:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                </div>
                <button onClick={createClub} className="mt-2 rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Crear club (+12 jugadores)</button>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-96 overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Club</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Ciudad</th><th className="px-2 py-2">Rep</th><th></th></tr></thead>
                    <tbody>{data.map((c:any)=><tr key={c.id} className="border-t border-fm-border hover:bg-fm-panel2"><td className="px-2 py-1.5"><span className="font-semibold">{c.name}</span> <span className="text-fm-dim">({c.short_name})</span></td><td className="px-2 py-1.5 text-xs">{c.nation}</td><td className="px-2 py-1.5 text-xs">{c.city}</td><td className="px-2 py-1.5 text-center font-mono">{c.reputation}</td><td className="px-2 py-1.5 text-right"><button onClick={()=>delClub(c.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td></tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="players" && (
            <div className="space-y-3">
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Nuevo jugador</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newPlayer.first} onChange={(e)=>setNewPlayer({...newPlayer,first:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Apellidos" value={newPlayer.last} onChange={(e)=>setNewPlayer({...newPlayer,last:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newPlayer.nation} onChange={(e)=>setNewPlayer({...newPlayer,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
                  </select>
                  <input placeholder="Club ID (opcional)" value={newPlayer.club} onChange={(e)=>setNewPlayer({...newPlayer,club:e.target.value})} className="w-28 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newPlayer.pos} onChange={(e)=>setNewPlayer({...newPlayer,pos:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    <option>POR</option><option>CIE</option><option>ALA</option><option>PIV</option><option>UNI</option>
                  </select>
                  <input type="number" placeholder="CA" value={newPlayer.ca} onChange={(e)=>setNewPlayer({...newPlayer,ca:Number(e.target.value)})} className="w-16 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="PA" value={newPlayer.pa} onChange={(e)=>setNewPlayer({...newPlayer,pa:Number(e.target.value)})} className="w-16 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={createPlayer} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Crear</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-96 overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Jugador</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Club</th><th className="px-2 py-2">Pos</th><th className="px-2 py-2">CA</th><th></th></tr></thead>
                    <tbody>{data.map((p:any)=><tr key={p.id} className="border-t border-fm-border hover:bg-fm-panel2"><td className="px-2 py-1.5 font-semibold">{p.first_name} {p.last_name}</td><td className="px-2 py-1.5 text-xs">{p.nation}</td><td className="px-2 py-1.5 text-xs">{p.club || "-"}</td><td className="px-2 py-1.5 text-center text-xs">{p.position}</td><td className="px-2 py-1.5 text-center font-mono">{p.ca}</td><td className="px-2 py-1.5 text-right"><button onClick={()=>delPlayer(p.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td></tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="competitions" && (
            <div className="space-y-3">
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Nueva competición</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newComp.name} onChange={(e)=>setNewComp({...newComp,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newComp.nation} onChange={(e)=>setNewComp({...newComp,nation:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    <option value="">Internacional</option>
                    {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
                  </select>
                  <input placeholder="Tier" value={newComp.tier} onChange={(e)=>setNewComp({...newComp,tier:e.target.value})} className="w-16 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" value={newComp.teams} onChange={(e)=>setNewComp({...newComp,teams:Number(e.target.value)})} className="w-20 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={createComp} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Crear</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <table className="w-full text-sm">
                  <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Competición</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Tier</th><th className="px-2 py-2">Equipos</th><th className="px-2 py-2">Temp.</th><th></th></tr></thead>
                  <tbody>{data.map((c:any)=><tr key={c.id} className="border-t border-fm-border hover:bg-fm-panel2"><td className="px-2 py-1.5 font-semibold">{c.name}</td><td className="px-2 py-1.5 text-xs">{c.nation || "—"}</td><td className="px-2 py-1.5 text-center">{c.tier ?? "—"}</td><td className="px-2 py-1.5 text-center">{c.total_teams ?? "—"}</td><td className="px-2 py-1.5 text-xs">{c.season}</td><td className="px-2 py-1.5 text-right"><button onClick={()=>delComp(c.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td></tr>)}</tbody>
                </table>
              </div>
              <div className="rounded bg-sky-500/10 p-3 text-xs text-sky-200">La app ya arranca con las 26 competiciones del PRD (España 1ª, 6×Segunda B, Copa, Supercopa, Brasil LNF/Taça, Portugal Liga/Taça, Rusia, Italia A/A2, Kazajistán, Irán, Argentina, Ucrania, Francia, Japón, Croacia, Chequia, Serbia, Polonia, Azerbaiyán, Colombia, Tailandia, Uruguay + 7 internacionales). Edita o añade aquí y lanza “Nueva partida”.</div>
            </div>
          )}

          {tab==="stadiums" && (
            <div className="rounded-xl border border-fm-border bg-fm-panel p-6 text-center text-sm text-fm-dim">
              Pabellones y ciudades se gestionan al crear/editar clubes. Usa la pestaña Clubes para cambiar ciudad/pabellón/capacidad. Próximamente edición directa de pabellones y staff.
            </div>
          )}
        </>
      )}
    </div>
  );
}
