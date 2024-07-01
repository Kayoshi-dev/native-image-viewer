<script lang="ts">
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { readDir } from "@tauri-apps/plugin-fs";
  import { path } from "@tauri-apps/api";
  import { dirPaths } from "$lib/stores/DirectoryStore";
  import { page } from "$app/stores";
  import { openLightbox } from "$lib/stores/LightboxStore";

  const nameSlug = (name: string) => name.toLowerCase().replace(/\s/g, "-");

  let images: string[] = [];

  $: {
    const dirName = $dirPaths.find(
      (path) => path.slug === $page.url.hash.substring(1)
    );

    if (dirName) {
      loadFolder(dirName.path);
    }
  }

  const loadFolder = async (dir: string) => {
    const fileList = await readDir(dir);

    // Get only images, this could be png, jpg, jpeg, etc.
    const imagesList = fileList.filter((file) =>
      file.name.match(/\.(jpe?g|png|webp)$/i)
    );

    const imagesListPaths = await Promise.all(
      imagesList.map(async (file) => {
        const filePath = await path.join(dir, file.name);
        return filePath;
      })
    );

    imagesListPaths.forEach(async (imagePath) => {
      const metadata = (await invoke("get_metadata", {
        imagePath,
      })) as MediaMetadata;

      console.log(metadata);
    });

    images = [...imagesListPaths];
  };

  const openDirectoryPicker = async () => {
    const dirName = (await open({
      directory: true,
    })) as string;

    dirPaths.update((paths) => {
      if (!paths.find((path) => path.path === dirName)) {
        paths.push({
          name: dirName.split("/").pop() as string,
          slug: nameSlug(dirName.split("/").pop() as string),
          path: dirName,
        });

        localStorage.setItem("paths", JSON.stringify(paths));
      }
      return paths;
    });

    // Redirect to the new folder
  };
</script>

<div class="p-4 sm:ml-64">
  <h1 class="text-2xl font-semibold text-gray-900 dark:text-white mb-3">
    Recent images
  </h1>
  <div
    class="p-4 border-2 border-gray-200 border-dashed rounded-lg dark:border-gray-700"
  >
    <div class="flex flex-wrap items-center gap-3">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
      {#each images as image}
        {@const imgSrc = convertFileSrc(image)}

        <img
          class="rounded-md cursor-pointer w-40 h-40 object-cover"
          alt="My dynamically loaded img"
          src={imgSrc}
          on:click={() => openLightbox(imgSrc)}
        />
      {/each}
    </div>

    <button
      type="button"
      class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
      on:click={openDirectoryPicker}
    >
      Select a folder
    </button>
  </div>
</div>
