/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { defineNodeSpec, Priority, union, withPriority } from 'prosekit/core';
import { defineSvelteNodeView, type SvelteNodeViewComponent } from 'prosekit/svelte';
import ImageView from './ImageView.svelte';

export type ImageAttrs = {
    src?: string | null;
    width?: number | null;
    height?: number | null;
};

export function defineImageViewerParserExtension() {
    const nodeSpec = defineNodeSpec({
        name: 'image',
        attrs: {
            src: { default: null, validate: 'string|null' },
            width: { default: null, validate: 'number|null' },
            height: { default: null, validate: 'number|null' }
        },
        group: 'block',
        defining: true,
        draggable: true,
        parseDOM: [
            {
                tag: 'img[src]',
                getAttrs: (element): ImageAttrs => {
                    if (typeof element === 'string') {
                        return { src: null };
                    }

                    const src = element.getAttribute('src') || null;

                    const parseNum = (value: string | null) =>
                        value === null ? null : Number(value);

                    const width: number | null = parseNum(element.getAttribute('width'));
                    const height: number | null = parseNum(element.getAttribute('height'));

                    return { src, width, height };
                }
            }
        ],
        toDOM(node) {
            const attrs = node.attrs as ImageAttrs;
            return ['img', attrs];
        }
    });

    const nodeView = defineSvelteNodeView({
        name: 'image',
        component: ImageView as SvelteNodeViewComponent
    });

    return union(nodeSpec, nodeView);
}
