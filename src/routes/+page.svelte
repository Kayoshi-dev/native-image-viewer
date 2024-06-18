<script lang="ts">
  import { onMount, tick } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { readDir } from "@tauri-apps/plugin-fs";
  import { path } from "@tauri-apps/api";
  import { dirPaths } from "$lib/stores/DirectoryStore";
  import { FaceDetector, FilesetResolver } from "@mediapipe/tasks-vision";
  import { page } from "$app/stores";

  const nameSlug = (name: string) => name.toLowerCase().replace(/\s/g, "-");

  let images: string[] = [];

  let faceDetector: FaceDetector;
  let runningMode = "IMAGE" as const;

  const initializefaceDetector = async () => {
    const vision = await FilesetResolver.forVisionTasks(
      "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.0/wasm"
    );
    faceDetector = await FaceDetector.createFromOptions(vision, {
      baseOptions: {
        modelAssetPath: `https://storage.googleapis.com/mediapipe-models/face_detector/blaze_face_short_range/float16/1/blaze_face_short_range.tflite`,
        delegate: "GPU",
      },
      runningMode,
    });
  };

  onMount(async () => {
    initializefaceDetector();
  });

  $: {
    const dirName = $dirPaths.find(
      (path) => path.slug === $page.url.hash.substring(1)
    );
    console.log(dirName);
    if (dirName) {
      loadFolder(dirName.path);
    }
  }

  const loadFolder = async (dir: string) => {
    console.log("dir", dir);
    const fileList = await readDir(dir);

    // Get only images, this could be png, jpg, jpeg, etc.
    const imagesList = fileList.filter((file) =>
      file.name.match(/\.(jpe?g|png)$/i)
    );

    const imagesListPaths = await Promise.all(
      imagesList.map(async (file) => {
        const filePath = await path.join(dir, file.name);
        return filePath;
      })
    );

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

  async function displayImageDetections(event: Event) {
    const myImage = event.target as HTMLImageElement;

    const img = new Image();
    img.src = myImage.src;
    img.crossOrigin = "anonymous";

    img.onload = async () => {
      const [detection] = faceDetector.detect(img).detections;

      console.log(
        "Confidence: " +
          Math.round(
            parseFloat(detection.categories[0].score.toString()) * 100
          ) +
          "%."
      );
    };
  }
</script>

<div class="p-4 sm:ml-64">
  <h1 class="text-2xl font-semibold text-gray-900 dark:text-white mb-3">
    Recent images
  </h1>
  {$page.url}
  <div
    class="p-4 border-2 border-gray-200 border-dashed rounded-lg dark:border-gray-700"
  >
    <div class="flex flex-wrap items-center gap-3">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
      {#each images as image}
        <!-- <enhanced:img src={convertFileSrc(image)} alt="some alt text" /> -->
        <img
          class="rounded-md cursor-pointer w-40 h-40 object-cover"
          alt="My dynamically loaded img"
          src={convertFileSrc(image)}
          on:click={(e) => displayImageDetections(e)}
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
