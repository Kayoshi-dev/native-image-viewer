<script>
  import Searchbar from "$lib/components/Searchbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import "../app.css";
</script>

<Sidebar />
<main class="mt-2">
  <Searchbar />
  <slot />
</main>
