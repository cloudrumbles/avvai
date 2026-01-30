use pdfium_render::prelude::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("/usr/lib/"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let pdf_path = "/home/shah/workspace/avvai/tamilvu_pdf/cikaram/Cikaram_Sem_1.pdf";
    let document = pdfium.load_pdf_from_file(pdf_path, None)?;

    println!("PDF loaded: {} pages", document.pages().len());

    // Page 10 is index 9 (0-indexed)
    let page_index = 9;
    let page = document.pages().get(page_index)?;

    println!("Processing page {} (index {})", page_index + 1, page_index);
    println!("Page size: {:?} x {:?}", page.width(), page.height());

    let mut image_count = 0;
    let output_dir = Path::new("/home/shah/workspace/avvai/tamilvu_pdf/cikaram/extracted");
    std::fs::create_dir_all(output_dir)?;

    for (obj_idx, object) in page.objects().iter().enumerate() {
        if let Some(image_object) = object.as_image_object() {
            image_count += 1;
            println!(
                "Found image object {} at index {}, bounds: {:?}",
                image_count,
                obj_idx,
                object.bounds()
            );

            if let Ok(image) = image_object.get_raw_image() {
                let filename = output_dir.join(format!("page10_image_{}.png", image_count));
                image.save(&filename)?;
                println!("Saved: {}", filename.display());
            } else {
                println!("Could not extract raw image for object {}", image_count);
            }
        }
    }

    println!("\nTotal images found on page 10: {}", image_count);

    Ok(())
}
