<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { BasicExtension } from 'prosekit/basic';
    import type { Editor } from 'prosekit/core';
    import { useEditorDerivedValue } from 'prosekit/svelte';
    import { m } from '$lib/paraglide/messages';

    import ToolbarButton from './ToolbarButton.svelte';
    import ImageUploader from './ImageUploader.svelte';

    function getToolbarItems(editor: Editor<BasicExtension>) {
        return {
            undo: editor.commands.undo
                ? {
                      isActive: false,
                      canExec: editor.commands.undo.canExec(),
                      command: () => editor.commands.undo()
                  }
                : undefined,
            redo: editor.commands.redo
                ? {
                      isActive: false,
                      canExec: editor.commands.redo.canExec(),
                      command: () => editor.commands.redo()
                  }
                : undefined,
            bold: editor.commands.toggleBold
                ? {
                      isActive: editor.marks.bold.isActive(),
                      canExec: editor.commands.toggleBold.canExec(),
                      command: () => editor.commands.toggleBold()
                  }
                : undefined,
            italic: editor.commands.toggleItalic
                ? {
                      isActive: editor.marks.italic.isActive(),
                      canExec: editor.commands.toggleItalic.canExec(),
                      command: () => editor.commands.toggleItalic()
                  }
                : undefined,
            underline: editor.commands.toggleUnderline
                ? {
                      isActive: editor.marks.underline.isActive(),
                      canExec: editor.commands.toggleUnderline.canExec(),
                      command: () => editor.commands.toggleUnderline()
                  }
                : undefined,
            strike: editor.commands.toggleStrike
                ? {
                      isActive: editor.marks.strike.isActive(),
                      canExec: editor.commands.toggleStrike.canExec(),
                      command: () => editor.commands.toggleStrike()
                  }
                : undefined,
            code: editor.commands.toggleCode
                ? {
                      isActive: editor.marks.code.isActive(),
                      canExec: editor.commands.toggleCode.canExec(),
                      command: () => editor.commands.toggleCode()
                  }
                : undefined,
            codeBlock: editor.commands.insertCodeBlock
                ? {
                      isActive: editor.nodes.codeBlock.isActive(),
                      canExec: editor.commands.insertCodeBlock.canExec({ language: 'javascript' }),
                      command: () => editor.commands.insertCodeBlock({ language: 'javascript' })
                  }
                : undefined,
            heading1: editor.commands.toggleHeading
                ? {
                      isActive: editor.nodes.heading.isActive({ level: 1 }),
                      canExec: editor.commands.toggleHeading.canExec({ level: 1 }),
                      command: () => editor.commands.toggleHeading({ level: 1 })
                  }
                : undefined,
            heading2: editor.commands.toggleHeading
                ? {
                      isActive: editor.nodes.heading.isActive({ level: 2 }),
                      canExec: editor.commands.toggleHeading.canExec({ level: 2 }),
                      command: () => editor.commands.toggleHeading({ level: 2 })
                  }
                : undefined,
            heading3: editor.commands.toggleHeading
                ? {
                      isActive: editor.nodes.heading.isActive({ level: 3 }),
                      canExec: editor.commands.toggleHeading.canExec({ level: 3 }),
                      command: () => editor.commands.toggleHeading({ level: 3 })
                  }
                : undefined,
            horizontalRule: editor.commands.insertHorizontalRule
                ? {
                      isActive: editor.nodes.horizontalRule.isActive(),
                      canExec: editor.commands.insertHorizontalRule.canExec(),
                      command: () => editor.commands.insertHorizontalRule()
                  }
                : undefined,
            blockquote: editor.commands.toggleBlockquote
                ? {
                      isActive: editor.nodes.blockquote.isActive(),
                      canExec: editor.commands.toggleBlockquote.canExec(),
                      command: () => editor.commands.toggleBlockquote()
                  }
                : undefined,
            bulletList: editor.commands.toggleList
                ? {
                      isActive: editor.nodes.list.isActive({ kind: 'bullet' }),
                      canExec: editor.commands.toggleList.canExec({ kind: 'bullet' }),
                      command: () => editor.commands.toggleList({ kind: 'bullet' })
                  }
                : undefined,
            orderedList: editor.commands.toggleList
                ? {
                      isActive: editor.nodes.list.isActive({ kind: 'ordered' }),
                      canExec: editor.commands.toggleList.canExec({ kind: 'ordered' }),
                      command: () => editor.commands.toggleList({ kind: 'ordered' })
                  }
                : undefined,

            // These are not used by us
            // taskList: editor.commands.toggleList
            //     ? {
            //           isActive: editor.nodes.list.isActive({ kind: 'task' }),
            //           canExec: editor.commands.toggleList.canExec({ kind: 'task' }),
            //           command: () => editor.commands.toggleList({ kind: 'task' })
            //       }
            //     : undefined,
            // toggleList: editor.commands.toggleList
            //     ? {
            //           isActive: editor.nodes.list.isActive({ kind: 'toggle' }),
            //           canExec: editor.commands.toggleList.canExec({ kind: 'toggle' }),
            //           command: () => editor.commands.toggleList({ kind: 'toggle' })
            //       }
            //     : undefined,
            indentList: editor.commands.indentList
                ? {
                      isActive: false,
                      canExec: editor.commands.indentList.canExec(),
                      command: () => editor.commands.indentList()
                  }
                : undefined,
            dedentList: editor.commands.dedentList
                ? {
                      isActive: false,
                      canExec: editor.commands.dedentList.canExec(),
                      command: () => editor.commands.dedentList()
                  }
                : undefined,
            insertImage: editor.commands.insertImage
                ? {
                      isActive: false,
                      canExec: editor.commands.insertImage.canExec()
                  }
                : undefined
        };
    }

    const items = useEditorDerivedValue(getToolbarItems);
