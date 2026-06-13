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
    import { AccordionItem, Accordion, Datepicker } from 'flowbite-svelte';
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

    let {
        filterOptions = $bindable()
    }: {
        filterOptions: FilterOptions;
    } = $props();

    let languages = $state(null as Language[] | null);

    let allSubjects = $state(null as Record<string, Subject[]> | null);
    /**
     * Subjects, organized in an array that can be easily traversed.
     *
     * Sorted by descending number of subjects.
     */
    let allSubjectsArray = $derived(
        Object.entries(allSubjects!)
            .map(([key, value]) => {
                return {
                    type: 'language',
                    language: key,
                    subjects: value
                } as LanguageSubjects;
            })
            .sort((a, b) => b.subjects.length - a.subjects.length)
    );
    let subjectOptions: Subjects = $derived(
        allSubjects === null
            ? { type: 'empty' }
            : filterOptions.languageCode === null
              ? {
                    type: 'all',
                    subjects: allSubjectsArray
                }
              : {
                    type: 'language',
                    language: filterOptions.languageCode,
                    subjects: allSubjects[filterOptions.languageCode]
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

    function getLanguageName(languageCode: string) {
        if (languages === null) return '';

        let result = languages.filter((l) => l.code === languageCode);

        if (result.length === 0) return '';

        return result[0].name;
    }

    /** Check whether the selected subject is still an option, and if not, reset the subject filter. */
    function checkSubject() {
        // If all subjects are available, no issues will arise.
        if (subjectOptions.type === 'all') return;

        // If the subject is included in the selected language, no issues will arise.
        if (
            subjectOptions.type === 'language' &&
            subjectOptions.subjects !== undefined &&
            subjectOptions.subjects.filter((s) => s.data.name === filterOptions.subject).length > 0
        )
            return;

        // Otherwise, reset the subject filter.
        filterOptions.subject = null;
    }

    onMount(async () => {
        languages = await getAllLanguages();
        allSubjects = await getAllSubjects();
    });

    $effect(() => {
        const _ = filterOptions;


    })
</script>

<Accordion flush multiple>
    <AccordionItem open classes={{ button: 'page__subheading cursor-pointer p-1' }}>
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
    <AccordionItem open classes={{ button: 'cursor-pointer page__subheading p-1' }}>
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="ml-1 text-left">{m.subject()}</p>
            </AnimatedUnderline>
        {/snippet}
        {@render subjectFilters()}
    </AccordionItem>
</Accordion>

<b>{m.creationDate()}</b>
<div class="flex">
    <label class="m-1 flex-1/2">
        {m.from()}
        <Datepicker
            bind:value={
                () => filterOptions.createdAfter ?? undefined,
                (v) => (filterOptions.createdAfter = v ?? null)
            }
            locale={timeFormatString}
            translationLocale={timeFormatString}
            placeholder={m.selectDate()}
        />
    </label>
    <label class="m-1 flex-1/2">
        {m.through()}
        <Datepicker
            bind:value={
                () => filterOptions.createdBefore ?? undefined,
                (v) =>
                    (filterOptions.createdBefore = v
                        ? new Date(v.getTime() + 24 * 60 * 60 * 1000 - 1) // Add 23:59:59.999 to make this date inclusive
                        : null)
            }
            locale={timeFormatString}
            translationLocale={timeFormatString}
            placeholder={m.selectDate()}
        />
    </label>
</div>
<label class="m-1">
    {m.ownerId()}
    <input class="rounded-md border-2" type="number" bind:value={filterOptions.ownerName} />
</label>

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
        <Accordion flush multiple>
            {#each subjectOptions.subjects as languageSubjects (languageSubjects)}
                <AccordionItem classes={{ button: 'cursor-pointer page__subsubheading p-0.5' }}>
                    {#snippet header()}
                        <AnimatedUnderline>
                            <p class="text-left">{getLanguageName(languageSubjects.language)}</p>
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
        />
        <label for="subject-any">{m.anySubject()}</label>
    </div>
{/snippet}
