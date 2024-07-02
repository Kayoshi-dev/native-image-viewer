import type { Image } from "$lib/types/ImageType";
import { writable } from "svelte/store";

const isLightboxOpen = writable(false);
const currentImage = writable<Image>();

const openLightbox = (image: Image) => {
  isLightboxOpen.set(true);
  currentImage.set(image);
};

export { isLightboxOpen, openLightbox, currentImage };
