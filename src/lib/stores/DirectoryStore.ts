import { writable } from "svelte/store";
import { page } from "$app/stores";
import { get } from "svelte/store";

type dirPaths = {
  name: string;
  slug: string;
  path: string;
};

let dirPaths = writable<dirPaths[]>([]);

// Get the slug of the current URL
const getSlug = () => {
  return get(page).url.hash.substring(1);
};

export { dirPaths, getSlug };
