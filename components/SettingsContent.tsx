"use client";

import { useState } from "react";
import {
  SlidersHorizontal,
  Palette,
  Gauge,
  Bell,
  Globe,
  HardDrive,
  Power,
  User,
  Wifi,
  Info,
  ChevronDown,
} from "lucide-react";

type SettingsSection = "general" | "appearance" | "performance" | "notifications" | "language" | "storage";

const sideNavItems: { id: SettingsSection; icon: typeof SlidersHorizontal; label: string }[] = [
  { id: "general", icon: SlidersHorizontal, label: "通用" },
  { id: "appearance", icon: Palette, label: "外观" },
  { id: "performance", icon: Gauge, label: "性能" },
  { id: "notifications", icon: Bell, label: "通知" },
  { id: "language", icon: Globe, label: "语言" },
  { id: "storage", icon: HardDrive, label: "存储" },
];

function Toggle({ on, onToggle }: { on: boolean; onToggle: () => void }) {
  return (
    <button
      onClick={onToggle}
      className={`relative w-11 h-6 rounded-full shrink-0 transition-colors ${
        on ? "bg-launcher-accent" : "bg-[#FFFFFF20]"
      }`}
    >
      <div
        className={`absolute top-0.5 w-5 h-5 rounded-full bg-white transition-all ${
          on ? "left-[22px]" : "left-0.5"
        }`}
      />
    </button>
  );
}

function Dropdown({ value }: { value: string }) {
  return (
    <button className="flex items-center gap-2 h-10 rounded-[14px] bg-[#FFFFFF0B] border border-[#FFFFFF1F] px-3.5 hover:bg-[#FFFFFF10] transition-colors">
      <span className="font-sans text-[13px] text-white">{value}</span>
      <ChevronDown size={16} className="text-[#AEB6C2]" />
    </button>
  );
}

