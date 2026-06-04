<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  let container: HTMLDivElement;
  let track: HTMLDivElement;
  let thumb: HTMLDivElement;

  let thumbHeight: number = 0;
  let thumbTop: number = 0;
  let isDragging: boolean = false;
  let dragStartY: number = 0;
  let dragStartScrollTop: number = 0;
  let isVisible: boolean = false;
  let hideTimeout: ReturnType<typeof setTimeout>;

  // --- Calculations ---

  function updateThumb(): void {
    if (!container) return;
    const { scrollTop, scrollHeight, clientHeight } = container;
    const ratio = clientHeight / scrollHeight;

    // Hide scrollbar if content fits
    isVisible = ratio < 1;
    if (!isVisible) return;

    thumbHeight = Math.max(ratio * track.clientHeight, 32);
    const maxThumbTop = track.clientHeight - thumbHeight;
    thumbTop = (scrollTop / (scrollHeight - clientHeight)) * maxThumbTop;
  }

  // --- Scroll event ---

  function onScroll(): void {
    updateThumb();
    showScrollbar();
  }

  function showScrollbar(): void {
    clearTimeout(hideTimeout);
    // Re-trigger CSS visibility (handled via class binding)
    hideTimeout = setTimeout(() => {
      // Fade handled by CSS transition on opacity
    }, 1200);
  }

  // --- Thumb dragging ---

  function onThumbMousedown(e: MouseEvent): void {
    e.preventDefault();
    isDragging = true;
    dragStartY = e.clientY;
    dragStartScrollTop = container.scrollTop;

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }

  function onMouseMove(e: MouseEvent): void {
    if (!isDragging) return;
    const delta = e.clientY - dragStartY;
    const { scrollHeight, clientHeight } = container;
    const maxThumbTop = track.clientHeight - thumbHeight;
    const scrollRatio = delta / maxThumbTop;
    container.scrollTop = dragStartScrollTop + scrollRatio * (scrollHeight - clientHeight);
  }

  function onMouseUp(): void {
    isDragging = false;
    window.removeEventListener('mousemove', onMouseMove);
    window.removeEventListener('mouseup', onMouseUp);
  }

  // --- Track click (jump to position) ---

  function onTrackClick(e: MouseEvent): void {
    if (e.target === thumb) return;
    const trackRect = track.getBoundingClientRect();
    const clickRatio = (e.clientY - trackRect.top - thumbHeight / 2) / (track.clientHeight - thumbHeight);
    const { scrollHeight, clientHeight } = container;
    container.scrollTop = Math.max(0, Math.min(1, clickRatio)) * (scrollHeight - clientHeight);
  }

  // --- ResizeObserver ---

  let resizeObserver: ResizeObserver;

  onMount(() => {
    updateThumb();

    resizeObserver = new ResizeObserver(updateThumb);
    resizeObserver.observe(container);
    // Also watch content size changes
    if (container.firstElementChild) {
      resizeObserver.observe(container.firstElementChild);
    }
  });

  onDestroy(() => {
    resizeObserver?.disconnect();
    clearTimeout(hideTimeout);
    window.removeEventListener('mousemove', onMouseMove);
    window.removeEventListener('mouseup', onMouseUp);
  });
</script>

<div class="scroll-wrapper">
  <!-- Scrollable content area -->
  <div
    class="scroll-content"
    bind:this={container}
    on:scroll={onScroll}
  >
    <slot />
  </div>

  <!-- Custom scrollbar -->
  {#if isVisible}
    <div
      class="scrollbar-track"
      bind:this={track}
      on:mousedown={onTrackClick}
      role="scrollbar"
      aria-controls="scroll-content"
      aria-valuenow={thumbTop}
      tabindex="-1"
    >
      <div
        class="scrollbar-thumb"
        bind:this={thumb}
        class:dragging={isDragging}
        style="height: {thumbHeight}px; transform: translateY({thumbTop}px);"
        on:mousedown={onThumbMousedown}
        role="presentation"
      ></div>
    </div>
  {/if}
</div>

<style>
  .scroll-wrapper {
    position: relative;
    display: flex;
    overflow: hidden;
    height: 100%;
    width: 100%;
  }

  /* Hide the native scrollbar */
  .scroll-content {
    flex: 1;
    overflow-y: scroll;
    overflow-x: hidden;
    scrollbar-width: none; /* Firefox */
  }

  .scroll-content::-webkit-scrollbar {
    display: none; /* WebView / Chrome */
  }

  /* --- Scrollbar track --- */
  .scrollbar-track {
    position: relative;
    width: 6px;
    flex-shrink: 0;
    margin: 4px 2px;
    border-radius: 999px;
    background: transparent;
    cursor: pointer;
    transition: width 0.15s ease, background 0.15s ease;
  }

  .scroll-wrapper:hover .scrollbar-track {
    width: 8px;
    background: var(--primary-color);
  }

  /* --- Scrollbar thumb --- */
  .scrollbar-thumb {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    border-radius: 999px;
    background: var(--hover-color);
    transition:
      background 0.15s ease,
      transform 0.05s linear;
    cursor: grab;
    will-change: transform;
  }

  .scrollbar-thumb:hover {
    background: var(--highlight-color);
  }

  .scrollbar-thumb.dragging {
    background: var(--highlight-color);
    cursor: grabbing;
    transition: background 0.1s ease; /* skip transform transition while dragging */
  }
</style>