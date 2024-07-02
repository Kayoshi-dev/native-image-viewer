export type Image = {
  imagePath: string;
  metadata: MediaMetadata;
};

export type MediaMetadata = {
  address: {
    city?: string;
    village?: string;
    country: string;
    state: string;
  };
  timestamp: number;
  device: string;
};
