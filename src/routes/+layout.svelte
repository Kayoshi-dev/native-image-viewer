<script>
  import Lightbox from "$lib/components/Lightbox.svelte";
  import Searchbar from "$lib/components/Searchbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Database from "@tauri-apps/plugin-sql";
  import { onMount } from "svelte";
  import { db } from "$lib/stores/DatabaseStore";
  import "@fontsource/roboto";
  import "@fontsource/roboto/500.css";
  import "@fontsource/roboto/700.css";
  import "../app.css";

  onMount(async () => {
    // Init database connection on App start
    db.set(await Database.load("sqlite:test.db"));
    // create new table and insert a random row
    await $db?.execute(
      "CREATE TABLE IF NOT EXISTS image_metadata (id INTEGER PRIMARY KEY, name TEXT)"
    );
  });
</script>

<Sidebar />
<main class="dark:bg-gray-700 min-h-screen font-roboto">
  <Searchbar />
  <slot />
</main>

<Lightbox />
