import { useEffect, useState } from "react";
import { api, type InboxRow } from "../../api";

export default function InboxView() {
  const [msgs, setMsgs] = useState<InboxRow[]>([]);
  const [loading, setLoading] = useState(true);

  const load = async () => {
    setLoading(true);
    try { setMsgs(await api.getInbox()); } finally { setLoading(false); }
  };
  useEffect(()=>{ load(); },[]);

  const mark = async (id:number) => {
    await api.markRead(id);
    setMsgs((m)=>m.map((x)=> x.id===id ? {...x, is_read:1}:x));
  };
  const markAll = async () => {
    await api.markAllRead();
    setMsgs((m)=> m.map((x)=>({...x, is_read:1})));
  };

  if (loading) return <div className="p-8 text-center text-fm-dim">Cargando bandeja…</div>;

  const unread = msgs.filter((m)=> m.is_read===0).length;

  return (
    <div className="mx-auto max-w-4xl space-y-4 p-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-black">Bandeja de entrada {unread>0 && <span className="ml-2 rounded-full bg-fm-accent px-2 py-0.5 text-xs font-bold text-black">{unread} sin leer</span>}</h2>
        <button onClick={markAll} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-1.5 text-sm">Marcar todo leído</button>
      </div>
      {msgs.length===0 ? <div className="rounded-xl border border-fm-border bg-fm-panel p-8 text-center text-fm-dim">Sin mensajes.</div> : (
        <div className="space-y-2">
          {msgs.map((m)=>(
            <div key={m.id} onClick={()=>mark(m.id)} className={`cursor-pointer rounded-xl border p-4 ${m.is_read ? "border-fm-border bg-fm-panel opacity-80" : "border-fm-accent/30 bg-fm-panel"} ${m.is_important ? "ring-1 ring-amber-500/30" : ""}`}>
              <div className="flex items-start justify-between gap-3">
                <div>
                  <div className="flex items-center gap-2">
                    <span className={`rounded px-1.5 py-0.5 text-xs font-bold ${m.sender==="board"?"bg-sky-500/20 text-sky-400":m.sender==="staff"?"bg-amber-500/20 text-amber-400":"bg-fm-bg text-fm-dim"}`}>{m.sender}</span>
                    {m.is_important===1 && <span className="rounded bg-amber-500 px-1.5 py-0.5 text-xs font-bold text-black">!</span>}
                    <span className="font-semibold">{m.subject}</span>
                  </div>
                  <div className="mt-1 text-sm text-fm-dim">{m.body}</div>
                </div>
                <span className="shrink-0 font-mono text-xs text-fm-dim">{m.date}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
