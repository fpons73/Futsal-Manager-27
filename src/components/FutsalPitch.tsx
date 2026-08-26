import { Stage, Layer, Rect, Circle, Line, Text } from "react-konva";

const W = 820;
const H = 420;
const SCALE = 20;

export default function FutsalPitch() {
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
          <Text x={W/2-60} y={H/2-8} text="Motor 2D en vivo — próximamente" fontSize={12} fill="#fff" opacity={0.9} />
          <Circle x={10+4*SCALE} y={10} radius={6} fill="#fff" stroke="#000" strokeWidth={1} />
          <Text x={12} y={18} text="Demo campo 40×20m" fontSize={10} fill="#fff" />
        </Layer>
      </Stage>
    </div>
  );
}
