use crate::hash_file::hash_image_content;
use exif::{In, Tag};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImageMetadata {
    hash: String,
    location: Option<String>,
    device: Option<String>,
}

// Use once_cell to create a static HTTP client, so we don't have to create a new one every time
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("native-image-builder-shicho")
        .build()
        .expect("Failed to create HTTP client")
});

// Convert something like "37 deg 48' 0.00\" N" to decimal degrees
pub fn convert_to_decimal(coordinate: String) -> Result<f64, &'static str> {
    let parts: Vec<&str> = coordinate.split_whitespace().collect();
    if parts.len() != 6 {
        return Err("Invalid input format");
    }

    let degrees = parts[0].parse::<f64>().map_err(|_| "Invalid degrees")?;
    let minutes = parts[2].parse::<f64>().map_err(|_| "Invalid minutes")?;
    let seconds = parts[4].parse::<f64>().map_err(|_| "Invalid seconds")?;

    Ok(degrees + (minutes / 60.0) + (seconds / 3600.0))
}

// Reverse geocode the latitude and longitude to get the location
pub async fn reverse_geocode(latitude: f64, longitude: f64) -> Result<String, reqwest::Error> {
    println!("Doing reverse geocoding for {}, {}", latitude, longitude);
    // Need to improve this as it can send too many requests.
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}",
        latitude, longitude
    );

    let response = HTTP_CLIENT.get(url).send().await?.text().await?;

    println!("Response: {:?}", response);

    Ok(response.to_string())
}

#[tauri::command]
pub async fn get_metadata(image_path: String) -> Result<ImageMetadata, tauri::Error> {
    println!("Image path: {:?}", image_path);

    let file = std::fs::File::open(image_path.clone())?;
    let mut bufreader = std::io::BufReader::new(&file);

    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut bufreader)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

    let latitude = exif
        .get_field(Tag::GPSLatitude, In::PRIMARY)
        .map(|field| field.display_value().to_string())
        .map(|coordinate| convert_to_decimal(coordinate));

    let longitude = exif
        .get_field(Tag::GPSLongitude, In::PRIMARY)
        .map(|field| field.display_value().to_string())
        .map(|coordinate| convert_to_decimal(coordinate));

    let hashed_image = hash_image_content(image_path).unwrap_or_default();
    println!("Hashed image: {:?}", hashed_image);

    let mut location: Option<String> = None;

    if latitude.is_some() || longitude.is_some() {
        match (latitude, longitude) {
            (Some(Ok(lat)), Some(Ok(lon))) => {
                location = reverse_geocode(lat, lon).await.ok();
            }
            _ => {
                println!("Invalid latitude or longitude")
            }
        }
    } else {
        println!("No GPS data found in image")
    }

    let device = Some(
        exif.get_field(Tag::Model, In::PRIMARY)
            .map(|field| field.display_value().to_string())
            .unwrap_or("Unknown".to_string()),
    );

    Ok(ImageMetadata {
        hash: hashed_image,
        location,
        device,
    })
}
