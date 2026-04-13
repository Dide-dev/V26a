<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
@component
General purpose popup, will display over all other elements.
Only layout is provided so bring your own cancel/ok buttons.
-->

<script lang="ts">
    import type { Snippet } from 'svelte';

    let { children }: { children: Snippet } = $props();
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="rounded bg-white p-4">
        {@render children()}
    </div>
</div>
