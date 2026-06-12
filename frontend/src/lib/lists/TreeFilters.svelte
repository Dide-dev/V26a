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
        getAllLanguages,
        getAllSubjects,
        type Language,
        type Subject
    } from '$lib/api-wrapper/trees';
    import { getLocale } from '$lib/paraglide/runtime';
    import { onMount } from 'svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';

    let { filterOptions = $bindable() } = $props();

    let languages = $state(null as Language[] | null);
    let selectedLanguage: string = $state(getLocale());

    let allSubjects = $state(null as Record<string, Subject[]> | null);
    let subjects = $derived(allSubjects !== null ? allSubjects[selectedLanguage] : []);

    onMount(async () => {
        languages = await getAllLanguages();
        allSubjects = await getAllSubjects();
    });
</script>

<Accordion flush>
    <AccordionItem headerClass="cursor-pointer page__subheading p-1">
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="text-left">{m.language()}</p>
            </AnimatedUnderline>
        {/snippet}
        {#each languages as language (language)}
            <div class="flex items-center gap-2">
                <input
                    id="language-${language.code}"
                    name="language"
                    type="radio"
                    value={language.code}
                    bind:group={selectedLanguage}
                />
                <label for="language-${language.code}">{language.name}</label>
            </div>
        {/each}
    </AccordionItem>
    <AccordionItem headerClass="cursor-pointer page__subheading p-1">
        {#snippet header()}
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                <p class="text-left">{m.subject()}</p>
            </AnimatedUnderline>
        {/snippet}
        {#each subjects as subject (subject)}
            <div class="flex items-center gap-2">
                <input id="subject-${subject.id}" name="subject" type="radio" value={subject.id} />
                <label for="subject-${subject.id}">{subject.name}</label>
            </div>
        {/each}
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
    <input class="rounded-md border-2" type="number" bind:value={filterOptions.ownerId} />
</label>