</script>

<div
    class="z-2 box-border flex flex-wrap items-center justify-between gap-1 border-t-0 border-r-0 border-b border-l-0 border-solid border-gray-200 p-1 dark:border-gray-800"
>
    {#if $items.undo}
        <ToolbarButton
            pressed={$items.undo.isActive}
            disabled={!$items.undo.canExec}
            onClick={$items.undo.command}
            tooltip={m.undo()}
        >
            <div class="i-lucide-undo-2 block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.redo}
        <ToolbarButton
            pressed={$items.redo.isActive}
            disabled={!$items.redo.canExec}
            onClick={$items.redo.command}
            tooltip={m.redo()}
        >
            <div class="i-lucide-redo-2 block size-5"></div>
        </ToolbarButton>
    {/if}

    {#if $items.bold}
        <ToolbarButton
            pressed={$items.bold.isActive}
            disabled={!$items.bold.canExec}
            onClick={$items.bold.command}
            tooltip={m.bold()}
        >
            <div class="i-lucide-bold block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.italic}
        <ToolbarButton
            pressed={$items.italic.isActive}
            disabled={!$items.italic.canExec}
            onClick={$items.italic.command}
            tooltip={m.italic()}
        >
            <div class="i-lucide-italic block size-5"></div>
        </ToolbarButton>
    {/if}
    <!-- {#if $items.underline}
        <ToolbarButton
            pressed={$items.underline.isActive}
            disabled={!$items.underline.canExec}
            onClick={$items.underline.command}
            tooltip="Underline"
        >
            <div class="i-lucide-underline block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.strike}
        <ToolbarButton
            pressed={$items.strike.isActive}
            disabled={!$items.strike.canExec}
            onClick={$items.strike.command}
            tooltip="Strike"
        >
            <div class="i-lucide-strikethrough block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.code}
        <ToolbarButton
            pressed={$items.code.isActive}
            disabled={!$items.code.canExec}
            onClick={$items.code.command}
            tooltip="Code"
        >
            <div class="i-lucide-code block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.codeBlock}
        <ToolbarButton
            pressed={$items.codeBlock.isActive}
            disabled={!$items.codeBlock.canExec}
            onClick={$items.codeBlock.command}
            tooltip="Code Block"
        >
            <div class="i-lucide-square-code block size-5"></div>
        </ToolbarButton>
    {/if} -->
    {#if $items.heading1}
        <ToolbarButton
            pressed={$items.heading1.isActive}
            disabled={!$items.heading1.canExec}
            onClick={$items.heading1.command}
            tooltip={m.headingN({ n: 1 })}
        >
            <div class="i-lucide-heading-1 block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.heading2}
        <ToolbarButton
            pressed={$items.heading2.isActive}
            disabled={!$items.heading2.canExec}
            onClick={$items.heading2.command}
            tooltip={m.headingN({ n: 2 })}
        >
            <div class="i-lucide-heading-2 block size-5"></div>
        </ToolbarButton>
    {/if}
    <!-- {#if $items.heading3}
        <ToolbarButton
            pressed={$items.heading3.isActive}
            disabled={!$items.heading3.canExec}
            onClick={$items.heading3.command}
            tooltip="Heading 3"
        >
            <div class="i-lucide-heading-3 block size-5"></div>
        </ToolbarButton>
    {/if} -->
    {#if $items.horizontalRule}
        <ToolbarButton
            pressed={$items.horizontalRule.isActive}
            disabled={!$items.horizontalRule.canExec}
            onClick={$items.horizontalRule.command}
            tooltip={m.divider()}
        >
            <div class="i-lucide-minus block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.blockquote}
        <ToolbarButton
            pressed={$items.blockquote.isActive}
            disabled={!$items.blockquote.canExec}
            onClick={$items.blockquote.command}
            tooltip={m.blockquote()}
        >
            <div class="i-lucide-text-quote block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.bulletList}
        <ToolbarButton
            pressed={$items.bulletList.isActive}
            disabled={!$items.bulletList.canExec}
            onClick={$items.bulletList.command}
            tooltip={m.bulletList()}
        >
            <div class="i-lucide-list block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.orderedList}
        <ToolbarButton
            pressed={$items.orderedList.isActive}
            disabled={!$items.orderedList.canExec}
            onClick={$items.orderedList.command}
            tooltip={m.orderedList()}
        >
            <div class="i-lucide-list-ordered block size-5"></div>
        </ToolbarButton>
    {/if}
    <!-- {#if $items.taskList}
        <ToolbarButton
            pressed={$items.taskList.isActive}
            disabled={!$items.taskList.canExec}
            onClick={$items.taskList.command}
            tooltip="Task List"
        >
            <div class="i-lucide-list-checks block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.toggleList}
        <ToolbarButton
            pressed={$items.toggleList.isActive}
            disabled={!$items.toggleList.canExec}
            onClick={$items.toggleList.command}
            tooltip="Toggle List"
        >
            <div class="i-lucide-list-collapse block size-5"></div>
        </ToolbarButton>
    {/if} -->
    <!-- {#if $items.indentList}
        <ToolbarButton
            pressed={$items.indentList.isActive}
            disabled={!$items.indentList.canExec}
            onClick={$items.indentList.command}
            tooltip="Increase indentation"
        >
            <div class="i-lucide-indent-increase block size-5"></div>
        </ToolbarButton>
    {/if}
    {#if $items.dedentList}
        <ToolbarButton
            pressed={$items.dedentList.isActive}
            disabled={!$items.dedentList.canExec}
            onClick={$items.dedentList.command}
            tooltip="Decrease indentation"
        >
            <div class="i-lucide-indent-decrease block size-5"></div>
        </ToolbarButton>
    {/if} -->
    {#if $items.insertImage}
        <ImageUploader disabled={!$items.insertImage.canExec} tooltip={m.uploadImage()}>
            <div class="i-lucide-image block size-5"></div>
        </ImageUploader>
    {/if}
</div>
