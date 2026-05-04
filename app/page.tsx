"use client";

import { useState, useCallback } from "react";
import Sidebar, { type PageId } from "@/components/Sidebar";
import TopBar from "@/components/TopBar";
import HomeContent from "@/components/HomeContent";
import MyProjectsContent from "@/components/MyProjectsContent";
import FabInventoryContent from "@/components/FabInventoryContent";
import EngineManagementContent from "@/components/EngineManagementContent";
import DownloadsContent from "@/components/DownloadsContent";
import SettingsContent from "@/components/SettingsContent";

const pageMeta: Record<PageId, { kicker: string; title: string }> = {
  home: { kicker: "UNREAL WORKSPACE", title: "首页" },
  "my-projects": { kicker: "PROJECT LIBRARY", title: "我的工程" },
  fab: { kicker: "FAB INVENTORY", title: "Fab 库存" },
  engine: { kicker: "ENGINE MANAGER", title: "引擎管理" },
  downloads: { kicker: "DOWNLOAD CENTER", title: "下载中心" },
  settings: { kicker: "SETTINGS", title: "通用设置" },
};

export default function Home() {
  const [activePage, setActivePage] = useState<PageId>("home");
  const [transitioning, setTransitioning] = useState(false);

  const handleNavigate = useCallback(
    (page: PageId) => {
      if (page === activePage || transitioning) return;
      setTransitioning(true);
      setTimeout(() => {
        setActivePage(page);
        setTimeout(() => setTransitioning(false), 50);
      }, 200);
    },
    [activePage, transitioning]
  );

  const meta = pageMeta[activePage];

  return (
    <main className="relative w-screen h-screen bg-launcher-bg overflow-hidden">
      {/* Background effects */}
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-0 right-[120px] w-[500px] h-[500px] rounded-full bg-[#4A9FD833] blur-[72px]" />
        <div className="absolute bottom-0 right-0 w-[320px] h-[320px] rounded-full bg-[#FF840026] blur-[64px]" />
      </div>

      {/* Launcher Shell */}
      <div className="relative z-10 flex gap-5 p-5 h-full">
        {/* Sidebar */}
        <Sidebar activePage={activePage} onNavigate={handleNavigate} />

        {/* Main Workspace */}
        <div className="flex flex-col gap-[18px] flex-1 h-full min-w-0">
          {/* Top Bar */}
          <TopBar kicker={meta.kicker} title={meta.title} />

          {/* Content Area with transition */}
          <div
            className={`flex-1 min-h-0 transition-all duration-200 ease-out ${
              transitioning
                ? "opacity-0 translate-y-1"
                : "opacity-100 translate-y-0"
            }`}
          >
            {activePage === "home" && <HomeContent />}
            {activePage === "my-projects" && <MyProjectsContent />}
            {activePage === "fab" && <FabInventoryContent />}
            {activePage === "engine" && <EngineManagementContent />}
            {activePage === "downloads" && <DownloadsContent />}
            {activePage === "settings" && <SettingsContent />}
          </div>
        </div>
      </div>
    </main>
  );
}