export default function SettingsContent() {
  const [activeSection, setActiveSection] = useState<SettingsSection>("general");
  const [toggles, setToggles] = useState({
    autoLaunch: true,
    minimizeOnStart: false,
    minimizeOnClose: true,
    notifications: true,
    notifSound: false,
    desktopNotif: true,
    proxy: false,
  });

  const handleToggle = (key: keyof typeof toggles) => {
    setToggles((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  return (
    <div className="flex gap-[18px] flex-1 min-h-0">
      {/* Settings Side Nav */}
      <div className="flex flex-col gap-1 w-[220px] shrink-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-4">
        {sideNavItems.map((item) => {
          const Icon = item.icon;
          const isActive = activeSection === item.id;
          return (
            <button
              key={item.id}
              onClick={() => setActiveSection(item.id)}
              className={`flex items-center gap-3 h-11 rounded-[14px] px-3.5 transition-colors ${
                isActive
                  ? "bg-[#FFFFFF1F] border border-[#FFFFFF2B] text-white"
                  : "text-[#AEB6C2] hover:bg-[#FFFFFF08] border border-transparent"
              }`}
            >
              <Icon size={18} />
              <span className="font-sans text-sm font-normal">
                {item.label}
              </span>
            </button>
          );
        })}
      </div>

      {/* Settings Panel */}
      <div className="flex flex-col gap-4 flex-1 min-w-0 rounded-3xl bg-launcher-panelAlt backdrop-blur-[18px] border border-launcher-border p-6 overflow-y-auto">
        {/* Panel Header */}
        <div className="flex flex-col gap-1">
          <span className="font-sans text-xl font-bold text-white">
            通用设置
          </span>
          <span className="font-sans text-[13px] text-[#8D96A4]">
            管理启动器的基本行为和偏好设置
          </span>
        </div>

        {/* Startup Section */}
        <div className="flex flex-col gap-3 rounded-[20px] bg-[#FFFFFF08] border border-[#FFFFFF1F] p-5">
          <div className="flex items-center gap-3">
            <Power size={20} className="text-launcher-accent" />
            <span className="font-sans text-[15px] font-semibold text-white">
              启动行为
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">开机自启动</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                系统启动时自动运行 Epic Games Launcher
              </span>
            </div>
            <Toggle
              on={toggles.autoLaunch}
              onToggle={() => handleToggle("autoLaunch")}
            />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">启动时最小化</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                启动器在系统启动后最小化到系统托盘
              </span>
            </div>
            <Toggle
              on={toggles.minimizeOnStart}
              onToggle={() => handleToggle("minimizeOnStart")}
            />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">
                关闭时最小化到托盘
              </span>
              <span className="font-sans text-xs text-[#8D96A4]">
                点击关闭按钮时最小化到系统托盘而不是退出
              </span>
            </div>
            <Toggle
              on={toggles.minimizeOnClose}
              onToggle={() => handleToggle("minimizeOnClose")}
            />
          </div>
        </div>

        {/* Notifications Section */}
        <div className="flex flex-col gap-3 rounded-[20px] bg-[#FFFFFF08] border border-[#FFFFFF1F] p-5">
          <div className="flex items-center gap-3">
            <Bell size={20} className="text-[#FF8400]" />
            <span className="font-sans text-[15px] font-semibold text-white">
              通知设置
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">启用通知</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                接收下载完成、游戏更新等通知
              </span>
            </div>
            <Toggle
              on={toggles.notifications}
              onToggle={() => handleToggle("notifications")}
            />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">通知声音</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                收到通知时播放提示音
              </span>
            </div>
            <Toggle
              on={toggles.notifSound}
              onToggle={() => handleToggle("notifSound")}
            />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">桌面通知</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                在系统通知中心显示通知
              </span>
            </div>
            <Toggle
              on={toggles.desktopNotif}
              onToggle={() => handleToggle("desktopNotif")}
            />
          </div>
        </div>

        {/* Language Section */}
        <div className="flex flex-col gap-3 rounded-[20px] bg-[#FFFFFF08] border border-[#FFFFFF1F] p-5">
          <div className="flex items-center gap-3">
            <Globe size={20} className="text-[#4DE0A7]" />
            <span className="font-sans text-[15px] font-semibold text-white">
              语言设置
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">界面语言</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                设置启动器界面显示语言
              </span>
            </div>
            <Dropdown value="简体中文" />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">内容区域</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                设置商店和推荐内容的区域
              </span>
            </div>
            <Dropdown value="中国大陆" />
          </div>
        </div>

        {/* Account Section */}
        <div className="flex flex-col gap-3 rounded-[20px] bg-[#FFFFFF08] border border-[#FFFFFF1F] p-5">
          <div className="flex items-center gap-3">
            <User size={20} className="text-[#4DE0A7]" />
            <span className="font-sans text-[15px] font-semibold text-white">
              账户信息
            </span>
          </div>
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-xl bg-launcher-accent shrink-0" />
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-[13px] font-semibold text-white">
                Player_01
              </span>
              <span className="font-sans text-xs text-[#8D96A4]">
                player@example.com
              </span>
            </div>
          </div>
          <div className="flex gap-3">
            <button className="flex items-center justify-center h-10 rounded-[22px] bg-launcher-accent px-5 hover:opacity-90 transition-opacity">
              <span className="font-sans text-xs font-semibold text-white">
                编辑资料
              </span>
            </button>
            <button className="flex items-center justify-center h-10 rounded-[22px] bg-[#FFFFFF1F] px-5 hover:bg-[#FFFFFF2A] transition-colors">
              <span className="font-sans text-xs font-semibold text-white">
                退出登录
              </span>
            </button>
          </div>
        </div>

        {/* Network Section */}
        <div className="flex flex-col gap-3 rounded-[20px] bg-[#FFFFFF08] border border-[#FFFFFF1F] p-5">
          <div className="flex items-center gap-3">
            <Wifi size={20} className="text-[#E0A84D]" />
            <span className="font-sans text-[15px] font-semibold text-white">
              网络设置
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">
                下载带宽限制
              </span>
              <span className="font-sans text-xs text-[#8D96A4]">
                限制下载速度以避免影响其他网络活动
              </span>
            </div>
            <Dropdown value="无限制" />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">使用代理</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                通过代理服务器连接网络
              </span>
            </div>
            <Toggle
              on={toggles.proxy}
              onToggle={() => handleToggle("proxy")}
            />
          </div>
          <div className="flex items-center justify-between w-full">
            <div className="flex flex-col gap-0.5">
              <span className="font-sans text-sm text-white">清除缓存</span>
              <span className="font-sans text-xs text-[#8D96A4]">
                当前缓存大小: 2.3 GB
              </span>
            </div>
            <button className="flex items-center justify-center h-9 rounded-[18px] bg-[#FFFFFF12] border border-[#FFFFFF24] px-[18px] hover:bg-[#FFFFFF1A] transition-colors">
              <span className="font-sans text-[13px] font-semibold text-white">
                清除
              </span>
            </button>
          </div>
        </div>

        {/* About Section */}
        <div className="flex flex-col gap-3 rounded-[20px] bg-[#FFFFFF08] border border-[#FFFFFF1F] p-5">
          <div className="flex items-center gap-3">
            <Info size={20} className="text-[#AEB6C2]" />
            <span className="font-sans text-[15px] font-semibold text-white">
              关于
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-[13px] text-[#8D96A4]">
              版本号
            </span>
            <span className="font-sans text-[13px] font-semibold text-white">
              v2.4.1
            </span>
          </div>
          <div className="flex items-center justify-between w-full">
            <span className="font-sans text-[13px] text-[#8D96A4]">
              检查更新
            </span>
            <button className="flex items-center justify-center h-10 rounded-[22px] bg-launcher-accent px-5 hover:opacity-90 transition-opacity">
              <span className="font-sans text-xs font-semibold text-white">
                检查更新
              </span>
            </button>
          </div>
          <span className="font-sans text-xs text-[#8D96A4]">
            &copy; 2026 Epic Games, Inc.
          </span>
        </div>
      </div>
    </div>
  );
}
