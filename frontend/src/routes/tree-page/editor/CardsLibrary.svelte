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
    import { type SideBar } from './Sidebar.svelte';
    import CardViewer from './CardViewer.svelte';
    import { Accordion, AccordionItem, Datepicker } from 'flowbite-svelte';
    import { timeFormatString } from '$lib';
    import { m } from '$lib/paraglide/messages';
    import { onMount } from 'svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import { type Option } from '$lib/api-wrapper/types';

    let { sidebar = $bindable() }: { sidebar: SideBar | null } = $props();

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

    async function updateLibrary() {
        libraryCards = await getFilteredList(cardFilterOptions);
    }

    function insertCard(libCard: LibraryCardGetData) {
        if (sidebar === null) {
            return;
        }
        sidebar.editingVersion = {
            front: libCard.front,
            back: libCard.back
        };
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

        let tags = [];

        // Find any words starting with a hashtag, add them to the tag list, and remove them from the search terms.
        for (let i = 0; i < words.length; i++) {
            if (words[i][0] === '#') {
                tags.push(words[i].slice(1));
                words.splice(i, 1);
                i--;
            }
        }

        let searchTerms: Option<string> = words.join(' ');

        if (searchTerms === '') {
            searchTerms = null;
        }

        return tags;
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

<div class="mt-1 mb-1 h-full overflow-x-hidden overflow-y-auto">
    <table class="border-separate">
        <tbody>
            {#each libraryCards as libraryCard (libraryCard.cardId)}
                <tr>
                    <td
                        ><b>{libraryCard.title}</b>
                        <div class="float-right">
                            {new Date(libraryCard.createdAt.toString()).toLocaleString()}
                        </div></td
                    >
                </tr>
                <tr>
                    <td>Tags: {libraryCard.tags.join(', ')}</td>
                </tr>
                <tr class="cursor-pointer" onclick={() => insertCard(libraryCard)}>
                    <CardViewer card={{ front: libraryCard.front, back: libraryCard.back }} />
                </tr>
            {/each}
        </tbody>
    </table>
</div>

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
