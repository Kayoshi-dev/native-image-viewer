import { FaceDetector, FilesetResolver } from "@mediapipe/tasks-vision";
import { invoke } from "@tauri-apps/api/core";

let faceDetector: FaceDetector;
let runningMode = "IMAGE" as const;

export const initializefaceDetector = async () => {
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

export async function displayImageDetections(event: Event) {
  const myImage = event.target as HTMLImageElement;

  const img = new Image();
  img.src = myImage.src;
  img.crossOrigin = "anonymous";

  img.onload = async () => {
    const [detection] = faceDetector.detect(img).detections;

    console.log(
      "Confidence: " +
        Math.round(parseFloat(detection.categories[0].score.toString()) * 100) +
        "%."
    );
  };
}
