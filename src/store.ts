import { writable } from "svelte/store";
//activate state for buttons
export const activeState = writable<string | null>(null);