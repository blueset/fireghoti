use crate::{database::cache, util::http_client};
use image::{io::Reader, ImageError, ImageFormat};
use isahc::AsyncReadResponseExt;
use nom_exif::{parse_jpeg_exif, EntryValue, ExifTag};
use std::io::Cursor;
use tokio::sync::Mutex;

#[macros::errors]
pub enum Error {
    #[error("Redis cache operation has failed")]
    Cache(#[from] cache::Error),
    #[error("failed to acquire an HTTP client")]
    HttpClient(#[from] http_client::Error),
    #[error("HTTP request failed")]
    Isahc(#[from] isahc::Error),
    #[doc = "bad HTTP status"]
    #[error("bad HTTP status ({0})")]
    BadStatus(String),
    #[error("failed to decode an image")]
    Image(#[from] ImageError),
    #[error("failed to decode an image")]
    Io(#[from] std::io::Error),
    #[error("failed to extract the exif data")]
    Exif(#[from] nom_exif::Error),
    #[doc = "too many fetch attempts"]
    #[error("too many fetch attempts for {0}")]
    TooManyAttempts(String),
    #[doc = "unsupported image type"]
    #[error("unsupported image type ({0})")]
    UnsupportedImage(String),
}

const BROWSER_SAFE_IMAGE_TYPES: [ImageFormat; 8] = [
    ImageFormat::Png,
    ImageFormat::Jpeg,
    ImageFormat::Gif,
    ImageFormat::WebP,
    ImageFormat::Tiff,
    ImageFormat::Bmp,
    ImageFormat::Ico,
    ImageFormat::Avif,
];

static MTX_GUARD: Mutex<()> = Mutex::const_new(());

#[cfg_attr(test, derive(Debug, PartialEq))]
#[macros::export(object)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

#[macros::export]
pub async fn get_image_size_from_url(url: &str) -> Result<ImageSize, Error> {
    let attempted: bool;

    {
        let _ = MTX_GUARD.lock().await;

        attempted = cache::get_one::<bool>(cache::Category::FetchUrl, url)
            .await?
            .is_some();

        if !attempted {
            cache::set_one(cache::Category::FetchUrl, url, &true, 10 * 60).await?;
        }
    }

    if attempted {
        tracing::warn!("attempt limit exceeded: {}", url);
        return Err(Error::TooManyAttempts(url.to_owned()));
    }

    tracing::info!("retrieving image from {}", url);

    let mut response = http_client::client()?.get_async(url).await?;

    if !response.status().is_success() {
        tracing::info!("status: {}", response.status());
        tracing::debug!("response body: {:#?}", response.body());
        return Err(Error::BadStatus(format!(
            "{} returned {}",
            url,
            response.status()
        )));
    }

    let image_bytes = response.bytes().await?;

    let reader = Reader::new(Cursor::new(&image_bytes)).with_guessed_format()?;

    let format = reader.format();
    if format.is_none() || !BROWSER_SAFE_IMAGE_TYPES.contains(&format.unwrap()) {
        return Err(Error::UnsupportedImage(format!("{:?}", format)));
    }

    let size = reader.into_dimensions()?;

    let res = ImageSize {
        width: size.0,
        height: size.1,
    };

    if format.unwrap() != ImageFormat::Jpeg {
        return Ok(res);
    }

    // handle jpeg orientation
    // https://magnushoff.com/articles/jpeg-orientation/

    let exif = parse_jpeg_exif(&*image_bytes)?;
    if exif.is_none() {
        return Ok(res);
    }

    let orientation = exif.unwrap().get_value(&ExifTag::Orientation)?;
    let rotated =
        orientation.is_some() && matches!(orientation.unwrap(), EntryValue::U32(v) if v >= 5);

    if !rotated {
        return Ok(res);
    }

    Ok(ImageSize {
        width: size.1,
        height: size.0,
    })
}

#[cfg(test)]
mod unit_test {
    use super::ImageSize;
    use crate::database::cache;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    #[cfg_attr(miri, ignore)] // can't call foreign function `getaddrinfo` on OS `linux`
    async fn get_image_size_from_url() {
        let png_url_1 = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/splash.png";
        let png_url_2 = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/notification-badges/at.png";
        let png_url_3 = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/api-doc.png";
        let rotated_jpeg_url = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/test/resources/rotate.jpg";
        let webp_url_1 = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/custom/assets/badges/error.webp";
        let webp_url_2 = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/screenshots/1.webp";
        let ico_url = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/favicon.ico";
        let gif_url = "https://firefish.dev/firefish/firefish/-/raw/b9c3dfbd3d473cb2cee20c467eeae780bc401271/packages/backend/test/resources/anime.gif";
        let mp3_url = "https://firefish.dev/firefish/firefish/-/blob/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/sounds/aisha/1.mp3";

        // delete caches in case you run this test multiple times
        cache::delete_all(cache::Category::FetchUrl).await.unwrap();

        let png_size_1 = ImageSize {
            width: 1024,
            height: 1024,
        };
        let png_size_2 = ImageSize {
            width: 96,
            height: 96,
        };
        let png_size_3 = ImageSize {
            width: 1024,
            height: 354,
        };
        let rotated_jpeg_size = ImageSize {
            width: 256,
            height: 512,
        };
        let webp_size_1 = ImageSize {
            width: 256,
            height: 256,
        };
        let webp_size_2 = ImageSize {
            width: 1080,
            height: 2340,
        };
        let ico_size = ImageSize {
            width: 256,
            height: 256,
        };
        let gif_size = ImageSize {
            width: 256,
            height: 256,
        };

        assert_eq!(
            png_size_1,
            super::get_image_size_from_url(png_url_1).await.unwrap()
        );
        assert_eq!(
            png_size_2,
            super::get_image_size_from_url(png_url_2).await.unwrap()
        );
        assert_eq!(
            png_size_3,
            super::get_image_size_from_url(png_url_3).await.unwrap()
        );
        assert_eq!(
            rotated_jpeg_size,
            super::get_image_size_from_url(rotated_jpeg_url)
                .await
                .unwrap()
        );
        assert_eq!(
            webp_size_1,
            super::get_image_size_from_url(webp_url_1).await.unwrap()
        );
        assert_eq!(
            webp_size_2,
            super::get_image_size_from_url(webp_url_2).await.unwrap()
        );
        assert_eq!(
            ico_size,
            super::get_image_size_from_url(ico_url).await.unwrap()
        );
        assert_eq!(
            gif_size,
            super::get_image_size_from_url(gif_url).await.unwrap()
        );
        assert!(super::get_image_size_from_url(mp3_url).await.is_err());
    }

    #[tokio::test]
    #[cfg_attr(miri, ignore)] // can't call foreign function `getaddrinfo` on OS `linux`
    async fn too_many_attempts() {
        let url = "https://firefish.dev/firefish/firefish/-/raw/5891a90f71a8b9d5ea99c683ade7e485c685d642/packages/backend/assets/splash.png";

        // delete caches in case you run this test multiple times
        cache::delete_one(cache::Category::FetchUrl, url)
            .await
            .unwrap();

        assert!(super::get_image_size_from_url(url).await.is_ok());
        assert!(super::get_image_size_from_url(url).await.is_err());
    }
}
