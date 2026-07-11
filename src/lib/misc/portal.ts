// intended to fix the issues with dropdowns being behind other elements during svelte transitions/css animations

export function portal(node: Node, target: () => HTMLElement | undefined) {
    const target1 = target();
    if (target1) {
        target1.appendChild(node);
        return {
            destroy() {
                if (node.parentNode) node.parentNode.removeChild(node);
            }
        };
    }

}