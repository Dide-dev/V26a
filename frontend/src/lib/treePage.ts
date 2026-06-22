/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { duplicateTree } from '$lib/api-wrapper/trees';
import { appLink } from '$lib/index';

// Colors from TailwindCSS, aside from `tree`, which is a custom green color, defined in theme.css.
export type BranchColor = 'cyan' | 'tree' | 'yellow' | 'red' | 'pink' | 'violet';

export const branchColors: BranchColor[] = ['cyan', 'tree', 'yellow', 'red', 'pink', 'violet'];

export async function duplicateThisTree(id: number) {
    const newTreeId = await duplicateTree(id);
    window.open(appLink(`/trees/${newTreeId}/edit`));
}
