<script lang="ts">
  import { page } from "$app/stores";
  import { dirPaths, getSlug } from "$lib/stores/DirectoryStore";
  import { X, Folder, FolderSearch } from "lucide-svelte";
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { basename } from "@tauri-apps/api/path";

  // Improve that part of the code later
  let activeSlug = "";
  $: {
    $page.url.hash;
    activeSlug = getSlug();
  }

  onMount(() => {
    const pathsList = localStorage.getItem("paths");

    // We set the value on the store so it can load the paths
    if (pathsList) {
      const paths = JSON.parse(pathsList);
      dirPaths.set(paths);
    }
  });

  const deletePath = (path: dirPaths) => {
    return () => {
      const paths = $dirPaths.filter((dirPath) => dirPath.name !== path.name);
      dirPaths.set(paths);
      localStorage.setItem("paths", JSON.stringify(paths));
    };
  };

  const nameSlug = (name: string) => name.toLowerCase().replace(/\s/g, "-");

  const openDirectoryPicker = async () => {
    const directory = (await open({
      directory: true,
    })) as string;

    const dirName = await basename(directory);
    const dirSlug = nameSlug(dirName);

    dirPaths.update((paths) => {
      if (!paths.find((path) => path.path === directory)) {
        paths.push({
          name: dirName,
          slug: dirSlug,
          path: directory,
        });

        localStorage.setItem("paths", JSON.stringify(paths));
      }
      return paths;
    });

    // Redirect to the new folder
  };
</script>

<button
  data-drawer-target="default-sidebar"
  data-drawer-toggle="default-sidebar"
  aria-controls="default-sidebar"
  type="button"
  class="inline-flex items-center p-2 mt-2 ms-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
>
  <span class="sr-only">Open sidebar</span>
  <svg
    class="w-6 h-6"
    aria-hidden="true"
    fill="currentColor"
    viewBox="0 0 20 20"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      clip-rule="evenodd"
      fill-rule="evenodd"
      d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"
    ></path>
  </svg>
</button>

<aside
  id="default-sidebar"
  class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
  aria-label="Sidebar"
>
  <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
    <ul class="space-y-2 font-medium">
      <li>
        <button
          on:click={openDirectoryPicker}
          class="w-full flex items-center justify-center group p-1 border-2 rounded-md border-dashed transition-all ease-in-out duration-300 border-gray-300 hover:border-blue-500 text-gray-500 hover:text-blue-700 dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          <span class="flex items-center">
            <FolderSearch size={16} />
            <span class="ms-3">Add a new folder</span>
          </span>
        </button>
      </li>
      {#each $dirPaths as dirPath}
        <li>
          <a
            href={`#${dirPath.slug}`}
            class="flex items-center justify-between group p-1 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 {dirPath.slug ===
            activeSlug
              ? 'bg-gray-200 dark:bg-gray-700'
              : ''}"
          >
            <span class="flex items-center">
              <Folder size={16} />
              <span class="ms-3">{dirPath.name}</span>
            </span>

            <button
              on:click|stopPropagation={deletePath(dirPath)}
              class="opacity-0 group-hover:opacity-100 hover:bg-gray-300 rounded-full p-1"
            >
              <X size={16} />
            </button>
          </a>
        </li>
      {/each}
    </ul>
  </div>
</aside>
