import { useState } from "react";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

export default function ImagePicker({ command, entityId, label, value, prefix, size = "h-16 w-16" }: {
  command: string; entityId: number; label: string; value: string | null; prefix: string; size?: string;
}) {
  const [src, setSrc] = useState<string | null>(value ?? null);
  const [err, setErr] = useState<string | null>(null);

  const onPick = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = async () => {
      const dataUrl = reader.result as string;
      const base64 = dataUrl.split(",")[1] || "";
      const ext = (dataUrl.match(/data:image\/(\w+)/)?.[1] || "png").toLowerCase();
      try {
        const path = await invoke<string>(command, { clubId: entityId, playerId: entityId, staffId: entityId, nationId: entityId, confedId: entityId, data: base64, ext });
        setSrc(path);
        setErr(null);
      } catch (e2) { setErr(String(e2)); }
    };
    reader.readAsDataURL(file);
  };

  return (
    <div>
      <div className={`flex ${size} items-center justify-center overflow-hidden rounded-lg border border-fm-border bg-fm-panel`}>
        {src ? <img src={convertFileSrc(src)} alt={label} className="h-full w-full object-contain" /> : <span className="text-fm-dim">{prefix}</span>}
      </div>
      <label className="mt-1 block cursor-pointer rounded border border-fm-border bg-fm-panel2 px-2 py-1 text-center text-xs font-semibold text-fm-dim hover:text-white">
        Elegir…
        <input type="file" accept="image/*" className="hidden" onChange={onPick} />
      </label>
      {err && <div className="mt-1 text-xs text-red-400">{err}</div>}
    </div>
  );
}
