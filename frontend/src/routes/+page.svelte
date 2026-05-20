<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { getLocale } from '$lib/paraglide/runtime';
    import { m } from '$lib/paraglide/messages.js';
    import { AccordionItem, Accordion } from 'flowbite-svelte';
    import { onMount } from 'svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import helpScheme from '$lib/assets/helpschema.jpg';
    import { appLink } from '$lib';

    // The contents of a card on the home-page.
    type CardContent = {
        title: string;
        color: string;
        plainContents: PlainContent[];
        popOpenContents: PopOpenContent[];
    };

    type PlainContent = string;

    // The contents of a pop-open item.
    type PopOpenContent = {
        heading: string;
        content: PlainContent[];
    };

    // The contents of cards to display, for each locale.
    let cards = {
        nl: [
            {
                title: 'Verbeter je oplosvaardigheden met heuristiekbomen',
                color: 'var(--color-tree-100)',
                plainContents: [
                    'Wanneer je bezig bent met een opgave, loop je op een gegeven moment vast. ' +
                        'Dan kun je ervoor kiezen om het antwoordenmodel erbij te pakken, maar ' +
                        'daarvan wordt je niet meteen beter in het oplossen van problemen.',
                    'In plaats daarvan, kun je ook gebruik maken van een heuristiekboom: ' +
                        'een set aan hintkaartjes waar je in een boomstructuur doorheen kunt lopen.',
                    `Je kunt <a href=${appLink('/all-trees')} class="text__link">hier</a> talloze heurstiekbomen vinden, ` +
                        'waarmee je meteen aan de slag kunt!'
                ],
                popOpenContents: [
                    {
                        heading: 'Wat is een heuristiek?',
                        content: [
                            'Platgeslagen is een heuristiek een stuk gereedschap. ' +
                                "Zo'n stuk gereedschap kan zo generiek zijn als een hamer of zaag, " +
                                'of zo specifiek als een kroonsteentang voor horlogemakers.',
                            'Heuristieken kunnen in diverse velden dienen als vuistregels ' +
                                'voor het oplossen van allerlei problemen.'
                        ]
                    },
                    {
                        heading: 'Wat is een heuristiekboom?',
                        content: [
                            'Een heuristiekboom biedt hints die gestructureerd zijn in de vorm van een boom. ' +
                                'Heuristiekbomen zijn handgemaakt voor één opdracht, ' +
                                'en bieden zo meer sturing dan een algemene lijst van heuristieken.'
                        ]
                    },
                    {
                        heading: 'Hoe gebruik ik een heuristiekboom?',
                        content: [
                            'Bij het openen van de heuristiekboom zijn alle hintkaartjes gesloten. ' +
                                'Deze kunnen van links naar rechts worden geopend, ' +
                                'door op de kaartjes te klikken. ' +
                                'De heuristieken beginnen algemeen en worden steeds concreter, ' +
                                'des te dieper je een boom induikt.',
                            'De meeste bomen zijn opgedeeld in drie hoofdtaken. ' +
                                'Iedere hoofdtak hoort bij een fase: ' +
                                '<ul>' +
                                '<li><b>Oriëntatie:</b> ' +
                                'hier ontdek je het probleem en probeer je er grip op te krijgen.' +
                                '</li>' +
                                '<li><b>Maken en uitvoeren van plannen:</b> ' +
                                'hier bedenk je een strategie en probeer je die uit te voeren, ' +
                                'totdat je het probleem hebt opgelost.' +
                                '</li>' +
                                '<li><b>Afronding:</b> ' +
                                'hier blik je terug op de oplossing en je proces daarnaartoe, ' +
                                'en probeer je leerpunten voor jezelf te vinden.' +
                                '</li>' +
                                '</ul>',
                            'Tijdens het doorlopen van de boom kun je het volgende hulpschema gebruiken:',
                            `<a href=${helpScheme} target="_blank">
                                    <img src=${helpScheme} alt="Hulpschema voor bij het doorlopen van een heuristiekboom.">
                                </a>`
                        ]
                    }
                ]
            },
            {
                title: 'Creëer nieuwe heuristiekbomen voor je leerlingen',
                color: 'var(--color-sky-100)',
                plainContents: [
                    `Door een account te <a href=${appLink('/registration')} class="text__link">registreren</a>, ` +
                        'kun je zelf nieuwe heuristiekbomen creëren voor die lastige opgaves, ' +
                        'waar je leerlingen moeite mee hebben.',
                    'Met onze <i>tree-editor</i> maak je gemakkelijk een heuristiekboom, ' +
                        'en publiceer je deze gratis, zodat al jouw leerlingen, ' +
                        'en hopelijk nog meer, daar gebruik van kunnen maken.'
                ],
                popOpenContents: [
                    {
                        heading: 'Hoe maak ik een heuristiekboom?',
                        content: [
                            'Om te beginnen met het maken van heuristiekbomen, ' +
                                'moet je eerst een account aanmaken op de ' +
                                `<a href=${appLink('/registration')} class="text__link">registratiepagina</a>.`,
                            'Als je eenmaal bent ingelogd, kun je je begeven naar de ' +
                                `<a href=${appLink('/tree-generator')} class="text__link">
                                        boom-creatie pagina</a>, ` +
                                'waarvandaan je, na het invullen van wat informatie over de boom, ' +
                                'wordt doorgeleid naar de <i>tree-editor</i>.'
                        ]
                    },
                    {
                        heading: 'Hoe bewerk ik mijn heuristiekboom?',
                        content: [
                            'Vanuit de <i>tree-editor</i> kun je gemakkelijk kaartjes ' +
                                'toevoegen, verwijderen en verslepen: ' +
                                '<ul>' +
                                '<li>Door op een van de gestippelde kaders met een plusje ' +
                                'te klikken voeg je een nieuwe kaart toe.</li>' +
                                '<li>In het zijmenu dat opent bij het toevoegen van een kaart, ' +
                                'kan de voor- en achtertekst worden bewerkt. ' +
                                'De voortekst wordt getoond voordat de kaart is geopend, ' +
                                'en de achterkant, die dus de heuristiek bevat, daarna.</li>' +
                                '<li>Dit zijmenu om de inhoud van een kaart te wijzigen ' +
                                'kan later weer worden geopend door te klikken op het potloodje ' +
                                'linksboven een kaart.</li>' +
                                '<li>Een kaart kan worden verwijderd door te klikken op het vuilnisbakje ' +
                                'linksbovenin een kaart.</li>' +
                                '<li>Een kaart kan samen met al haar <i>kinderen</i> worden versleept' +
                                'door de pijltjes linksboven in een kaart vast te grijpen. ' +
                                'De kaart(en) worden geplaatst op het gestippelde kader ' +
                                'waarboven ze worden losgelaten.</li>' +
                                '<li>Je kunt je laatste actie ongedaan maken door te klikken ' +
                                'op de pijltjes bovenaan de pagina, of door gebruik te maken van ' +
                                'de sneltoetsen <i>Ctrl + Z</i> en <i>Ctrl + Y</i>.</li>' +
                                '</ul>',
                            `Door op '${m.editMetadata()}' te klikken bovenaan de pagina, ` +
                                'kun je ook nog de titel, beschrijving en andere informatie ' +
                                'van de boom aanpassen.'
                        ]
                    },
                    {
                        heading: 'Hoe kan ik mijn kaarten vormgeven?',
                        content: [
                            'De kaarten kunnen worden vormgegeven met behulp van ' +
                                '<a href="https://www.markdownguide.org/cheat-sheet/" target="_blank" class="text__link">' +
                                'Markdown</a>- en ' +
                                '<a href="https://katex.org/docs/supported" target="_blank" class="text__link">' +
                                'KaTeX</a>-notatie.',
                            'Met Markdown wordt <span class="text__code">**Belangrijke tekst**</span> ' +
                                'bijvoorbeeld getoond als <b>Belangrijke tekst</b>, en ' +
                                'met KaTeX kunnen allerlei wiskundige formules worden weergegeven, ' +
                                'bijvoorbeeld een breuk met ' +
                                '<span class="text__code">$\\frac{1}{2}$</span>.',
                            'Bekijk de hierboven gelinkte documentatie voor alle mogelijkheden. ' +
                                'Niet alles wat in de Markdown-documentatie onder ' +
                                "'Extended Syntax' staat wordt ondersteund."
                        ]
                    },
                    {
                        heading: 'Hoe publiceer ik mijn heuristiekboom?',
                        content: [
                            'Het publiceren van de boom kan vanaf de <i>tree-editor</i>-pagina. ' +
                                `Klik daar op '${m.editMetadata()}' en vink het veld ` +
                                `'${m.published()}' aan, om de boom te publiceren.`,
                            'Om te controleren of een boom gepubliceerd is, kun je op de ' +
                                `<a href=${appLink('/my-trees')} class="text__link">` +
                                'mijn bomen</a>-pagina kijken, onder de kolom ' +
                                `'${m.published()}'.`
                        ]
                    },
                    {
                        heading: 'Welk onderzoek ligt er achter het gebruik van heuristiekbomen?',
                        content: [
                            'De afgelopen jaren zijn er aan de Universiteit Utrecht ' +
                                'verschillende onderzoeken geweest naar het gebruik van ' +
                                'heuristiekbomen bij het onderwijzen van wiskunde. ' +
                                'Lees bijvoorbeeld ' +
                                '<a href="https://edspace.nl/htree/Vastgelopen_Je_kunt_de_boom_in.pdf" target="_blank" class="text__link">' +
                                'deze publicatie</a> in Euclides van Theo van den Bogaart en Rogier Bos.',
                            'Andere onderzoeken:' +
                                '<ul>' +
                                '<li>Bos, R., & van den Bogaart, T. (2022). Heuristic Trees as ' +
                                'a Digital Tool to Foster Compression and Decompression in ' +
                                'Problem‑Solving. ' +
                                '<i>Digital Experiences in Mathematics Education, 8</i>, 157–182. ' +
                                '<a href="https://doi.org/10.1007/s40751-022-00101-6" target="_blank" class="text__link">' +
                                'https://doi.org/10.1007/s40751-022-00101-6</a></li>' +
                                '<li>Lemmink, R. (2019): Improving Help-Seeking Behavior for ' +
                                'Online Mathematical Problem-Solving Lessons, ' +
                                '<a href="https://studenttheses.uu.nl/handle/20.500.12932/33363" target="_blank" class="text__link">' +
                                'https://studenttheses.uu.nl/handle/20.500.12932/33363</a></li>' +
                                '<li>Stokkermans, J. (2024), Supporting reasoning skills with ' +
                                'heuristic trees in pre-university mathematics, ' +
                                '<a href="https://studenttheses.uu.nl/handle/20.500.12932/48210" target="_blank" class="text__link">' +
                                'https://studenttheses.uu.nl/handle/20.500.12932/48210</a></li>' +
                                '<li>Zwennes, E. (2024), The fruits of heuristic trees, ' +
                                '<a href="https://studenttheses.uu.nl/handle/20.500.12932/47001" target="_blank" class="text__link">' +
                                'https://studenttheses.uu.nl/handle/20.500.12932/47001</a></li>'
                        ]
                    }
                ]
            }
        ],
        en: [
            {
                title: 'Improve your problem-solving skills with heuristic trees',
                color: 'var(--color-tree-100)',
                plainContents: [
                    "When working on an assignment, you'll likely get suck at some point. " +
                        "At that point you might choose to look at a solution, but this won't " +
                        'immediately make you better at solving similar problems in the future.',
                    'Instead, you could use a heuristic tree: a set of hint-cards you can ' +
                        'follow through a tree structure.',
                    `<a href=${appLink('/all-trees')} class="text__link">Here</a> you may find ` +
                        'countless heuristic trees, you can get started on right away!'
                ],
                popOpenContents: [
                    {
                        heading: 'What is a heuristic?',
                        content: [
                            'Put very bluntly, a heuristic is a tool. ' +
                                'Such a tool may be as generic as a hammer or saw, ' +
                                'or as specific as a pair of crown wheel pliers for watchmakers.',
                            'Heuristics may serve as a rule of thumb in various areas, ' +
                                'to help solve a wide array of problems.'
                        ]
                    },
                    {
                        heading: 'What is a heuristic tree?',
                        content: [
                            'A heuristic tree offers hints structured in the form of a tree. ' +
                                'Heuristic trees are taylor made for a specific assignment, ' +
                                'and as such, they offer more direction than a general list of heuristics.'
                        ]
                    },
                    {
                        heading: 'How do I use a heuristic tree?',
                        content: [
                            'Upon opening a heuristic tree, all hint-cards are closed. ' +
                                'These can be opened from left to right, by clicking on the cards. ' +
                                'The heuristics start off general and get more concrete, ' +
                                'the deeper you crawl into the tree.',
                            'Most trees are separated into three main branches. ' +
                                'Each branch belongs to a phase: ' +
                                '<ul>' +
                                '<li><b>Orientation:</b> ' +
                                'here you explore the problem and you try to get a grip on it.' +
                                '</li>' +
                                '<li><b>Making & executing plans:</b> ' +
                                'here you come up with a strategy and try to execute it – at best, ' +
                                'until you have solved the problem.' +
                                '</li>' +
                                '<li><b>Afronding:</b> ' +
                                'Here you look back at the solution and your solution process, ' +
                                'and try to come up with learning points for the future.' +
                                '</li>' +
                                '</ul>'
                        ]
                    }
                ]
            },
            {
                title: 'Create new heuristic trees for your students',
                color: 'var(--color-sky-100)',
                plainContents: [
                    `By <a href=${appLink('/registration')} class="text__link">registering</a>, ` +
                        'an account, you can create heuristic trees for those difficult problems, ' +
                        'your students keep struggling with.',
                    'With our <i>tree-editor</i> you can easily create your own heuristic tree, ' +
                        'and publish it for free, so each of your students, ' +
                        'and hopefully even more, can make use of it.'
                ],
                popOpenContents: [
                    {
                        heading: 'How do I create a heuristic tree?',
                        content: [
                            'To start creating heuristic trees, you must first register an account ' +
                                `on the <a href=${appLink('/registration')} class="text__link">registration-page</a>.`,
                            'Once you have logged in, you can navigate to the ' +
                                `<a href=${appLink('/tree-generator')} class="text__link">
                                        tree-generator page</a> ` +
                                'from where you will be sent to the <i>tree-editor</i>, ' +
                                'after filling out some information on the tree.'
                        ]
                    },
                    {
                        heading: 'How do I edit my heuristic tree?',
                        content: [
                            'Within the <i>tree-editor</i> you can easily add, remove and ' +
                                'drag cards throughout your tree:' +
                                '<ul>' +
                                '<li>By clicking on one of the dashed boxed with a plus, ' +
                                'you can add a new card to the tree.</li>' +
                                '<li>In the sidebar that is opened upon adding a new card, ' +
                                'the front- and back-texts of the card can be edited. ' +
                                'The front-text is shown before the card has been opened, ' +
                                'and the back-text, containing the actual heuristic, afterward.</li>' +
                                '<li>This sidebar can be reopened later, by clicking on the ' +
                                'little pencil at the top-left of a card.</li>' +
                                '<li>A card can be deleted by clicking on the little bin ' +
                                'at the top-left of the card.</li>' +
                                '<li>A card can be moved, together with all its <i>children</i>, ' +
                                'by holding onto the arrows at the top-left of a card. ' +
                                'The card(s) will be placed on the dashed box above which ' +
                                'they are let go.</li>' +
                                '<li>You can undo and redo your last actions by clicking on ' +
                                'the forwards and backwards arrows at the top of the page, ' +
                                'or by using the keyboard shortcuts ' +
                                '<i>Ctrl + Z</i> and <i>Ctrl + Y</i>.</li>' +
                                '</ul>',
                            `By clicking on '${m.editMetadata()}' at the top of the page, ` +
                                'you can also edit the title, description and other information ' +
                                'of your tree.'
                        ]
                    },
                    {
                        heading: 'How can I format my cards?',
                        content: [
                            'Cards can be formatted by using ' +
                                '<a href="https://www.markdownguide.org/cheat-sheet/" target="_blank" class="text__link">' +
                                'Markdown</a>- and ' +
                                '<a href="https://katex.org/docs/supported" target="_blank" class="text__link">' +
                                'KaTeX</a>-notation.',
                            'With Markdown <span class="text__code">**Important text**</span> ' +
                                'will be displayed as <b>Important text</b>, for example, and ' +
                                'with KaTeX lots of mathmetical formulas can be displayed. ' +
                                'For example, a fraction can be displayed using this notation: ' +
                                '<span class="text__code">$\\frac{1}{2}$</span>.',
                            'See the above-linked documentation for all the possibilities. ' +
                                "Keep in mind, not everything stated under Markdown's " +
                                "'Extended Syntax' is supported."
                        ]
                    },
                    {
                        heading: 'How do I publish my heuristic tree?',
                        content: [
                            'On the <i>tree-editor</i>-page, you can easily publish your tree. ' +
                                `Click on '${m.editMetadata()}' and check the field ` +
                                `'${m.published()}', to publish the tree.`,
                            'To verify whether a tree of yours is published, you can check on the ' +
                                `<a href=${appLink('/my-trees')} class="text__link">` +
                                `my trees</a>-page, under the column '${m.published()}'.`
                        ]
                    },
                    {
                        heading: 'What research supports the use of heuristic trees?',
                        content: [
                            'Over the last few years, various studies have been conducted ' +
                                'into the use of heuristic trees for teaching mathematics ' +
                                'at Utrecht University.',
                            '<ul>' +
                                '<li>Bos, R., & van den Bogaart, T. (2022). Heuristic Trees as ' +
                                'a Digital Tool to Foster Compression and Decompression in ' +
                                'Problem‑Solving. ' +
                                '<i>Digital Experiences in Mathematics Education, 8</i>, 157–182. ' +
                                '<a href="https://doi.org/10.1007/s40751-022-00101-6" target="_blank" class="text__link">' +
                                'https://doi.org/10.1007/s40751-022-00101-6</a></li>' +
                                '<li>Lemmink, R. (2019): Improving Help-Seeking Behavior for ' +
                                'Online Mathematical Problem-Solving Lessons, ' +
                                '<a href="https://studenttheses.uu.nl/handle/20.500.12932/33363" target="_blank" class="text__link">' +
                                'https://studenttheses.uu.nl/handle/20.500.12932/33363</a></li>' +
                                '<li>Stokkermans, J. (2024), Supporting reasoning skills with ' +
                                'heuristic trees in pre-university mathematics, ' +
                                '<a href="https://studenttheses.uu.nl/handle/20.500.12932/48210" target="_blank" class="text__link">' +
                                'https://studenttheses.uu.nl/handle/20.500.12932/48210</a></li>' +
                                '<li>Zwennes, E. (2024), The fruits of heuristic trees, ' +
                                '<a href="https://studenttheses.uu.nl/handle/20.500.12932/47001" target="_blank" class="text__link">' +
                                'https://studenttheses.uu.nl/handle/20.500.12932/47001</a></li>'
                        ]
                    }
                ]
            }
        ]
    };

    onMount(() => {});
