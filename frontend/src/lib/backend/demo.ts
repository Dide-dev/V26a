import { api_call_get, api_call_post } from './index';

type JsonTestStruct = {
	a: string;
	/** Current value of the global counter */
	b: number;
};

export async function get_json_test(): Promise<JsonTestStruct> {
	return api_call_get('demo/json');
}

/** Increases the counter by 1 */
export async function counter_increment() {
	api_call_get('demo/increment');
}

export type AddCounterBody = {
	add: number;
};

/** Increases the counter by arbitrary amount */
export async function counter_add(body: AddCounterBody) {
	api_call_post('demo/add-to-counter', body);
}
