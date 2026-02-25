<script lang="ts">
  let { children, ...rest } = $props();
</script>

<button {...rest}>
  {@render children()}
</button>

<style>
  button { color: blue; }
  .sm { font-size: 1em; padding: 0.1em; }
  .lg { font-size: 2em; }
</style>