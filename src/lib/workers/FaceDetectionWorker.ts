import * as faceapi from "face-api.js";

self.onmessage = async (event) => {
  const { myFirstImage, mySecondImage } = event.data;

  if (true) {
    console.log("hello from worker");

    await faceapi.nets.ssdMobilenetv1.loadFromUri("/models");
    await faceapi.nets.tinyFaceDetector.loadFromUri("/models");
    await faceapi.nets.faceLandmark68Net.loadFromUri("/models");
    await faceapi.nets.faceRecognitionNet.loadFromUri("/models");
    await faceapi.nets.faceExpressionNet.loadFromUri("/models");

    const firstImageResult = await faceapi
      .detectSingleFace(myFirstImage, new faceapi.TinyFaceDetectorOptions())
      .withFaceLandmarks()
      .withFaceDescriptor();

    const secondImageResult = await faceapi
      .detectSingleFace(mySecondImage, new faceapi.TinyFaceDetectorOptions())
      .withFaceLandmarks()
      .withFaceDescriptor();

    if (firstImageResult && secondImageResult) {
      const distance = faceapi.euclideanDistance(
        firstImageResult.descriptor,
        secondImageResult.descriptor
      );

      self.postMessage({ firstImageResult, secondImageResult, distance });
    }
  }
};
