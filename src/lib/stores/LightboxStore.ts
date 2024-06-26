import { writable } from "svelte/store";

const isLightboxOpen = writable(false);
const currentImage = writable("");

const openLightbox = (image: string) => {
  isLightboxOpen.set(true);
  currentImage.set(image);
};

export { isLightboxOpen, openLightbox, currentImage };
