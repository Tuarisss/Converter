use image::ImageReader;

pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {


    let img = ImageReader::open(input)?.decode()?;
    img.save(output)?;


    
    Ok(())
}