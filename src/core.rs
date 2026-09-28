// 
use crate::cli::MediaType;


pub fn convert(
    kind: &MediaType,
    output: &str,
    input: &str,
) ->  Result<(), Box<dyn std::error::Error>> {
// Определяем куда направить конвертацию
    match kind {
        MediaType::Photo => {
            crate::photo::convert(input, output)?; // Если photo то вызвать photo.rs и передать в переменную вход и выход
        }

        MediaType::Video => {
            crate::video::convert(input, output)?;
        }
    }
    Ok(())

}