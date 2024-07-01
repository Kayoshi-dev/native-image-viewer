<script lang="ts">
  import { isLightboxOpen, currentImage } from "$lib/stores/LightboxStore";
  import {
    displayImageDetections,
    initializefaceDetector,
  } from "$lib/utils/faceDetection";
  import { onMount } from "svelte";
  import { ArrowLeft, ArrowRight } from "lucide-svelte";

  onMount(async () => {
    await initializefaceDetector();
  });

  const handleClick = (event: MouseEvent) => {
    if (event.target === event.currentTarget) {
      $isLightboxOpen = false;
    } else {
      console.log("clicked on image");
    }
  };

  const handleEscape = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      $isLightboxOpen = false;
    }
  };
</script>

<svelte:window on:keydown={handleEscape} />

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
{#if $isLightboxOpen && $currentImage}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <dialog
    open={$isLightboxOpen}
    class="fixed inset-0 w-full h-screen bg-slate-900 bg-opacity-75 z-50 flex justify-center items-center"
    on:click={handleClick}
  >
    <button class="fixed top-1/2 left-0">
      <ArrowLeft />
    </button>
    <img
      class="max-w-4/5 max-h-full object-contain"
      src={$currentImage}
      alt="truc"
      on:click={displayImageDetections}
    />
  </dialog>
  <button class="fixed top-1/2 right-0 text-white">
    <ArrowRight />
  </button>
{/if}
