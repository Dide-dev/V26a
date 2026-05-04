/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import remarkHtml from 'remark-html';
import remarkParse from 'remark-parse';
import { unified } from 'unified';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';
import remarkRehype from 'remark-rehype';
import rehypeStringify from 'rehype-stringify';

export function htmlFromMarkdown(markdown: string): string {
    return unified()
        .use(remarkParse)
        .use(remarkMath)
        .use(remarkHtml)
        .use(remarkRehype)
        .use(rehypeKatex)
        .use(rehypeStringify)
        .processSync(markdown)
        .toString();
}
