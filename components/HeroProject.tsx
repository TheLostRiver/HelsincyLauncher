"use client";

import { Play, Ellipsis, MountainSnow } from "lucide-react";

export default function HeroProject() {
  return (
    <div className="flex rounded-3xl bg-launcher-panel backdrop-blur-[22px] border border-launcher-border shadow-[0_18px_64px_rgba(0,0,0,0.6)] overflow-hidden h-[400px]">
      {/* Project Info */}
      <div className="flex flex-col justify-center gap-[18px] flex-1 p-6">
        {/* Status Row */}
        <div className="flex items-center gap-2.5">
          <div className="flex items-center gap-2 px-3 py-[7px] rounded-full bg-launcher-successBg border border-[#4DE0A766]">
            <div className="w-2 h-2 rounded-full bg-launcher-success" />
            <span className="font-sans text-xs font-semibold text-[#B6FFCE]">
              已同步
            </span>
          </div>
          <div className="px-3 py-[7px] rounded-full bg-[#FFFFFF10] border border-launcher-borderLight">
            <span className="font-mono text-xs font-semibold text-[#D9DEE7]">
              UE 5.4.4
            </span>
          </div>
        </div>

        {/* Title */}
        <h1 className="font-sans text-[42px] font-bold text-white leading-[1.15]">
          Aurora Facility
        </h1>

        {/* Description */}
        <p className="font-sans text-[15px] text-launcher-textSecondary leading-[1.45]">
          最近编辑的虚幻工程，包含 Fab 场景资产、Nanite 环境套件和团队共享插件。
        </p>

        {/* Action Buttons */}
        <div className="flex items-center gap-3">
          <button className="flex items-center gap-2 h-11 rounded-[22px] bg-white px-[18px] hover:bg-gray-100 transition-colors">
            <Play size={18} className="text-launcher-bg" />
            <span className="font-sans text-sm font-bold text-launcher-bg">
              打开工程
            </span>
          </button>
          <button className="flex items-center justify-center w-11 h-11 rounded-[22px] bg-launcher-hover border border-launcher-border hover:bg-launcher-hoverLight transition-colors">
            <Ellipsis size={20} className="text-white" />
          </button>
        </div>

        {/* Meta */}
        <div className="flex items-end justify-end">
          <span className="font-mono text-xs text-launcher-textMuted">
            12 个本地工程
          </span>
        </div>
      </div>

      {/* Preview Panel */}
      <div className="w-[280px] shrink-0 rounded-[20px] m-6 relative overflow-hidden bg-gradient-to-br from-[#244A60] to-[#0C1016] border border-[#FFFFFF22]">
        <div className="absolute top-6 left-6 w-[232px] h-[200px] rounded-2xl bg-[#FFFFFF0F] border border-[#FFFFFF1A]" />
        <div className="absolute top-[60px] left-[35px] w-[210px] h-[210px] rounded-full bg-[#4A9FD84D] blur-[36px]" />
        <div className="absolute inset-0 flex items-center justify-center">
          <MountainSnow size={88} className="text-[#FFFFFFCC]" />
        </div>
      </div>
    </div>
  );
}
