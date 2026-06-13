<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { Accordion, AccordionItem, Datepicker } from 'flowbite-svelte';
    import { timeFormatString } from '$lib';
    import {
        type FilterOptions,
        getAllLanguages,
        getAllSubjects,
        type Language,
        type Subject
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import type { Option } from '$lib/api-wrapper/types';
    import { tryParseISBN } from '$lib/treeMetadata';
    import { getUserInfo } from '$lib/api-wrapper/users';
    import { SvelteSet } from 'svelte/reactivity';

    let {
        filterOptions = $bindable(),
        handleFilterUpdate,
        onlyOwnTrees = false
    }: {
        filterOptions: FilterOptions;
        handleFilterUpdate: () => void;
        onlyOwnTrees?: boolean;
    } = $props();

    let userInfo = getUserInfo();

    let localOptions = $state({
        searchQuery: null as Option<string>,
        ownerName: null as Option<string>,
        book: [] as string[]
    });
    let newBook = $state('');

    let errors = $state({
        bookInvalidIsbns: [false],
        newBookInvalid: false
    });

    let languages = $state(null as Language[] | null);

    let subjectsRecord = $state(null as Record<string, Subject[]> | null);
    /**
     * Organize subjects by language, with any duplicates between languages separated.
     *
     * Language groups are ordered by descending number of subjects, after removing duplicates.
     */
    let allSubjects: AllSubjects = $derived.by(() => {
        // Convert into an array.
        let subjectsByLanguage = Object.entries(subjectsRecord!).map(([key, value]) => {
            return {
                type: 'language',
                language: key,
                subjects: value
            } as LanguageSubjects;
        });

        // Find duplicate subject names.
        let foundSubjects: SvelteSet<string> = new SvelteSet();
        let dupSubjectNames: SvelteSet<string> = new SvelteSet();
        for (const langSubjects of subjectsByLanguage) {
            for (const subject of langSubjects.subjects.map((s) => s.data.name)) {
                if (foundSubjects.has(subject)) {
                    dupSubjectNames.add(subject);
                } else {
                    foundSubjects.add(subject);
                }
            }
        }

        // Filter out any found duplicates from the language-separated subject arrays.
        subjectsByLanguage.forEach((ls) => {
            ls.subjects = ls.subjects.filter((s) => !dupSubjectNames.has(s.data.name));
        });

        // Filter out empty language-separated subject arrays and sort by descending number of subjects.
        subjectsByLanguage = subjectsByLanguage
            .filter((ls) => ls.subjects.length > 0)
            .sort((a, b) => b.subjects.length - a.subjects.length);

        return {
            type: 'all',
            duplicateSubjects: dupSubjectNames,
            subjects: subjectsByLanguage
        };
    });
    /**
     * Current options for subjects to select, based on selected language.
     */
    let subjectOptions: Subjects = $derived(
        subjectsRecord === null
            ? { type: 'empty' }
            : filterOptions.languageCode === null
              ? allSubjects
              : {
                    type: 'language',
                    language: filterOptions.languageCode,
                    subjects: subjectsRecord[filterOptions.languageCode]
                }
    );

    type Subjects = EmptySubjects | LanguageSubjects | AllSubjects;

    type EmptySubjects = {
        type: 'empty';
    };

    type LanguageSubjects = {
        type: 'language';
        language: string;
        subjects: Subject[];
    };

    type AllSubjects = {
        type: 'all';
        subjects: LanguageSubjects[];
    };

    type Query = {
        searchTerms: Option<string>;
        tags: string[];
    };

    function handleSearch() {
        const query = localOptions.searchQuery;

        if (query === null || query === '') {
            filterOptions.searchTerms = null;
            filterOptions.tag = [];
        } else {
            const { searchTerms, tags } = extractTags(query);
            filterOptions.searchTerms = searchTerms;
            filterOptions.tag = tags;
        }

        handleFilterUpdate();
    }

    function extractTags(query: string): Query {
        const words = query.trim().split(' ');

        let tags = [];

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

        return {
            searchTerms: searchTerms,
            tags: tags
        };
    }

    function getLanguageName(languageCode: string) {
        if (languages === null) return '';

        let result = languages.filter((l) => l.code === languageCode);

        if (result.length === 0) return '';

        return result[0].name;
    }

    /** Check whether the selected subject is still an option, and if not, reset the subject filter. */
    function checkSubject() {
        // If the selected subject does not exist in the newly selected language, reset its value.
        if (
            subjectOptions.type === 'language' &&
            (subjectOptions.subjects === undefined ||
                subjectOptions.subjects.filter((s) => s.data.name === filterOptions.subject)
                    .length === 0)
        ) {
            filterOptions.subject = null;
        }

        handleFilterUpdate();
    }

    function handleOwnerUpdate() {
        if (localOptions.ownerName === '') localOptions.ownerName = null;
        filterOptions.ownerName = localOptions.ownerName;

        handleFilterUpdate();
    }

    function handleBookUpdate(i: number) {
        errors.bookInvalidIsbns[i] = false;
        let isbn = tryParseISBN(localOptions.book[i]);
        if (isbn === null) {
            errors.bookInvalidIsbns[i] = true;
            return;
        }

        localOptions.book[i] = isbn;
        filterOptions.book[i] = localOptions.book[i];

        if (localOptions.book[i] === '') {
            localOptions.book.splice(i, 1);
            filterOptions.book.splice(i, 1);
            errors.bookInvalidIsbns.splice(i, 1);
        }

        handleFilterUpdate();
    }

    function handleNewBook() {
        errors.newBookInvalid = false;
        let isbn = tryParseISBN(newBook);
        if (isbn === null) {
            errors.newBookInvalid = true;
            return;
        }

        if (newBook !== '') {
            localOptions.book.push(newBook);
            filterOptions.book.push(newBook);
            errors.bookInvalidIsbns.push(false);
            newBook = '';
        }

        handleFilterUpdate();
    }

    onMount(async () => {
        languages = await getAllLanguages();
        subjectsRecord = await getAllSubjects();
    });
</script>

{@render searchBar()}

<Accordion flush multiple>
    <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.language()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render anyLanguageRadioButton()}
        {#each languages as language (language)}
            {@render languageRadioButton(language)}
        {/each}
    </AccordionItem>
    <AccordionItem classes={{ button: 'cursor-pointer page__subheading p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.subject()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render subjectFilters()}
    </AccordionItem>
    <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.creationDate()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render dateFilters()}
    </AccordionItem>
    {#if !onlyOwnTrees}
        <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
            {#snippet header()}
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                    <p class="ml-1 text-left">{m.owner()}</p>
                </AnimatedUnderline>
            {/snippet}
            {@render ownerFilters()}
        </AccordionItem>
    {/if}
    <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.book()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render bookFilters()}
    </AccordionItem>
    {#if !onlyOwnTrees}
        <AccordionItem classes={{ button: 'page__subheading cursor-pointer p-1' }}>
            {#snippet header()}
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                    <p class="ml-1 text-left">{m.searchOptions()}</p>
                </AnimatedUnderline>
            {/snippet}
            {@render optionCheckboxes()}
        </AccordionItem>
    {/if}
</Accordion>

{#snippet searchBar()}
    <div class="mx-1 flex flex-col gap-1">
        <label for="search-bar">{m.search()}</label>
        <input
            id="search-bar"
            class="input__text"
            type="text"
            name="search"
            placeholder={m.examplePlaceholder({ value: m.searchTerms() })}
            bind:value={localOptions.searchQuery}
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

{#snippet languageRadioButton(language: Language)}
    <div class="ml-1 flex items-center gap-2">
        <input
            id="language-${language.code}"
            name="language"
            type="radio"
            value={language.code}
            bind:group={filterOptions.languageCode}
            onchange={checkSubject}
        />
        <label for="language-${language.code}">{language.name}</label>
    </div>
{/snippet}

{#snippet anyLanguageRadioButton()}
    <div class="ml-1 flex items-center gap-2">
        <input
            id="language-any"
            name="language"
            type="radio"
            value={null}
            bind:group={filterOptions.languageCode}
            onchange={checkSubject}
        />
        <label for="language-any">{m.anyLanguage()}</label>
    </div>
{/snippet}

{#snippet subjectFilters()}
    {#if subjectOptions.type === 'empty'}
        <div class="ml-1 flex items-center gap-2">
            <em>{m.loading()}</em>
        </div>
    {:else}
        {@render anySubjectRadioButton()}
    {/if}
    {#if subjectOptions.type === 'language'}
        {@render languageSubjectFilters(subjectOptions)}
    {:else if subjectOptions.type === 'all'}
        {#each subjectOptions.duplicateSubjects as subjectName, i (subjectName)}
            {@render subjectRadioButton({
                id: -i,
                data: {
                    name: subjectName,
                    languageCode: ''
                }
            })}
        {/each}
        <Accordion flush multiple>
            {#each subjectOptions.subjects as languageSubjects (languageSubjects)}
                <AccordionItem classes={{ button: 'cursor-pointer page__subsubheading p-0.5' }}>
                    {#snippet header()}
                        <AnimatedUnderline>
                            <p class="ml-1 text-left">
                                {getLanguageName(languageSubjects.language)}
                            </p>
                        </AnimatedUnderline>
                    {/snippet}
                    {@render languageSubjectFilters(languageSubjects)}
                </AccordionItem>
            {/each}
        </Accordion>
    {/if}
{/snippet}

{#snippet languageSubjectFilters(langSubjects: LanguageSubjects)}
    {#each langSubjects.subjects as subject (subject)}
        {@render subjectRadioButton(subject)}
    {/each}
{/snippet}

{#snippet subjectRadioButton(subject: Subject)}
    <div class="ml-1 flex items-center gap-2">
        <input
            id="subject-${subject.id}"
            name="subject"
            type="radio"
            value={subject.data.name}
            bind:group={filterOptions.subject}
            onchange={handleFilterUpdate}
        />
        <label for="subject-${subject.id}">{subject.data.name}</label>
    </div>
{/snippet}

{#snippet anySubjectRadioButton()}
    <div class="ml-1 flex items-center gap-2">
        <input
            id="subject-any"
            name="subject"
            type="radio"
            value={null}
            bind:group={filterOptions.subject}
            onchange={handleFilterUpdate}
        />
        <label for="subject-any">{m.anySubject()}</label>
    </div>
{/snippet}

{#snippet dateFilters()}
    <div class="mx-1">
        <Datepicker
            id="date-from"
            range
            bind:rangeFrom={
                () => filterOptions.createdAfter ?? undefined,
                (v) => {
                    filterOptions.createdAfter = v ?? null;
                    handleFilterUpdate();
                }
            }
            bind:rangeTo={
                () => filterOptions.createdBefore ?? undefined,
                (v) => {
                    filterOptions.createdBefore = v
                        ? new Date(v.getTime() + 24 * 60 * 60 * 1000 - 1) // Add 23:59:59.999 to make this date inclusive
                        : null;
                    handleFilterUpdate();
                }
            }
            onchange={handleFilterUpdate}
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
            class="input__text w-full"
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
    </div>
{/snippet}

{#snippet bookFilters()}
    <div class="mx-1 flex flex-col gap-1">
        <!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
        {#each localOptions.book as _, i (i)}
            <input
                class={[errors.bookInvalidIsbns[i] && 'error', 'input__text w-full']}
                type="text"
                name="isbn"
                bind:value={localOptions.book[i]}
                placeholder={m.searchByBook()}
                onblur={() => handleBookUpdate(i)}
                onkeydown={(e) => {
                    if (e.key === 'Enter') {
                        e.preventDefault();
                        handleBookUpdate(i);
                    }
                }}
            />
            {#if errors.bookInvalidIsbns[i]}
                <p class="text__error">{m.invalidISBN()}</p>
            {/if}
        {/each}
        <input
            class={[errors.newBookInvalid && 'error', 'input__text w-full']}
            type="text"
            name="isbn"
            bind:value={newBook}
            placeholder={m.searchByBook()}
            onblur={handleNewBook}
            onkeydown={(e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    handleNewBook();
                }
            }}
        />
        {#if errors.newBookInvalid}
            <p class="text__error">{m.invalidISBN()}</p>
        {/if}
    </div>
{/snippet}

{#snippet optionCheckboxes()}
    <div class="mx-1 flex flex-col gap-1">
        <div class="flex gap-2">
            <input
                id="show-own-private"
                class="input__checkbox ml-1"
                type="checkbox"
                bind:checked={filterOptions.showOwnPrivate}
                onchange={handleFilterUpdate}
            />
            <label for="show-own-private" class="text-wrap">{m.showOwnPrivate()}</label>
        </div>
        {#if userInfo?.accountType === 'admin'}
            <div class="flex gap-2">
                <input
                    id="show-all-private"
                    class="input__checkbox ml-1"
                    type="checkbox"
                    bind:checked={filterOptions.showPrivate}
                    onchange={handleFilterUpdate}
                />
                <label for="show-all-private" class="text-wrap">{m.showAllPrivate()}</label>
            </div>
        {/if}
    </div>
{/snippet}
