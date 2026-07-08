import { invoke } from "@tauri-apps/api/core";
import type { Theme } from "./theme";
import type { Tag, TagColor } from "./types/task";
import type { DateFormatName } from "./misc/datePrints";


export const themes = {  
  pinkLight: {
    name: "Pink Light",
    plight: "#ffffff",
    pdark: "#383838",
    hoverpdark: "#202020",
    pcolor: "#f8edf0",
    scolor: "#f8dae0",
    highlightcolor: "#946363",
    bcolor: "#b8b8b8",
    hovercolor: "#ecc7ca",
    accentcolor: "#fc8d9c"
  },

  pinkDark: {
    name: "Pink Dark",
    plight: "#221c1c",
    pdark: "#f9e1e1",
    hoverpdark: "#cda7a7",
    pcolor: "#312529",
    scolor: "#312529",
    highlightcolor: "#e6b7c6",
    bcolor: "#967373",
    hovercolor: "#60484f",
    accentcolor: "#fc8d9c"
  },

  ocean: {
    name: "Ocean",
    plight: "#f8fafc",
    pdark: "#1e293b",
    hoverpdark: "#334155",
    pcolor: "#e0f2fe",
    scolor: "#bae6fd",
    highlightcolor: "#0369a1",
    bcolor: "#94a3b8",
    hovercolor: "#c7e8f7",
    accentcolor: "#0ea5e9"
  },

  forest: {
    name: "Forest",
    plight: "#f7faf7",
    pdark: "#243024",
    hoverpdark: "#344434",
    pcolor: "#e7f5e7",
    scolor: "#cce8cc",
    highlightcolor: "#3f7d3f",
    bcolor: "#9aaa9a",
    hovercolor: "#d8ead8",
    accentcolor: "#65a30d"
  },

  nord: {
    name: "Nord",
    plight: "#2e3440",
    pdark: "#eceff4",
    hoverpdark: "#d8dee9",
    pcolor: "#3b4252",
    scolor: "#434c5e",
    highlightcolor: "#88c0d0",
    bcolor: "#616e88",
    hovercolor: "#4c566a",
    accentcolor: "#81a1c1"
  },

  solarized: {
    name: "Solarized",
    plight: "#fdf6e3",
    pdark: "#073642",
    hoverpdark: "#586e75",
    pcolor: "#eee8d5",
    scolor: "#eee8d5",
    highlightcolor: "#268bd2",
    bcolor: "#93a1a1",
    hovercolor: "#c2b99c",
    accentcolor: "#ffe283"
  },
    kimbieDark: {
    name: "Kimbie Dark",
    plight: "#282522",      // warm dark background
    pdark: "#f5e6c8",       // light foreground/text
    hoverpdark: "#4a4035",  // warm hover
    pcolor: "#362f2a",      // panels/surfaces
    scolor: "#453d34",      // secondary surfaces
    highlightcolor: "#d8a657",      // golden headings
    bcolor: "#7b6b58",      // muted borders
    hovercolor: "#51463c",  // hover surface
    accentcolor: "#f06449"  // warm orange accent
  },
} satisfies Record<string, Theme>;


export type ThemeName = keyof typeof themes;

export const theme = $state({
    name: null as null | ThemeName,
    theme: null as null | Theme
});

export const username = $state({
    name: "Person"
});

interface Flavor {
  name: TagColor;
  bgcolor: string;
  color: string;
  border?: string;
}

export const flavorMap: Record<string, Flavor> = {
  default: {
    name: "default",
    bgcolor: "#e6e6e6",
    color: "black"
  },
  defaultoutline: {
    name: "defaultoutline",
    bgcolor: "#e6e6e6",
    color: "black",
    border: "1px solid black"
  },
  secondary: {
    name: "secondary",
    bgcolor: "var(--highlight-color)",
    color: "black"
  },
  danger: {
    name: "danger",
    bgcolor: "#ffb4b4",
    color: "#86231c"
  },
  outline: {
    name: "outline",
    bgcolor: "transparent",
    color: "var(--primary-dark)",
    border: "1px solid var(--border-color)"
  },
  blue: {
    name: "blue",
    bgcolor: "#749cdc",
    color: "#2f33b0"
  },
  green: {
    name: "green",
    bgcolor: "#86e28c",
    color: "#1a681a"
  },
  greenoutline: {
    name: "greenoutline",
    bgcolor: "#daf7ed",
    color: "#1a681a",
    border: "1px solid #1a681a"
  },
  yellow: {
    name: "yellow",
    bgcolor: "#F7D774",
    color: "#713F12"
  },
};


export function startClock(update: (date: Date) => void) {
    function tick() {
        update(new Date());
    }
    tick();
    const id = setInterval(tick, 1000);
    return () => clearInterval(id);
}

export async function getAllTags() {
  return await invoke<Tag[]>('get_all_tags'); 
}

export function dayKey(d: Date): string {
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, "0");
        const day = String(d.getDate()).padStart(2, "0");
        return `${y}-${m}-${day}`;
    }

export const selectedDateFormat = $state({
    name: "dayOfWeekAndMonth" as DateFormatName
});

export const currentLocation = $state({
  lat: undefined as number | undefined,
  lon: undefined as number | undefined
});