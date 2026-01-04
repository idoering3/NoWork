import type { Theme } from "./theme";

export let themes: Record<string, Theme> = {
    "Pink Light": {
        name: "Pink Light",
        plight: "#ffffff",
        pdark: "#383838",
        hoverpdark: "#202020",
        pcolor: "#f8edf0",
        scolor: "#f8dae0",
        hcolor: "#946363ff",
        bcolor: "#b8b8b8",
        hovercolor: "#ecc7ca",
        accentcolor: "#fc8d9c"
    },
    "Pink Dark": {
        name: "Pink Dark",
        plight: "#221c1c",      // base dark background
        pdark: "#f9e1e1",       // even darker (panels, cards, etc.)
        hoverpdark: "#cda7a7",  // hover background
        pcolor: "#312529",      // primary text/pink highlight
        scolor: "#312529",      // secondary accent pink
        hcolor: "#e6b7c6",      // heading / brighter pink
        bcolor: "#967373",      // body text (light gray)
        hovercolor: "#60484f",   // hover accent (softer pink)
        accentcolor: "#fc8d9c"
    },
    "Catpuccin'": {
        name: "Catpuccin'",
        plight: "#313244",      // base dark background
        pdark: "#cdd6f4",       // even darker (panels, cards, etc.)
        hoverpdark: "#313244",  // hover background
        pcolor: "#1e1e2e",      // primary text/pink highlight
        scolor: "#1e1e2e",      // secondary accent pink
        hcolor: "#f5c2e7",      // heading / brighter pink
        bcolor: "#f5c2e7",      // border color (light gray)
        hovercolor: "#313244",   // hover accent (softer pink)
        accentcolor: "#7d89b3ff"
    },
};

export const theme = $state({
    theme: null as null | Theme
});

export const username = $state({
    name: "Person"
})

interface Flavor {
  name: string;
  bgcolor: string;
  color: string;
}

export const flavorMap = {
  default: {
    name: "default",
    bgcolor: "#e6e6e6",
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
    color: "var(--primary-dark)"
  },
  blue: {
    name: "blue",
    bgcolor: "#749cdc",
    color: "#363cda"
  },
} as const satisfies Record<string, Flavor>;


export function startClock(update: (date: Date) => void) {
  function tick() {
    const now = new Date();
    update(now);
  }

  tick(); // run immediately
  setInterval(tick, 1000);
}