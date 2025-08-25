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
}

export function setColors(root: HTMLElement, theme: Theme) {
    root.style.setProperty("--primary-light", theme.plight);
    root.style.setProperty("--primary-dark", theme.pdark);
    root.style.setProperty("--hover-primary-dark", theme.hoverpdark);
    root.style.setProperty("--primary-color", theme.pcolor);
    root.style.setProperty("--secondary-color", theme.scolor);
    root.style.setProperty("--highlight-color", theme.hcolor);
    root.style.setProperty("--border-color", theme.bcolor);
    root.style.setProperty("--hover-color", theme.hovercolor);
}