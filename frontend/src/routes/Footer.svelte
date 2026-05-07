<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { appLink } from '$lib';
    import logo from '$lib/assets/logo.png';
    import type { Tween } from 'svelte/motion';

    export type FooterLink = {
        href: string;
        text: string;
        target: string;
        localized: boolean;
    };

    let {
        links,
        offset
    }: {
        links: FooterLink[];
        offset: Tween<number>;
    } = $props();
</script>

<footer style="transform: translateY({offset.current}px);" class="w-100vw">
{#snippet footerItem(link: FooterLink)}
    {#if link.localized}
        <a class="footer-item m-0.5" href={appLink(link.href)} target={link.target}>
            {link.text}
        </a>
    {:else}
        <a class="footer-item m-0.5" rel="external" href={link.href} target={link.target}>
            {link.text}
        </a>
    {/if}
{/snippet}

<footer class="w-100vw">
    <a href={appLink('/')} class="flex items-center">
        <img class="aspect-square w-35 not-md:w-20" src={logo} alt="logo" />
    </a>
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