</script>

<div class="flex w-full justify-center">
    <div class="flex w-full flex-col items-center justify-center">
        <h1 class="page__title mb-8">{m.heuristicTrees()}</h1>
        <div class="alternating__container flex-col">
            {#each cards[getLocale()] as card (card)}
                {@render contentCard(card)}
            {/each}
        </div>
    </div>
</div>

{#snippet contentCard(content: CardContent)}
    <div class="flex" style={`--color-card: ${content.color}`}>
        <div class="card__container p-3">
            <div class="md:max-w-4/5">
                <h1 class="page__heading">{content.title}</h1>
                {#each content.plainContents as item (item)}
                    <p class="mb-1 last-of-type:mb-4">
                        <!-- We render rich text. -->
                        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                        {@html item}
                    </p>
                {/each}
            </div>
            <Accordion
                activeClass="bg-neutral-300 not-last:border-b-2!"
                inactiveClass="bg-white hover:bg-neutral-300 transition-colors duration-150 ease-in-out"
                class="overflow-hidden border-4 border-neutral-300 *:*:border-0! *:*:border-neutral-300! *:*:p-0!"
            >
                {#each content.popOpenContents as popOpenItem (popOpenItem)}
                    <AccordionItem
                        class="*:cursor-pointer *:rounded-none! *:border-0! *:p-0! *:pr-5! not-first:*:border-t-2! not-last:*:border-neutral-300! first:*:border-t-0!"
                    >
                        {#snippet header()}
                            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                                <p class="m-5 text-left text-lg">{popOpenItem.heading}</p>
                            </AnimatedUnderline>
                        {/snippet}
                        <div class="bg-white p-2">
                            {#each popOpenItem.content as paragraph (paragraph)}
                                <p class="mb-1 text-left">
                                    <!-- We render rich text. -->
                                    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                    {@html paragraph}
                                </p>
                            {/each}
                        </div>
                    </AccordionItem>
                {/each}
            </Accordion>
        </div>
    </div>
{/snippet}

<style>
    @layer components {
        .alternating__container {
            /* Layout */
            width: calc(2 / 3 * 100%);

            > * > * {
                width: calc(3 / 4 * 100%);
            }

            @media (width < 48rem) {
                width: calc(7 / 8 * 100%);

                > * > * {
                    width: calc(7 / 8 * 100%);
                }
            }

            > * {
                /* Layout */
                width: 100%;
                position: relative;
                margin-bottom: calc(var(--spacing) * 6);

                /* Card backdrop with offset */
                &::before {
                    content: '';
                    position: absolute;
                    z-index: -1;
                    height: calc(3 / 4 * 100%);
                    width: calc(3.3 / 5 * 100%);
                    @media (width < 48rem) {
                        width: calc(100% + var(--spacing) * 6);
                    }

                    /* Color provided in the style of the element */
                    background-color: var(--color-card);
                }

                /* Left-aligned cards */
                &:nth-child(odd) {
                    /* Layout */
                    justify-content: flex-start;

                    .page__heading {
                        text-align: left;

                        &::before {
                            left: calc(var(--spacing) * -3);
                        }
                    }

                    /* Offset to the left */
                    &::before {
                        top: calc(var(--spacing) * -2);
                        left: calc(var(--spacing) * -6);
                        border-radius: var(--radius-2xl) var(--radius-sm) var(--radius-2xl)
                            var(--radius-sm);

                        @media (width < 48rem) {
                            left: calc(var(--spacing) * -4);
                        }
                    }
                }

                /* Right-aligned cards */
                &:nth-child(even) {
                    /* Layout */
                    justify-content: flex-end;

                    .page__heading {
                        text-align: right;

                        &::before {
                            right: calc(var(--spacing) * -3);
                        }
                    }

                    /* Align content to the right */
                    .card__container {
                        display: flex;
                        flex-direction: column;
                        justify-content: right;
                        align-items: flex-end;
                    }

                    /* Offset to the right */
                    &::before {
                        top: calc(var(--spacing) * -2);
                        right: calc(var(--spacing) * -6);
                        border-radius: var(--radius-sm) var(--radius-2xl) var(--radius-sm)
                            var(--radius-2xl);

                        @media (width < 48rem) {
                            right: calc(var(--spacing) * -4);
                        }
                    }
                }
            }

            /* Card headings */
            .page__heading {
                position: relative;
                margin-bottom: var(--spacing-padding-small);

                /* Heading marker */
                &::before {
                    /* Layout */
                    content: '';
                    position: absolute;
                    bottom: 5%;
                    height: 87.5%;
                    z-index: 1;

                    /* Formatting */
                    background-color: var(--color-neutral-350);
                    width: var(--spacing-padding-minor);
                    transition: background-color 200ms ease-in-out;
                }

                &:hover::before {
                    background-color: var(--color-neutral-500);
                }
            }
        }
    }
</style>
