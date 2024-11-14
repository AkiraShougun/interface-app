import { writable } from "svelte/store";
export const activeState = writable<string | null>(null);