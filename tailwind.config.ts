import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        mono: ["JetBrains Mono", "monospace"],
        sans: ["Geist", "system-ui", "sans-serif"],
      },
      colors: {
        launcher: {
          bg: "#08090B",
          glass: "#111318B8",
          panel: "#12161ECC",
          panelAlt: "#10141BCC",
          accent: "#4A9FD8",
          accentDim: "#4A9FD826",
          success: "#4DE0A7",
          successBg: "#103D30",
          warning: "#FFB15C",
          text: "#FFFFFF",
          textSecondary: "#AEB6C2",
          textMuted: "#6F7A86",
          textDark: "#8D96A4",
          border: "#FFFFFF24",
          borderLight: "#FFFFFF1F",
          hover: "#FFFFFF12",
          hoverLight: "#FFFFFF1F",
          active: "#FFFFFF1F",
        },
      },
      borderRadius: {
        "2xl": "22px",
        "3xl": "28px",
      },
    },
  },
  plugins: [],
};
export default config;
