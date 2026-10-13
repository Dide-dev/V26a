/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { m } from '$lib/paraglide/messages.js';
import { appLink } from '$lib';
import helpSchemeDutch from '$lib/assets/helpschema.jpg';
import helpSchemeEnglish from '$lib/assets/helpscheme.png';

/** The contents of cards to display on the home-page, for each locale. */
export const homePageCards = {
    nl: [
        {
            title: 'Werken aan probleemaanpak met heuristiekbomen',
            color: 'var(--color-tree-100)',
            plainContents: [
                'Heuristiekbomen bieden hulp bij het aanpakken van problemen. Een heuristiekboom ' +
                    'raadplegen is beter dan het antwoordmodel erbij te pakken, omdat de heuristiekboom ' +
                    'zich richt op het leren van heuristieken, concepten, technieken, ' +
                    'en metacognitieve vaardigheden.',
                'Docenten en anderen hebben al heel wat heuristiekbomen gemaakt, en die vind je ' +
                    `<a href=${appLink('/trees')} class="text__link">hier</a>!`
            ],
            popOpenContents: [
                {
                    heading: 'Wat is een heuristiek?',
                    content: [
                        'Een heuristiek is een abstracte hint: een hint die zo is geformuleerd dat hij niet alleen van ' +
                            'toepassing is op één probleem, maar op heel veel problemen. Dat betekent dat de hint in termen ' +
                            'is van algemene concepten, technieken en proposities die van toepassing zijn op het probleem.'
                    ]
                },
                {
                    heading: 'Wat is een heuristiekboom?',
                    content: [
                        'Een heuristiekboom is een verzameling hintkaartjes bij een probleem, georganiseerd in een ' +
                            'boomstructuur. Er is een lijstje ontwerpprincipes voor heuristiekbomen, maar de belangrijkste ' +
                            'is dat de hints langs de takken abstract beginnen en dan steeds concreter worden. De eerste ' +
                            'kaartjes zijn dus algemene heuristieken, en pas bij latere kaartjes wordt preciezer duidelijk ' +
                            'hoe die van toepassing zijn op het probleem. Daarnaast biedt de heuristiekboom de mogelijkheid ' +
                            'om de takken te labelen naar bepaalde fasen van probleemaanpak. Dat kan afhangen van het type ' +
                            'probleem, maar over het algemeen is het iets als oriëntatie, plan maken en uitvoeren, en afronding.'
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
                        'De meeste bomen zijn opgedeeld in drie hoofdtakken. ' +
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
                        `<a href=${helpSchemeDutch} target="_blank">
                                    <img src=${helpSchemeDutch} alt="Hulpschema voor bij het doorlopen van een heuristiekboom.">
                                </a>`
                    ]
                }
            ]
        },
        {
            title: 'Creëer nieuwe heuristiekbomen voor je leerlingen',
            color: 'var(--color-sky-100)',
            plainContents: [
                `Door je te <a href=${appLink('/registration')} class="text__link">registreren</a>, ` +
                    'kun je zelf nieuwe heuristiekbomen creëren voor lastige opgaves ' +
                    'waar je leerlingen moeite mee hebben.',
                'Met onze <i>tree editor</i> maak je gemakkelijk een heuristiekboom ' +
                    'en publiceer je deze gratis, zodat al jouw leerlingen ' +
                    'en ieder ander die dat wil, daar gebruik van kunnen maken.'
            ],
            popOpenContents: [
                {
                    heading: 'Hoe maak ik een heuristiekboom?',
                    content: [
                        'Om te beginnen met het maken van heuristiekbomen ' +
                            'moet je eerst een account aanmaken op de ' +
                            `<a href=${appLink('/registration')} class="text__link">registratie</a>-pagina.`,
                        'Als je eenmaal bent ingelogd, kun je je begeven naar de ' +
                            `<a href=${appLink('/trees/add')} class="text__link">
                                        boom-creatie</a>-pagina, ` +
                            'waarvandaan je, na het invullen van wat informatie over de boom, ' +
                            'wordt doorgeleid naar de <i>tree editor</i>.'
                    ]
                },
                {
                    heading: 'Hoe bewerk ik mijn heuristiekboom?',
                    content: [
                        'Vanuit de <i>tree editor</i> kun je gemakkelijk kaartjes ' +
                            'toevoegen, verwijderen en verslepen: ' +
                            '<ul>' +
                            '<li>Door op een van de gestippelde kaders met een plusje ' +
                            'te klikken voeg je een nieuwe kaart toe.</li>' +
                            '<li>In het zijmenu opent bij het toevoegen van een kaart het menu ' +
                            'waarop de tekst op de voor- en achterkant kan worden bewerkt. ' +
                            'Tijdens het gebruiken van de boom, wordt ' +
                            'de voortekst getoond voordat de kaart is geopend, ' +
                            'en de achterkant, die dus de heuristiek bevat, daarna.</li>' +
                            '<li>Dit zijmenu om de inhoud van een kaart te wijzigen ' +
                            'kan later weer worden geopend door te klikken op het potloodje ' +
                            'linksboven een kaart.</li>' +
                            '<li>Een kaart kan worden verwijderd door te klikken op het vuilnisbakje ' +
                            'linksboven een kaart.</li>' +
                            '<li>Een kaart kan samen met al haar <i>kinderen</i> worden versleept ' +
                            'door de pijltjes linksboven in een kaart vast te grijpen. ' +
                            'De kaart(en) worden geplaatst op het gestippelde kader ' +
                            'waarboven ze worden losgelaten.</li>' +
                            '<li>Je kunt je laatste acties ongedaan maken door te klikken ' +
                            'op de pijltjes bovenaan de pagina, of door gebruik te maken van ' +
                            'de sneltoetsen <i>Ctrl + Z</i> en <i>Ctrl + Y</i>.</li>' +
                            '</ul>',
                        `Door op '${m.editMetadata()}' te klikken in het zijmenu, ` +
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
                        'Het publiceren van de boom kan vanaf de <i>tree editor</i>-pagina. ' +
                            `Klik daar op '${m.share()}' in het zijmenu en klik op de knop ` +
                            `'${m.publish()}', om de boom te publiceren.`,
                        'Nadat de boom is gepubliceerd, kun je hier ook de link naar ' +
                            'de boom kopiëren.'
                    ]
                },
                {
                    heading: 'Hoe druk ik een heuristiekboom af?',
                    content: [
                        'Vanaf de weergave-pagina van een boom kun je op de print-knop naast de titel ' +
                            'klikken. Deze leidt je naar een versimpelde versie van de pagina, ' +
                            'waar je de breedte van de kaartjes kunt instellen voor een passend ' +
                            'post-it formaat.',
                        'Zodra je dat correct hebt ingesteld, kun je de pagina afdrukken (naar pdf) ' +
                            'door de toetsen <i>Ctrl + P</i> in te drukken, en de stappen in het ' +
                            'dialoogvenster te volgen. Hierbij kun je uit verschillende formaten papier ' +
                            'kiezen, en kiezen de achtergronden van de kaartjes wit te laten.',
                        'Let er bij het printen op dat je het optie- en informatie-menu bovenaan ' +
                            'de pagina hebt gesloten, en dat de schaal bij het printen op 100 blijft staan.'
                    ]
                },
                {
                    heading: 'Welk onderzoek ligt er achter het gebruik van heuristiekbomen?',
                    content: [
                        'De afgelopen jaren zijn er aan de Universiteit Utrecht en Hogeschool Utrecht ' +
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
                            '<i>Digital Experiences in Mathematics Education</i>, 8, 157–182. ' +
                            '<a href="https://doi.org/10.1007/s40751-022-00101-6" target="_blank" class="text__link">' +
                            'https://doi.org/10.1007/s40751-022-00101-6</a></li>' +
                            '<li>Bos, R. (2025). Heuristic trees and their fruits. In M. Bosch, G. Bolondi, S. Carreira, C. Spagnolo, & M. Gaidoschik (Eds.), ' +
                            'Proceedings of the Fourteenth Congress of the European Society for Research in Mathematics Education (CERME14) (pp. 1-8). ' +
                            'Free University of Bozen-Bolzano and ERME. ' +
                            '<a href="https://hal.science/CERME14/hal-05283597v1" target="_blank" class=text__link">https://hal.science/CERME14/hal-05283597v1</a></li>' +
                            "<li>Bos, R., & van den Bogaart, T. (2022). Teachers' design of heuristic trees. Hiroshima Journal for Mathematics Education, 15(1). " +
                            '<a href="https://https://www.jstage.jst.go.jp/article/hjme/15/1/15_1502/_article" class="text__link" target="_blank">[DOI]</a></li>' +
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
                            'https://studenttheses.uu.nl/handle/20.500.12932/47001</a></li>' +
                            '</ul>'
                    ]
                }
            ]
        }
    ],
    en: [
        {
            title: 'Improving problem-solving skills with heuristic trees',
            color: 'var(--color-tree-100)',
            plainContents: [
                'Heuristic trees provide assistance when tackling problems. Using a heuristic tree ' +
                    'for assistance is better than checking the solutions to the problem, because a ' +
                    'heuristic tree focuses on teaching heuristics, concepts, techniques and meta-cognitive abilities.',
                'Teachers and other users have already made many heuristic trees, which you can find ' +
                    `<a href=${appLink('/trees')} class="text__link">here</a>!`
            ],
            popOpenContents: [
                {
                    heading: 'What is a heuristic?',
                    content: [
                        "A heuristic is an abstract hint: it's formulated as such that it's not just " +
                            'applicable to one problem, but to many problems. This means that a hint is ' +
                            'written using general concepts, techniques and propositions that apply ' +
                            'to the problem.'
                    ]
                },
                {
                    heading: 'What is a heuristic tree?',
                    content: [
                        'A heuristic tree is a collection of hints for a problem, organized in a tree structure. ' +
                            "There's a list of design principles for heuristic trees, but the most important one is " +
                            'that hints should start from abstract and go towards more concrete as you traverse the branches. ' +
                            'Thus, the first few cards should contain general heuristics, and the cards you find later ' +
                            'should clarify how the heuristics apply to the problem. The heuristic trees also provide the ' +
                            'option to label different branches to different phases of the problem-solving process. These ' +
                            'phases can differ based on the type of problem, but the standard labels are orientation, ' +
                            'making & executing plans, and completion.'
                    ]
                },
                {
                    heading: 'How do I use a heuristic tree?',
                    content: [
                        'Upon opening a heuristic tree, all hint-cards are closed. ' +
                            'These can be opened from left to right, by clicking on the cards. ' +
                            'The heuristics start off abstract and get more concrete, ' +
                            'the deeper you crawl into the tree.',
                        'Most trees are separated into three main branches. ' +
                            'Each branch belongs to a phase: ' +
                            '<ul>' +
                            '<li><b>Orientation:</b> ' +
                            'here you explore the problem and you try to grasp it.' +
                            '</li>' +
                            '<li><b>Making & executing plans:</b> ' +
                            'here you come up with a strategy and try to execute it, ' +
                            'until you have solved the problem.' +
                            '</li>' +
                            '<li><b>Completion:</b> ' +
                            'here you look back at the solution and your solution process, ' +
                            'and try to come up with learning points for the future.' +
                            '</li>' +
                            '</ul>',
                        'While traversing the tree, you may use the following help scheme:',
                        `<a href=${helpSchemeEnglish} target="_blank">
                                <img src=${helpSchemeEnglish} alt="Help scheme for traversing heuristic trees.">
                            </a>`
                    ]
                }
            ]
        },
        {
            title: 'Create new heuristic trees for your students',
            color: 'var(--color-sky-100)',
            plainContents: [
                `By <a href=${appLink('/registration')} class="text__link">registering</a> ` +
                    'an account, you can create heuristic trees for those difficult problems ' +
                    'your students keep struggling with.',
                'With our tree editor you can easily create your own heuristic tree ' +
                    'and publish it for free, so each of your students, ' +
                    'and anyone else interested, can make use of it.'
            ],
            popOpenContents: [
                {
                    heading: 'How do I create a heuristic tree?',
                    content: [
                        'To start creating heuristic trees, you must first register an account ' +
                            `on the <a href=${appLink('/registration')} class="text__link">registration</a>-page.`,
                        'Once you have logged in, you can navigate to the ' +
                            `<a href=${appLink('/trees/add')} class="text__link">
                                        tree generator</a>-page ` +
                            'from where you will be sent to the tree editor, ' +
                            'after filling out some information on the tree.'
                    ]
                },
                {
                    heading: 'How do I edit my heuristic tree?',
                    content: [
                        'Within the tree editor you can easily add, remove and ' +
                            'drag cards throughout your tree:' +
                            '<ul>' +
                            '<li>By clicking on one of the dashed boxes containing a plus, ' +
                            'you can add a new card to the tree.</li>' +
                            '<li>Upon adding a new card, a menu opens in the sidebar where ' +
                            'the texts on the front and back of the card can be edited. ' +
                            'When using the tree, the front-text is shown before the card has been opened, ' +
                            'and the back-text, containing the actual heuristic, afterward.</li>' +
                            '<li>This menu in the sidebar can be reopened later, by clicking on the ' +
                            'little pencil at the top-left of a card, to edit its contents.</li>' +
                            '<li>A card can be deleted by clicking on the little bin ' +
                            'at the top-left of the card.</li>' +
                            '<li>A card can be moved, together with all its <i>children</i>, ' +
                            'by holding onto the arrows at the top-left of a card. ' +
                            'The card(s) will be placed on the highlighted dashed box above which ' +
                            'they are let go.</li>' +
                            '<li>You can undo and redo your last actions by clicking on ' +
                            'the forward and backward arrows at the top of the page, ' +
                            'or by using the keyboard shortcuts ' +
                            '<i>Ctrl + Z</i> and <i>Ctrl + Y</i>.</li>' +
                            '</ul>',
                        `By clicking on '${m.editMetadata()}' in the sidebar, ` +
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
                            'with KaTeX lots of mathematical formulas can be displayed. ' +
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
                        'On the tree editor-page, you can easily publish your tree. ' +
                            `Click on '${m.share()}' in the sidebar and click on ` +
                            `the '${m.publish()}'-button, to publish the tree.`,
                        'After your tree has been published, you can also copy the link to ' +
                            'your tree, from this menu.'
                    ]
                },
                {
                    heading: 'How do I print a heuristic tree?',
                    content: [
                        'On the viewer page of a tree, you can press the print-button next to the title, ' +
                            'which will lead you to a simplified version of the page, where you can ' +
                            'adjust the size of the cards to match  your preferred post-it size.',
                        'Once this has been set correctly, you can print the page (to a pdf) by pressing ' +
                            '<i>Ctrl + P</i>, and following the steps in the dialogue window.' +
                            'Here you can choose between different sizes of paper sheets, ' +
                            'and decide to leave the background of the cards blank.',
                        'Before printing, make sure you have collapsed the options/information-menu, ' +
                            'and the scale is (still) set to 100.'
                    ]
                },
                {
                    heading: 'What research supports the use of heuristic trees?',
                    content: [
                        'Over the last few years, various studies have been conducted ' +
                            'into the use of heuristic trees for teaching mathematics ' +
                            'at Utrecht University and Utrecht University of Applied Sciences.',
                        '<ul>' +
                            '<li>Bos, R., & van den Bogaart, T. (2022). Heuristic Trees as ' +
                            'a Digital Tool to Foster Compression and Decompression in ' +
                            'Problem‑Solving. ' +
                            '<i>Digital Experiences in Mathematics Education</i>, 8, 157–182. ' +
                            '<a href="https://doi.org/10.1007/s40751-022-00101-6" target="_blank" class="text__link">' +
                            'https://doi.org/10.1007/s40751-022-00101-6</a></li>' +
                            '<li>Bos, R. (2025). Heuristic trees and their fruits. In M. Bosch, G. Bolondi, S. Carreira, C. Spagnolo, & M. Gaidoschik (Eds.), ' +
                            'Proceedings of the Fourteenth Congress of the European Society for Research in Mathematics Education (CERME14) (pp. 1-8). ' +
                            'Free University of Bozen-Bolzano and ERME. ' +
                            '<a href="https://hal.science/CERME14/hal-05283597v1" target="_blank" class=text__link">https://hal.science/CERME14/hal-05283597v1</a></li>' +
                            "<li>Bos, R., & van den Bogaart, T. (2022). Teachers' design of heuristic trees. Hiroshima Journal for Mathematics Education, 15(1). " +
                            '<a href="https://https://www.jstage.jst.go.jp/article/hjme/15/1/15_1502/_article" class="text__link" target="_blank">[DOI]</a></li>' +
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
                            'https://studenttheses.uu.nl/handle/20.500.12932/47001</a></li>' +
                            '</ul>'
                    ]
                }
            ]
        }
    ]
};
