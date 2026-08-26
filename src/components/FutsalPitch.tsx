import { Stage, Layer, Rect, Circle, Line, Text, Group } from "react-konva";
import type { MatchSnapshot } from "../api";

const W = 820;
const H = 420;
const SCALE = 20;

export default function FutsalPitch({ snap }: { snap?: MatchSnapshot }) {
  const players = snap?.players.filter((p) => p.on_pitch) ?? [];
  const ball = snap ? { x: snap.ball[0] * SCALE, y: snap.ball[1] * SCALE } : null;
  const colors = ["#A50044", "#004D98"];

  return (
    <div className="overflow-hidden rounded-xl border border-fm-border bg-[#1b5e20]">
      <Stage width={W} height={H}>
        <Layer>
          <Rect x={0} y={0} width={W} height={H} fill="#2d8a2d" />
          <Rect x={10} y={10} width={W-20} height={H-20} stroke="#fff" strokeWidth={2} />
          <Line points={[W/2,10,W/2,H-10]} stroke="#fff" strokeWidth={2} />
          <Circle x={W/2} y={H/2} radius={40} stroke="#fff" strokeWidth={2} />
          <Rect x={10} y={120} width={120} height={160} stroke="#fff" strokeWidth={2} />
          <Rect x={W-130} y={120} width={120} height={160} stroke="#fff" strokeWidth={2} />
          <Circle x={130} y={H/2} radius={3} fill="#fff" />
          <Circle x={W-130} y={H/2} radius={3} fill="#fff" />
          <Circle x={200} y={H/2} radius={3} fill="#fff" />
          <Circle x={W-200} y={H/2} radius={3} fill="#fff" />

          {!snap && <Text x={W/2-90} y={H/2-8} text="Pulsa Iniciar para ver el motor en vivo" fontSize={12} fill="#fff" opacity={0.9} />}

          {players.map((p) => (
            <Group key={p.id} x={p.x * SCALE} y={p.y * SCALE}>
              <Circle radius={10} fill={colors[p.team_id] ?? "#fff"} stroke="#000" strokeWidth={1.5} opacity={p.stamina < 35 ? 0.65 : 1} />
              <Text text={String(p.shirt)} fontSize={10} fill="#fff" offsetX={5} offsetY={4} fontStyle="bold" />
              {p.id === snap?.ball_holder && <Circle radius={13} stroke="#FFD600" strokeWidth={2} dash={[4,3]} />}
            </Group>
          ))}

          {ball && <Circle x={ball.x} y={ball.y} radius={6} fill="#fff" stroke="#000" strokeWidth={1} />}

          {snap && (
            <Group>
              <Rect x={W/2-70} y={8} width={140} height={18} fill="#000" opacity={0.55} cornerRadius={4} />
              <Text x={W/2-55} y={12} text={`${snap.score[0]} - ${snap.score[1]} · ${String(Math.floor(snap.time_seconds/60)).padStart(2,"0")}:${String(snap.time_seconds%60).padStart(2,"0")}`} fontSize={12} fill="#FFD600" fontStyle="bold" />
            </Group>
          )}
        </Layer>
      </Stage>
    </div>
  );
}
