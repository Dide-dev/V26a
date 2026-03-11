import type { PageServerLoad } from './$types';
import {error } from '@sveltejs/kit';
import {apiCallGetTree} from '$lib/backend/trees';

export const load: PageServerLoad = async ({params}) => {
	if(!params.id){
		throw error(400, 'Which tree?');
	}

	try{
		const tree = await apiCallGetTree(params.id); // WELK BOSPAD? id = tree id?

		if (!tree){
			throw error(404, 'Tree not found, has it been chopped?');
		}
		return {
			tree
		};
	} catch(err){
		throw error(500, 'Failed to load tree');
	}
	
};