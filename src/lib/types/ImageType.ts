export type Image = {
  imagePath: string;
  metadata: MediaMetadata;
};

export type Address = {
  city?: string;
  village?: string;
  country: string;
  state: string;
};

export type MediaMetadata = {
  hash: string; // Act as an ID
  location: Address;
  timestamp: number;
  device: string;
};
