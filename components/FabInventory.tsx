"use client";

import { Boxes, Plug } from "lucide-react";

const fabItems = [
  {
    icon: Boxes,
    iconColor: "text-[#9ED8FF]",
    name: "Nordic Megascans Pack",
    meta: "3.2 GB",
    bg: "bg-[#FFFFFF10]",
  },
  {
    icon: Plug,
    iconColor: "text-launcher-warning",
    name: "MetaHuman Animator",
    meta: "已安装",
    bg: "bg-[#FFFFFF0B]",
  },
];

export default function FabInventory() {
  return (
    <div className="flex flex-col gap-4 flex-1 rounded-[22px] bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-5">
      {/* Header */}
      <div className="flex items-start justify-between w-full">
        <div className="flex flex-col gap-[3px]">
          <span className="font-sans text-lg font-bold text-white">
            Fab 库存
          </span>
          <span className="font-sans text-xs text-launcher-textDark">
            资产、材质、插件
          </span>
        </div>
        <div className="px-2.5 py-[6px] rounded-full bg-launcher-accentDim border border-[#4A9FD866]">
          <span className="font-mono text-[11px] font-semibold text-[#9ED8FF]">
            284 items
          </span>
        </div>
      </div>

      {/* Asset List */}
      <div className="flex flex-col gap-2.5 flex-1">
        {fabItems.map((item) => {
          const Icon = item.icon;
          return (
            <div
              key={item.name}
              className={`flex items-center gap-3 flex-1 rounded-2xl ${item.bg} p-3`}
            >
              <Icon size={22} className={item.iconColor} />
              <span className="font-sans text-[13px] font-semibold text-white flex-1">
                {item.name}
              </span>
              <span className="font-mono text-xs text-launcher-textDark">
                {item.meta}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
