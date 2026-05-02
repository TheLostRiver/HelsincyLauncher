"use client";

import { User, LogIn } from "lucide-react";

export default function AccountLogin() {
  return (
    <div className="flex flex-col gap-3 rounded-[22px] bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-4 h-[170px]">
      {/* Header */}
      <div className="flex items-center gap-3 w-full">
        <div className="w-12 h-12 rounded-2xl bg-white flex items-center justify-center shrink-0">
          <User size={22} className="text-launcher-bg" />
        </div>
        <div className="flex flex-col gap-[3px] flex-1">
          <span className="font-sans text-[17px] font-bold text-white">
            账户登录
          </span>
          <span className="font-sans text-xs text-launcher-textDark">
            同步工程、资产和许可证
          </span>
        </div>
      </div>

      {/* Login Button */}
      <button className="flex items-center justify-center gap-2 h-[42px] w-full rounded-[21px] bg-launcher-accent hover:bg-[#5AACE5] transition-colors">
        <LogIn size={18} className="text-white" />
        <span className="font-sans text-[13px] font-bold text-white">
          使用 Epic 账户登录
        </span>
      </button>

      {/* License Row */}
      <div className="flex items-center justify-between w-full">
        <span className="font-sans text-xs text-launcher-textDark">
          组织许可证
        </span>
        <span className="font-mono text-[11px] text-launcher-warning">
          未连接
        </span>
      </div>
    </div>
  );
}
