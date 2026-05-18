/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import type { State as HastToMdastState } from 'hast-util-to-mdast';
import { type State as MdastToHastState, defaultHandlers } from 'mdast-util-to-hast';
import type { Element as HastElement } from 'hast';
import type { Text } from 'mdast';

/**
 * Extends the default remark image to rehype, by adding support for image sizes.
 * The formatting follows the hedgedoc format, so like:
 * `![Image](/icon.png =100x50)`
 */
export function rehypeRemarkImgHandler(
    state: HastToMdastState,
    node: HastElement
): { type: 'html'; value: string } {
    const url = node.properties['src']! as string;

    const width = (node.properties['width'] ?? undefined) as string | undefined;
    const height = (node.properties['height'] ?? undefined) as string | undefined;

    let sizeExtension = '';
    if (width !== undefined || height !== undefined) {
        sizeExtension += ` =${width}x${height}`;
    }

    // TODO? alt text
    const value = `![](${url}${sizeExtension})`;

    // We use the 'html' since no further processing will need to be done on this element.
    return { type: 'html', value };
}

/**
 * Translate a remmark 'text' element to a rehype element,
 * will parse it as an image if the syntax for it is present.
 */
export function remarkRehypeTextHandler(state: MdastToHastState, node: Text) {
    const txt = node.value.trim();
    // TODO? alt text
    const start = '![](';
    const end = ')';

    if (!txt.startsWith(start) || !txt.endsWith(end)) {
        return defaultHandlers.text(state, node);
    }

    const middle = txt.slice(start.length, -end.length);
    const spaceParts = middle.split(' ');

    const src = spaceParts[0];

    // We don't need to handle the case where the width and height are not specified,
    // as in that case the remark parser has already turned it into an image.
    // Just in case the user would input something invalid, we will treat the input as text on any error.
    if (spaceParts.length != 2 || !spaceParts[1].startsWith('=')) {
        return defaultHandlers.text(state, node);
    }

    const xParts = spaceParts[1].slice(1).split('x');
    if (xParts.length != 2) {
        return defaultHandlers.text(state, node);
    }

    const width = xParts[0];
    const height = xParts[1];

    const element: HastElement = {
        type: 'element',
        tagName: 'img',
        properties: { src, width, height },
        children: []
    };

    return element;
}
