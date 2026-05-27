/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { nodeFromJSON, setSelectionAround, type NodeJSON } from 'prosekit/core';
import { Fragment, Slice } from 'prosekit/pm/model';
import { TextSelection, type Command } from 'prosekit/pm/state';
import { dropPoint } from 'prosekit/pm/transform';

// This is modified from the `insertNode` of prosekit
export function insertAndSelectInsideNode(options: NodeJSON): Command {
    return (state, dispatch) => {
        const node = nodeFromJSON(options, { schema: state.schema });

        const insertPos = dropPoint(
            state.doc,
            state.selection.anchor,
            new Slice(Fragment.from([node]), 0, 0)
        );
        if (insertPos == null) return false;

        if (dispatch) {
            const tr = state.tr.insert(insertPos, node);
            setSelectionAround(tr, insertPos);
            const start = tr.doc.resolve(insertPos + 1);
            const end = tr.doc.resolve(insertPos + node.nodeSize - 1);
            const selection = TextSelection.between(start, end);
            tr.setSelection(selection);
            dispatch(tr);
        }
        return true;
    };
}
