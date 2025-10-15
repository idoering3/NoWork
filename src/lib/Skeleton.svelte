<script lang='ts'>
</script>

<div class="skeleton-loader">
    <div>
        <div class="skeleton-title"></div>
        <div class="skeleton-text"></div>
        <div class="skeleton-text"></div>
    </div>
    <div>
        <div class="skeleton-text"></div>
        <div class="skeleton-text"></div>
        <div class="skeleton-text"></div>
    </div>
</div>

<style>
.skeleton-loader {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 2rem;
    background-color: transparent;
    border-radius: 15px;
    width: 100%;
    height: 100%;
    /* Add other styles to mimic your content's container */
}

/* Base styles for all skeleton elements */
.skeleton-title,
.skeleton-text {
  border-radius: 15px;
  margin-bottom: 0.75rem;
}

.skeleton-title {
  height: 3rem;
  width: 60%;
}

.skeleton-text {
  height: 1rem;
  width: 100%;
}

/* Animation for the "shimmer" effect */
.skeleton-title,
.skeleton-text {
  /* Add animation to the main container */
  animation: shimmer 1.5s infinite ease;
  background: linear-gradient(to right, transparent 0%, var(--highlight-color) 50%, transparent 100%);
  background-size: 200% 100%;
}

@keyframes shimmer {
  0% {
    background-position: -100% 0;
  }
  100% {
    background-position: 100% 0;
  }
}
</style>