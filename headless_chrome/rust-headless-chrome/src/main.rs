use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
use std::fs;

fn main() -> Result<(), failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_until_navigated()?;

    let jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot1.jpg", &jpeg_data)?;

    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?;
    let jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot2.jpg", &jpeg_data)?;

    tab.press_key("Enter")?;

    tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));

    let jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot3.jpg", &jpeg_data)?;

    Ok(())
}
