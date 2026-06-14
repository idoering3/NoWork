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

  function updateThumb(): void {
    if (!container || !track) return;
    const { scrollTop, scrollHeight, clientHeight } = container;
    const ratio = clientHeight / scrollHeight;

    isVisible = ratio < 1;
    if (!isVisible) return;

    thumbHeight = Math.max(ratio * track.clientHeight, 32);
    const maxThumbTop = track.clientHeight - thumbHeight;
    thumbTop = (scrollTop / (scrollHeight - clientHeight)) * maxThumbTop;
  }

  function onScroll(): void {
    updateThumb();
    showScrollbar();
  }

  function showScrollbar(): void {
    clearTimeout(hideTimeout);
    hideTimeout = setTimeout(() => {}, 1200);
  }

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

  function onTrackClick(e: MouseEvent): void {
    if (e.target === thumb) return;
    const trackRect = track.getBoundingClientRect();
    const clickRatio = (e.clientY - trackRect.top - thumbHeight / 2) / (track.clientHeight - thumbHeight);
    const { scrollHeight, clientHeight } = container;
    container.scrollTop = Math.max(0, Math.min(1, clickRatio)) * (scrollHeight - clientHeight);
  }

  let resizeObserver: ResizeObserver;

  onMount(() => {
    updateThumb();

    resizeObserver = new ResizeObserver(updateThumb);
    resizeObserver.observe(container);
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
  <div
    class="scroll-content"
    bind:this={container}
    on:scroll={onScroll}
  >
    <slot />
  </div>

  <!-- Always rendered so `track` binding is always available -->
  <div
    class="scrollbar-track"
    class:hidden={!isVisible}
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
</div>

<style>
  .scroll-wrapper {
    position: relative;
    display: flex;
    overflow: hidden;
    height: 100%;
    width: 100%;
  }

  .scroll-content {
    flex: 1;
    overflow-y: scroll;
    overflow-x: hidden;
    scrollbar-width: none;
  }

  .scroll-content::-webkit-scrollbar {
    display: none;
  }

  .scrollbar-track {
    position: relative;
    width: 6px;
    flex-shrink: 0;
    margin: 4px 2px;
    border-radius: 999px;
    background: transparent;
    cursor: pointer;
    transition: width 0.15s ease, background 0.15s ease, opacity 0.15s ease;
  }

  .scrollbar-track.hidden {
    opacity: 0;
    pointer-events: none;
  }

  .scroll-wrapper:hover .scrollbar-track:not(.hidden) {
    width: 8px;
    background: var(--primary-color);
  }

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
    transition: background 0.1s ease;
  }
</style>