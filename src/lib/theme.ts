import { cssVarToRGBArray, type ShaderRenderer } from "./shaders/shader";

export interface Theme {
    name: string;
    plight: string;
    pdark: string;
    hoverpdark: string;
    pcolor: string;
    scolor: string;
    hcolor: string;
    bcolor: string;
    hovercolor: string;
    accentcolor: string;
}

export function setColors(root: HTMLElement, theme: Theme, renderer: ShaderRenderer) {
    root.style.setProperty("--primary-light", theme.plight);
    root.style.setProperty("--primary-dark", theme.pdark);
    root.style.setProperty("--hover-primary-dark", theme.hoverpdark);
    root.style.setProperty("--primary-color", theme.pcolor);
    root.style.setProperty("--secondary-color", theme.scolor);
    root.style.setProperty("--highlight-color", theme.hcolor);
    root.style.setProperty("--border-color", theme.bcolor);
    root.style.setProperty("--hover-color", theme.hovercolor);
    root.style.setProperty("--accent-color", theme.accentcolor);

    const inner = hexToRGBArray(theme.pcolor);
    const outer = hexToRGBArray(theme.accentcolor);
    renderer?.updateColors(inner, outer);
}


function hexToRGBArray(hex: string): [number, number, number] {
    if (hex.startsWith("#")) hex = hex.slice(1);
    const r = parseInt(hex.slice(0,2),16)/255;
    const g = parseInt(hex.slice(2,4),16)/255;
    const b = parseInt(hex.slice(4,6),16)/255;
    return [r,g,b];
}