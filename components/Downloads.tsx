"use client";

import { FolderOpen } from "lucide-react";

export default function Downloads() {
  return (
    <div className="flex flex-col gap-3 rounded-[22px] bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-4 h-[242px]">
      {/* Header */}
      <div className="flex items-center justify-between w-full">
        <span className="font-sans text-lg font-bold text-white">下载</span>
        <span className="font-mono text-xs text-[#9ED8FF]">42.8 MB/s</span>
      </div>

      {/* Active Download */}
      <div className="flex flex-col gap-2 rounded-[18px] bg-[#FFFFFF10] p-3 h-[82px]">
        <div className="flex items-center justify-between w-full">
          <span className="font-sans text-[13px] font-bold text-white">
            City Sample Assets
          </span>
          <span className="font-mono text-xs text-white">68%</span>
        </div>
        <div className="w-full h-2.5 rounded-full bg-[#FFFFFF18] overflow-hidden">
          <div className="h-full w-[68%] rounded-full bg-launcher-accent" />
        </div>
      </div>

      {/* Queued Download */}
      <div className="flex flex-col gap-1.5 rounded-[18px] bg-[#FFFFFF0B] p-3 flex-1">
        <div className="flex items-center justify-between w-full">
          <span className="font-sans text-[13px] font-bold text-white">
            UE 5.5 Preview
          </span>
          <span className="font-mono text-xs text-launcher-textDark">
            等待中
          </span>
        </div>
        <div className="flex gap-2 mt-auto">
          <button className="flex items-center justify-center flex-1 h-[34px] rounded-[17px] bg-launcher-hover border border-[#FFFFFF20] hover:bg-launcher-hoverLight transition-colors">
            <span className="font-sans text-xs font-bold text-white">
              暂停全部
            </span>
          </button>
          <button className="flex items-center justify-center w-10 h-[34px] rounded-[17px] bg-launcher-hover border border-[#FFFFFF20] hover:bg-launcher-hoverLight transition-colors">
            <FolderOpen size={17} className="text-white" />
          </button>
        </div>
      </div>
    </div>
  );
}
