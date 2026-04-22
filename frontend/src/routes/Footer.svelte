<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { appLink } from '$lib';
    import { type Link } from './Navigation.svelte';
    import logo from '$lib/assets/logo.png';

    let {
        links
    }: {
        links: Link[];
    } = $props();
</script>

{#snippet footerItem(link: Link)}
    <a class="footer-item m-0.5 {link.formatting}" href={appLink(link.href)}>
        {link.text}
    </a>
{/snippet}

<footer class="w-100vw">
    <img class="size-40" src={logo} alt="logo" />
    <div class="flex flex-auto flex-col items-center justify-center">
        <div class="flex flex-col">
            {#each links as link (link)}
                {@render footerItem(link)}
            {/each}
        </div>
    </div>
    <div class="flex flex-col items-end justify-end">
        <p class="m-2 inline-block">© Utrecht University (ICS).</p>
    </div>
</footer>

<style>
    @layer components {
        footer {
            margin-top: 2rem;
            background-color: var(--color-brown);
            display: flex;
        }

        .footer-item:hover {
            text-decoration: underline;
        }
    }
</style>
