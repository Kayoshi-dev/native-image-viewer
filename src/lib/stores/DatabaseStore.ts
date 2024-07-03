import Database from "@tauri-apps/plugin-sql";
import { writable } from "svelte/store";

const db = writable<Database | null>(null);

// It's already done automatically, but it would be a nice thing to do the DB queries on the Rust side, need to find a way to get the DB pool.
const findDb = async () => {};

export { db };
