// This file is responsible for setting and getting the user context for the parent +page.svelte div.
// Used primarily for the dropdown scrolling.

import { createContext } from 'svelte';

type PageElement = () => HTMLElement | undefined;

export const [getPageEl, setPageEl] = createContext<PageElement>();