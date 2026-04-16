<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { getLocale, locales, localizeHref } from '$lib/paraglide/runtime';
    import { appLink } from '$lib';
    import { type Link } from './Navigation.svelte';

    let {
        links
    }: {
        links: Link[];
    } = $props();
</script>

<footer class="justify-end *:m-2">
    <p class="inline-block">© Utrecht University (ICS).</p>
</footer>

<style>
    @layer components {
        footer {
            display: flex;
            position: relative;
            width: 100%;
        }
    }
</style>
