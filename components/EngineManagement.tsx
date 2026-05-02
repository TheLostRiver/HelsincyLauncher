"use client";

import { Box, Rocket, Plus } from "lucide-react";

const plugins = [
  { name: "Fab Bridge", status: "Ready", statusColor: "text-[#B6FFCE]" },
  { name: "Quixel Tools", status: "Update", statusColor: "text-launcher-warning" },
];

export default function EngineManagement() {
  return (
    <div className="flex flex-col gap-2.5 flex-1 rounded-[22px] bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-4">
      {/* Title */}
      <span className="font-sans text-lg font-bold text-white">引擎管理</span>

      {/* Current Engine */}
      <div className="flex items-center gap-3.5 rounded-[18px] bg-launcher-hover border border-launcher-borderLight p-3 h-[70px]">
        <Box size={28} className="text-white shrink-0" />
        <div className="flex flex-col gap-1 flex-1">
          <span className="font-sans text-[15px] font-bold text-white">
            Unreal Engine 5.4.4
          </span>
          <span className="font-mono text-[11px] text-launcher-textDark">
            源构建可用 · 56.8 GB
          </span>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2.5 h-[34px]">
        <button className="flex items-center justify-center gap-2 flex-1 rounded-[20px] bg-white hover:bg-gray-100 transition-colors">
          <Rocket size={18} className="text-launcher-bg" />
          <span className="font-sans text-[13px] font-bold text-launcher-bg">
            启动
          </span>
        </button>
        <button className="flex items-center justify-center gap-2 flex-1 rounded-[20px] bg-launcher-hover border border-launcher-border hover:bg-launcher-hoverLight transition-colors">
          <Plus size={18} className="text-white" />
          <span className="font-sans text-[13px] font-bold text-white">
            安装版本
          </span>
        </button>
      </div>

      {/* Plugin Compatibility */}
      <div className="flex flex-col gap-2 rounded-[18px] bg-[#FFFFFF0B] p-2.5 flex-1">
        <span className="font-sans text-[13px] font-bold text-white">
          插件兼容性
        </span>
        {plugins.map((p) => (
          <div
            key={p.name}
            className="flex items-center justify-between w-full"
          >
            <span className="font-sans text-xs text-launcher-textSecondary">
              {p.name}
            </span>
            <span className={`font-mono text-[11px] ${p.statusColor}`}>
              {p.status}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
