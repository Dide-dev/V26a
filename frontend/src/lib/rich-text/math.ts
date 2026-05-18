import type { Element, Text, Root } from 'hast';
import type { State } from 'hast-util-to-mdast';
import { render } from 'katex';
import type { Nodes, RootContent } from 'mdast';
import { defineMath } from 'prosekit/extensions/math';
import { visitParents } from 'unist-util-visit-parents';

export function defineKatexMath() {
    function renderKaTeXMathBlock(text: string, element: HTMLElement) {
        render(text, element, { displayMode: true, throwOnError: false, output: 'html' });
    }
    function renderKaTeXMathInline(text: string, element: HTMLElement) {
        render(text, element, { displayMode: false, throwOnError: false, output: 'html' });
    }

    return defineMath({
        renderMathBlock: renderKaTeXMathBlock,
        renderMathInline: renderKaTeXMathInline
    });
}

/**
 * The following function is for converting from the 'remarkMath' html math variant
 * to the math variatn of prosekit
 *
 * The difference is:
 * `<div class="prosemirror-math-block"><pre><code>x^2</code></pre></div>`
 * vs
 * `<pre><code class="language-math">x^2</code></pre>`
 *
 * and
 *
 * `<span class="prosemirror-math-inline"><code>x^2</code></span>`
 * vs
 * `<code class="language-math">x^2</code>`
 *
 *
 * Due to rehypeSanitize the classes 'math-display' and 'math-line' from remarkMath are gone.
 * But we can still decypher what variant it is based on if the parent is a <pre>.
 */
export function rehypeMathToProsekit() {
    // There is no need for options here, it is custom build to do one thing.

    // 'tree' is of type `Root', file is of type `VFile`
    return function (tree: Root) {
        visitParents(tree, 'element', function (element, parents) {
            const classes = Array.isArray(element.properties.className)
                ? element.properties.className
                : [];

            if (!classes.includes('language-math')) {
                return;
            }

            let parent = parents[parents.length - 1];

            let prosemirrorClassname = 'prosemirror-math-inline';
            let newTagName = 'span';
            if ('tagName' in parent && parent.tagName === 'pre') {
                element = parent;
                parents = parents.slice(0, -1);
                parent = parents[parents.length - 1];

                prosemirrorClassname = 'prosemirror-math-block';
                newTagName = 'div';
            }

            const result: Element = {
                type: 'element',
                tagName: newTagName,
                properties: {
                    className: [prosemirrorClassname]
                },
                children: [element]
            };
            const index = parent.children.indexOf(element);
            parent.children.splice(index, 1, result);
            return 'skip';
        });
    };
}

/** Will translate a rehype 'span' element to a remark node, and handles inline math correctly. */
export function rehypeRemarkSpanHandler(
    state: State,
    node: Element
): RootContent[] | { type: 'inlineMath'; value: string } {
    const classes = node.properties.className as string[];

    if (!classes.includes('prosemirror-math-inline')) {
        // This is the default implementation for a 'span' found in hast-util-to-mdast
        return state.all(node);
    }

    const codeElement = node.children[0] as Element;

    // It is possible that the text in the inline math is undefined deleting its content.
    if (codeElement.children.length === 0) {
        return [];
    }

    return { type: 'inlineMath', value: (codeElement.children[0] as Text).value };
}

/** Will translate a rehype 'div' element to a remark node, and handles math blocks correctly. */
export function rehypeRemarkDivHandler(
    state: State,
    node: Element
): Nodes[] | { type: 'math'; value: string } {
    const classes = node.properties.className as string[] | undefined;

    if (classes === undefined || !classes.includes('prosemirror-math-block')) {
        // This is the default implementation for a 'div' found in hast-util-to-mdast
        return state.toFlow(state.all(node));
    }
    const codeElement = (node.children[0] as Element).children[0] as Element;

    // It is possible that the text in the code block is undefined when initially creating it.
    let value = '';
    if (codeElement.children.length > 0) {
        value = (codeElement.children[0] as Text).value;
    }

    return { type: 'math', value };
}
