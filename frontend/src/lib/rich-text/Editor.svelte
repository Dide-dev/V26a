<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    // We do not use the recommended 'typography.css' from prosekit,
    // since that would make it no longer 'what you see is what you get'
    // Instead, at the bottom of this file are some of the styles to make
    // math editing work nicely. More styles are not needed for us.
    import 'prosekit/basic/style.css';

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
    import { DropIndicator } from 'prosekit/svelte/drop-indicator';

    import { defineImageViewerParserExtension } from './imageProsekit';
    import { defineBlockquote } from '@prosekit/extensions/blockquote';
    import { defineBold } from '@prosekit/extensions/bold';
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
    import { defineVirtualSelection } from '@prosekit/extensions/virtual-selection';
    import { defineImageCommands } from '@prosekit/extensions/image';
    import { defineKatexMath } from './math';
    import { defineCodeBlock } from '@prosekit/extensions/code-block';
    import { defineCode } from '@prosekit/extensions/code';

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
        // If this one ever gets removed, please add the `defineCodeBlockKeymap`,
        // so that pressing 'enter' a bunch in math block mode, the user can exit it.
        defineCodeBlock(),

        // Own custom stuff
        defineImageViewerParserExtension(),
        defineImageCommands(),

        // Marks
        defineItalic(),
        defineBold(),
        defineCode(),
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
                if (
                    node.content[i + 1]?.type === 'image' &&
                    subnode.type === 'paragraph' &&
                    subnode.content === undefined
                ) {
                    node.content.splice(i, 1);
                    i--;
                    continue;
                }

                filterEmptyParagraphs(subnode);
            }
        };
        if (valueText.trim() !== '') {
            filterEmptyParagraphs(newProsekitJson);
        }

        editor.setContent(newProsekitJson);
    });

    useDocChange(
        () => {
            // If this ever turns out to be too expensive to run on every keypress,
            // you will need to implement debouncing.
            let html = editor.getDocHTML();
            let markdown = markdownFromProsekitHtml(html);
            lastTextExported = markdown;
            valueText = markdown;
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
        class="box-border flex h-full min-h-36 w-full flex-col rounded-md border border-solid border-gray-200 bg-[canvas] text-black shadow-sm dark:border-gray-700 dark:text-white"
    >
        <Toolbar />
        <DropIndicator class="z-50 bg-blue-500 transition-all" />
        <div class="relative box-border w-full flex-1 overflow-y-auto">
            <div class="ProseMirror min-h-full p-1 outline-hidden" {@attach editor.mount}></div>
        </div>
        <p class="text-xs" class:text-red-700={valueText.length > maxLength}>
            {valueText.length}/{maxLength}
        </p>
    </div>
</ProseKit>

<style>
    :global(.ProseMirror) {
        /** Force math blocks to be visible if empty */
        :global(pre.prosemirror-math-source:has(.ProseMirror-trailingBreak)) {
            display: block !important;

            /** hide the empty vertical space created by the empty katex display */
            :global(+ .prosemirror-math-display) {
                display: none;
            }
        }
        /** Same for inline math */
        :global(span.prosemirror-math-source:has(code:empty)) {
            opacity: unset !important;
            max-width: unset !important;
            max-height: unset !important;
            pointer-events: unset !important;
            overflow: unset !important;
        }

        :global(.prosemirror-math-inline .prosemirror-math-source) {
            :global(&:before, &:after) {
                content: '$';
                opacity: 0.5;
            }
        }

        :global(pre) {
            border: 1px solid #9993;
            border-radius: 0.375rem;
            margin: 0.5rem 0;
            padding: 1rem;
            overflow-x: auto;
        }

        :global(code) {
            font-size: 0.875em;
            line-height: 1.6rem;
        }
    }
</style>
