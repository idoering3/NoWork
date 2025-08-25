import type { Theme } from "./theme";

export let themes: Record<string, Theme> = {
    "Pink Light": {
        name: "Pink Light",
        plight: "#ffffff",
        pdark: "#383838",
        hoverpdark: "#202020",
        pcolor: "#f8edf0",
        scolor: "#f8dae0",
        hcolor: "#8d6577",
        bcolor: "#b8b8b8",
        hovercolor: "#ecc7ca"
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
        hovercolor: "#60484f"   // hover accent (softer pink)
    },
};

export const theme = $state({
    theme: null as null | Theme
});