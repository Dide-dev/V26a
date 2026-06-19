<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import {
        type CardFilterOptions,
        getFilteredList,
        type LibraryCardGetData
    } from '$lib/api-wrapper/cards_library';
    import { type CardSideBar, type MenuSideBar } from './Sidebar.svelte';
    import CardViewer from '../CardViewer.svelte';
    import { Accordion, AccordionItem, Datepicker } from 'flowbite-svelte';
    import { timeFormatString } from '$lib';
    import { m } from '$lib/paraglide/messages';
    import { onMount } from 'svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import { type Option } from '$lib/api-wrapper/types';
    import { branchColors } from '$lib/treePage';

    let { sidebar = $bindable() }: { sidebar: CardSideBar | MenuSideBar } = $props();

    let cardFilterOptions: CardFilterOptions = $state({
        ownerName: null,
        createdAfter: null,
        createdBefore: null,
        tags: []
    });

    let localOptions: { tagsQuery: string | null; ownerName: string | null } = $state({
        tagsQuery: null,
        ownerName: null
    });

    let libraryCards: LibraryCardGetData[] = $state([]);

    let malformedUsername: boolean = $state(false);

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);

    async function updateLibrary() {
        libraryCards = await getFilteredList(cardFilterOptions);
    }

    function insertCard(libCard: LibraryCardGetData) {
        if (sidebar.kind === 'Menu') return;

        sidebar.editingVersion = {
            front: libCard.front,
            back: libCard.back
        };
        sidebar.insertingFromLibrary = false;
    }

    function handleSearch() {
        const query = localOptions.tagsQuery;

        if (query === null || query === '') {
            cardFilterOptions.tags = [];
        } else {
            cardFilterOptions.tags = extractTags(query);
        }

        updateLibrary();
    }
    /**
     * Extract any tags from a search query, and return both the sanitized search terms and the tags separately.
     *
     * Tags are found as words starting with a '#'.
     */
    function extractTags(query: string): string[] {
        const words = query.trim().split(' ');

        let tags = new Map();

        // Find any words starting with a hashtag, add them to the tag list, and remove them from the search terms.
        for (let i = 0; i < words.length; i++) {
            if (words[i][0] === '#') {
                tags.set(words[i].slice(1).toLowerCase(), words[i].slice(1));
                words.splice(i, 1);
                i--;
            }
        }

        let searchTerms: Option<string> = words.join(' ');

        if (searchTerms === '') {
            searchTerms = null;
        }

        return tags.values().toArray();
    }

    function handleOwnerUpdate() {
        const pattern = '[a-zA-Z0-9._\\-]*';
        malformedUsername = false;

        if (localOptions.ownerName === null) {
            cardFilterOptions.ownerName = localOptions.ownerName = null;
        } else if (localOptions.ownerName?.match(pattern)![0] !== localOptions.ownerName) {
            malformedUsername = true;
            return;
        }

        cardFilterOptions.ownerName = localOptions.ownerName;

        updateLibrary();
    }

    onMount(async () => {
        await updateLibrary();
    });
</script>

<h2 class="page__heading">{m.cardLibrary()}</h2>

<Accordion flush multiple>
    <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.tags()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render searchBar()}
    </AccordionItem>
    <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.creationDate()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render dateFilters()}
    </AccordionItem>
    <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.owner()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render ownerFilters()}
    </AccordionItem>
</Accordion>

<div class="mt-1 mb-1 flex h-fit max-h-screen flex-col overflow-x-hidden overflow-y-auto">
    {#each libraryCards as libraryCard (libraryCard.cardId)}
        {@render cardEntry(libraryCard)}
    {/each}
</div>

{#snippet cardEntry(libraryCard: LibraryCardGetData)}
    <div class="flex w-full flex-row justify-between">
        <div class="flex flex-col gap-0.5">
            <strong>{libraryCard.title}</strong>
            <em class="-mt-1">{m.madeBy()}: {libraryCard.ownerName}</em>
        </div>
        <div class="text-right">
            <p class="font-semibold">
                {timeFormat.format(new Date(libraryCard.createdAt))}
            </p>
            <p class="text-neutral-700">{libraryCard.tags.map((t) => `#${t}`).join(' ')}</p>
        </div>
    </div>
    <button class="cursor-pointer" onclick={() => insertCard(libraryCard)} type="button">
        <CardViewer
            card={{ front: libraryCard.front, back: libraryCard.back }}
            color={sidebar.kind !== 'CardEditor'
                ? 'tree'
                : branchColors[sidebar.location[0] % branchColors.length]}
            isBeingEdited={false}
        />
    </button>
{/snippet}

{#snippet searchBar()}
    <div class="mx-1 flex flex-col gap-1">
        <label for="search-bar">{m.search()}</label>
        <input
            id="search-bar"
            class="input__text"
            type="text"
            name="search"
            placeholder={m.examplePlaceholder({ value: m.searchTags() })}
            bind:value={localOptions.tagsQuery}
            onblur={handleSearch}
            onkeydown={(e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    handleSearch();
                }
            }}
        />
    </div>
{/snippet}

{#snippet dateFilters()}
    <div class="mx-1">
        <Datepicker
            id="date-from"
            range
            bind:rangeFrom={
                () => cardFilterOptions.createdAfter ?? undefined,
                (v) => {
                    cardFilterOptions.createdAfter = v ?? null;
                    updateLibrary();
                }
            }
            bind:rangeTo={
                () => cardFilterOptions.createdBefore ?? undefined,
                (v) => {
                    cardFilterOptions.createdBefore = v
                        ? new Date(v.getTime() + 24 * 60 * 60 * 1000 - 1) // Add 23:59:59.999 to make this date inclusive
                        : null;
                    updateLibrary();
                }
            }
            onchange={updateLibrary}
            locale={timeFormatString}
            translationLocale={timeFormatString}
            placeholder={m.selectDate()}
            classes={{ dayButton: 'text-neutral-700 hover:text-black' }}
        />
    </div>
{/snippet}

{#snippet ownerFilters()}
    <div class="mx-1">
        <input
            class={[malformedUsername && 'error', 'input__text w-full']}
            type="text"
            name="username"
            bind:value={localOptions.ownerName}
            placeholder={m.searchByOwner()}
            onblur={handleOwnerUpdate}
            onkeydown={(e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    handleOwnerUpdate();
                }
            }}
        />
        {#if malformedUsername}
            <p class="text__error">{m.allowedRegex({ field: m.names() })}</p>
        {/if}
    </div>
{/snippet}
