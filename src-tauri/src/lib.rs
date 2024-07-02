use exif::{In, Tag};
use once_cell::sync::Lazy;
use reqwest::Client;

// Use once_cell to create a static HTTP client, so we don't have to create a new one every time
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("native-image-builder-shicho")
        .build()
        .expect("Failed to create HTTP client")
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![get_metadata])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Convert something like "37 deg 48' 0.00\" N" to decimal degrees
fn convert_to_decimal(latitude: &str) -> Result<f64, &'static str> {
    let parts: Vec<&str> = latitude.split_whitespace().collect();
    if parts.len() != 6 {
        return Err("Invalid input format");
    }

    let degrees = parts[0].parse::<f64>().map_err(|_| "Invalid degrees")?;
    let minutes = parts[2].parse::<f64>().map_err(|_| "Invalid minutes")?;
    let seconds = parts[4].parse::<f64>().map_err(|_| "Invalid seconds")?;

    Ok(degrees + (minutes / 60.0) + (seconds / 3600.0))
}

// Reverse geocode the latitude and longitude to get the location
async fn reverse_geocode(latitude: f64, longitude: f64) -> Result<String, reqwest::Error> {
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
async fn get_metadata(image_path: String) -> Result<String, tauri::Error> {
    println!("Image path: {:?}", image_path);

    let file = std::fs::File::open(image_path)?;
    let mut bufreader = std::io::BufReader::new(&file);

    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut bufreader)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

    let latitude = convert_to_decimal(
        &exif
            .get_field(Tag::GPSLatitude, In::PRIMARY)
            .map(|field| field.display_value().to_string())
            .unwrap_or_default(),
    )
    .unwrap_or(0.0);

    let longitude = convert_to_decimal(
        &exif
            .get_field(Tag::GPSLongitude, In::PRIMARY)
            .map(|field| field.display_value().to_string())
            .unwrap_or_default(),
    )
    .unwrap_or(0.0);

    println!("Latitude: {:?}", latitude);
    println!("Longitude: {:?}", longitude);

    if latitude != 0.0 && longitude != 0.0 {
        let location = reverse_geocode(latitude, longitude)
            .await
            .unwrap_or_default();
        return Ok(location);
    }

    return Ok("No data".to_string());
}
