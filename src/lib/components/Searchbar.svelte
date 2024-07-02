<script>
  import { Search } from "lucide-svelte";
</script>

<form class="pt-2 max-w-md mx-auto">
  <label
    for="default-search"
    class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white"
    >Search</label
  >
  <div class="relative">
    <input
      type="search"
      id="default-search"
      class="block w-full p-2 ps-6 text-sm text-gray-900 rounded-lg bg-gray-100 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
      placeholder="Search for images..."
      required
    />
    <button
      type="submit"
      class="text-gray-800 absolute end-2.5 bottom-0.5 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
    >
      <Search size={20} />
    </button>
  </div>
</form>
