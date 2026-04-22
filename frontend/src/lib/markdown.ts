/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import remarkHtml from 'remark-html';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import { unified } from 'unified';

export function htmlFromMarkdown(markdown: string): string {
    return unified()
        .use(remarkParse)
        .use(remarkGfm)
        .use(remarkHtml)
        .processSync(markdown)
        .toString();
}
