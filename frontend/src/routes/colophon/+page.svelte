<!-- This program has been developed by students from the bachelor Computer Science at Utrecht
University within the Software Project course. © Copyright Utrecht University (Department of
Information and Computing Sciences) -->

<script lang="ts">
    import { getLocale } from '$lib/paraglide/runtime';
    import Nl from './Nl.svelte';
    import En from './En.svelte';

    const locale = getLocale();
</script>

<div class="flex w-full justify-center *:w-md">
    {#if locale === 'nl'}
        <Nl />
    {:else if locale === 'en'}
        <En />
    {/if}
</div>
