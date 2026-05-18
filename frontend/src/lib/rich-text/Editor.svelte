<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import 'prosekit/basic/style.css';
    import 'prosekit/basic/typography.css';

    import {
        createEditor,
        defineBaseCommands,
        defineBaseKeymap,
        defineHistory,
        jsonFromHTML,
        union,
        type NodeJSON
    } from 'prosekit/core';
    import { ProseKit, useDocChange } from 'prosekit/svelte';
    import { onMount } from 'svelte';
    import { markdownFromProsekitHtml, markdownToProsekitHtml } from './markdown';
    import Toolbar from './Toolbar.svelte';

    import { defineImageViewerParserExtension } from './imageProsekit';
    import { defineBlockquote } from '@prosekit/extensions/blockquote';
    import { defineBold } from '@prosekit/extensions/bold';
    // import { defineCode } from '@prosekit/extensions/code';
    // import { defineCodeBlock } from '@prosekit/extensions/code-block';
    import { defineDoc } from '@prosekit/extensions/doc';
    import { defineGapCursor } from '@prosekit/extensions/gap-cursor';
    import { defineHardBreak } from '@prosekit/extensions/hard-break';
    import { defineHeading } from '@prosekit/extensions/heading';
    import { defineHorizontalRule } from '@prosekit/extensions/horizontal-rule';
    import { defineItalic } from '@prosekit/extensions/italic';
    import { defineLink } from '@prosekit/extensions/link';
    import { defineList } from '@prosekit/extensions/list';
    import { defineModClickPrevention } from '@prosekit/extensions/mod-click-prevention';
    import { defineParagraph } from '@prosekit/extensions/paragraph';
    import { defineTable } from '@prosekit/extensions/table';
    import { defineText } from '@prosekit/extensions/text';
    // import { defineUnderline } from '@prosekit/extensions/underline';
    import { defineVirtualSelection } from '@prosekit/extensions/virtual-selection';
    import { defineImageCommands } from '@prosekit/extensions/image';
    import { defineKatexMath } from './math';

    let {
        maxLength,
        valueText = $bindable()
    }: {
        maxLength: number;
        valueText: string;
    } = $props();

    const extension = union(
        // Nodes
        defineDoc(),
        defineText(),
        defineParagraph(),
        defineHeading(),
        defineList(),
        defineBlockquote(),
        defineHorizontalRule(),
        defineHardBreak(),
        defineTable(),
        // defineCodeBlock(),

        // Own custom stuff
        defineImageViewerParserExtension(),
        defineImageCommands(),

        // Marks
        defineItalic(),
        defineBold(),
        // defineUnderline(),
        // defineStrike(),
        // defineCode(),
        defineLink(),

        // Others
        defineBaseKeymap(),
        defineBaseCommands(),
        defineHistory(),
        defineGapCursor(),
        defineVirtualSelection(),
        defineModClickPrevention(),

        defineKatexMath()
    );

    const editor = createEditor({ extension });

    /**
     * Used to prevent changes to the valueText by the editor to be send to the editor again,
     * as prosekit will make those changes internally.
     */
    let lastTextExported: null | string = $state(null);
    $effect(() => {
        if (lastTextExported === valueText) return;

        let newProsekitJson = jsonFromHTML(markdownToProsekitHtml(valueText), {
            schema: editor.schema
        });

        // Remove empty paragraphs that prosekit creates from the <p><img .../></p>
        const filterEmptyParagraphs = (node: NodeJSON) => {
            if (node.content === undefined) return;

            for (let i = 0; i < node.content.length; i++) {
                let subnode = node.content[i];
                if (subnode.type === 'paragraph' && subnode.content === undefined) {
                    node.content.splice(i, 1);
                    i--;
                    continue;
                }

                filterEmptyParagraphs(subnode);
            }
        };
        filterEmptyParagraphs(newProsekitJson);

        console.log('Editor got updated new markdown:', valueText);
        editor.setContent(newProsekitJson);
    });

    useDocChange(
        () => {
            // TODO: is this too expensive to run on every keypress?
            // if so we need to implement debouncing.
            let html = editor.getDocHTML();
            let markdown = markdownFromProsekitHtml(html);
            lastTextExported = markdown;
            valueText = markdown;

            console.log('Editor output markdown:', valueText);
        },
        { editor }
    );

    onMount(() => {
        // This is very useful for the sidebar,
        // if we want this editor in other places we might want to make this optional.
        editor.focus();
    });
</script>

<ProseKit {editor}>
    <div
        class="box-border flex h-full min-h-36 w-full flex-col overflow-x-hidden overflow-y-hidden rounded-md border border-solid border-gray-200 bg-[canvas] text-black shadow-sm dark:border-gray-700 dark:text-white"
    >
        <Toolbar />
        <div class="relative box-border w-full flex-1 overflow-y-auto">
            <div class="ProseMirror min-h-full p-1 outline-hidden" {@attach editor.mount}></div>
        </div>
        <p class="text-xs" class:text-red-700={valueText.length > maxLength}>
            {valueText.length}/{maxLength}
        </p>
    </div>
</ProseKit>

<style>
    /* Default prosekit style has a lot of additional room, but that looks weird */
    :global(.ProseMirror pre) {
        padding: 1rem;
    }
    :global(.ProseMirror img) {
        margin: 0;
    }
    :global(.ProseMirror p) {
        margin: 0;
    }
    :global(.ProseMirror .prosemirror-math-block) {
        margin: 0 !important;
        :global(.prosemirror-math-display) {
            padding: 0 !important;
        }
    }
</style>
