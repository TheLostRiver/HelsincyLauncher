"use client";

import { SlidersHorizontal, HardDrive } from "lucide-react";

export default function Settings() {
  return (
    <div className="flex flex-col gap-3 rounded-[22px] bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-[18px] flex-1">
      {/* Header */}
      <div className="flex items-center justify-between w-full">
        <span className="font-sans text-lg font-bold text-white">设置</span>
        <SlidersHorizontal size={20} className="text-launcher-textSecondary" />
      </div>

      {/* Toggle Row 1 */}
      <div className="flex items-center justify-between w-full h-[46px]">
        <span className="font-sans text-[13px] text-[#D9DEE7]">
          自动更新引擎
        </span>
        <div className="flex items-center justify-end w-[42px] h-6 rounded-full bg-launcher-accent p-[3px] cursor-pointer">
          <div className="w-[18px] h-[18px] rounded-full bg-white" />
        </div>
      </div>

      {/* Toggle Row 2 */}
      <div className="flex items-center justify-between w-full h-[46px]">
        <span className="font-sans text-[13px] text-[#D9DEE7]">
          下载后校验文件
        </span>
        <div className="flex items-center w-[42px] h-6 rounded-full bg-[#FFFFFF18] p-[3px] cursor-pointer">
          <div className="w-[18px] h-[18px] rounded-full bg-launcher-textDark" />
        </div>
      </div>

      {/* Path Box */}
      <div className="flex items-center gap-2.5 rounded-2xl bg-[#FFFFFF0B] px-3 py-2.5 h-12">
        <HardDrive size={18} className="text-launcher-textDark shrink-0" />
        <span className="font-mono text-xs text-launcher-textSecondary">
          D:/EpicLibrary
        </span>
      </div>
    </div>
  );
}
