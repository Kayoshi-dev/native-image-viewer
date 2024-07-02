<script>
  import Lightbox from "$lib/components/Lightbox.svelte";
  import Searchbar from "$lib/components/Searchbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import "../app.css";
</script>

<Sidebar />
<main class="dark:bg-gray-700 min-h-screen">
  <Searchbar />
  <slot />
</main>

<Lightbox />
