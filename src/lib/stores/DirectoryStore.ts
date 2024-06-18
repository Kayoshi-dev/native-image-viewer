import { writable } from "svelte/store";

type dirPaths = {
  name: string;
  slug: string;
  path: string;
};

let dirPaths = writable<dirPaths[]>([]);

export { dirPaths };
