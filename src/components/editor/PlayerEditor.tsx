import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ImagePicker from "./ImagePicker";

type Attr = { ca:number; pa:number; position:string } & Record<string, number>;

const GROUPS: Record<string, string[]> = {
  "Técnica": ["firstTouch","dribbling","ballControl","technique","passing","vision","crossing","longShots","finishing","heading","penaltyTaking","tackling","marking","interception","blocking"],
  "Mental": ["anticipation","decisions","positioning","offTheBall","workRate","composure","concentration","determination","bravery","aggression","leadership","teamwork","flair"],
  "Físico": ["acceleration","pace","agility","balance","stamina","strength","jumping"],
  "Portero": ["reflexes","handling","oneOnOnes","positioningGk","rushingOut","throwing","kicking"],
  "Oculto": ["professionalism","consistency","importantMatches","injuryProneness"],
};

function Num({ label, v, onChange }: { label:string; v:number; onChange:(n:number)=>void }) {
  return (
    <label className="flex flex-col">
      <span className="text-[10px] uppercase tracking-wider text-fm-dim">{label}</span>
      <input type="number" min={1} max={20} value={v} onChange={(e)=>onChange(Number(e.target.value))} className="w-16 rounded border border-fm-border bg-fm-bg px-1.5 py-1 font-mono text-sm" />
    </label>
  );
}

export default function PlayerEditor({ player, nations, onClose }: { player:any; nations:any[]; onClose:()=>void }) {
  const [first, setFirst] = useState(player.first_name ?? "");
  const [last, setLast] = useState(player.last_name ?? "");
  const [nationId, setNationId] = useState(player.nation_id ?? 1);
  const [clubId, setClubId] = useState<string>(player.club_id ? String(player.club_id) : "");
  const [attrs, setAttrs] = useState<Attr | null>(null);
  const [msg, setMsg] = useState<string | null>(null);

  useEffect(() => {
    invoke<Attr>("editor_get_player_attributes", { playerId: player.id }).then(setAttrs).catch((e)=>setMsg(String(e)));
  }, [player.id]);

  const setAttr = (k:string, n:number) => setAttrs((a)=> a ? ({ ...(a as any), [k]: n }) : a);

  const saveIdentity = async () => {
    try {
      await invoke("editor_update_player", { id: player.id, first, last, nationId, clubId: clubId ? Number(clubId) : null, ca: attrs?.ca ?? 0, pa: attrs?.pa ?? 0, pos: attrs?.position ?? player.position });
      setMsg("Guardado");
    } catch (e) { setMsg(String(e)); }
  };
  const saveAttrs = async () => {
    if (!attrs) return;
    try {
      await invoke("editor_update_player_attributes", { playerId: player.id, attributes: attrs });
      setMsg("Atributos guardados");
    } catch (e) { setMsg(String(e)); }
  };

  if (!attrs) return <div className="p-6 text-center text-fm-dim">Cargando atributos…</div>;

  return (
    <div className="rounded-xl border border-sky-500/30 bg-fm-panel p-4">
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-lg font-black">Jugador · {player.first_name} {player.last_name} <span className="text-fm-dim">(ID {player.id})</span></h3>
        <button onClick={onClose} className="rounded-lg border border-fm-border px-3 py-1 text-sm text-fm-dim hover:text-white">Cerrar</button>
      </div>
      {msg && <div className="mb-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-3">
        <div className="space-y-3 rounded-lg border border-fm-border bg-fm-bg p-3">
          <ImagePicker command="editor_set_player_photo" entityId={player.id} label="Foto" value={player.photo_path ?? null} prefix="F" />
          <div className="space-y-2 text-sm">
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nombre</label><input value={first} onChange={(e)=>setFirst(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Apellidos</label><input value={last} onChange={(e)=>setLast(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nacionalidad</label>
              <select value={nationId} onChange={(e)=>setNationId(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1">
                {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
              </select>
            </div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Club ID</label><input value={clubId} onChange={(e)=>setClubId(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <button onClick={saveIdentity} className="mt-1 w-full rounded bg-fm-accent px-2 py-1.5 text-sm font-bold text-black">Guardar identidad</button>
          </div>
        </div>

        <div className="lg:col-span-2 space-y-4 rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="flex flex-wrap items-end gap-3">
            <div>
              <label className="block text-[10px] uppercase tracking-wider text-fm-dim">Posición</label>
              <select value={attrs.position} onChange={(e)=>setAttrs((a)=> a ? ({ ...(a as any), position: e.target.value }) : a)} className="rounded border border-fm-border bg-fm-panel px-2 py-1 text-sm">
                {["POR","CIE","ALA","PIV","UNI"].map((p)=> <option key={p} value={p}>{p}</option>)}
              </select>
            </div>
            <Num label="CA" v={attrs.ca} onChange={(n)=>setAttrs((a)=> a ? ({ ...(a as any), ca: n }) : a)} />
            <Num label="PA" v={attrs.pa} onChange={(n)=>setAttrs((a)=> a ? ({ ...(a as any), pa: n }) : a)} />
          </div>

          {Object.entries(GROUPS).map(([group, keys])=>(
            <div key={group}>
              <div className="mb-1 text-xs font-bold uppercase tracking-widest text-fm-dim">{group}</div>
              <div className="grid grid-cols-3 gap-2 sm:grid-cols-5 lg:grid-cols-8">
                {keys.map((k)=> <Num key={k} label={k} v={attrs[k] ?? 10} onChange={(n)=>setAttr(k, n)} />)}
              </div>
            </div>
          ))}

          <button onClick={saveAttrs} className="rounded bg-fm-accent px-4 py-1.5 text-sm font-bold text-black">Guardar atributos</button>
          <div className="text-xs text-fm-dim">Nota: el select de posición se aplica al guardar atributos (actualiza player_positions).</div>
        </div>
      </div>
    </div>
  );
}
