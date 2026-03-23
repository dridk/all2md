use crate::error::All2mdError;
use crate::strategy::FormatParser;

pub struct PdfParser;

impl FormatParser for PdfParser {
    fn to_markdown(&self, data: &[u8]) -> Result<String, All2mdError> {
        let mut doc = pdf_oxide::PdfDocument::from_bytes(data.to_vec())
            .map_err(|e| All2mdError::ParseError(format!("PDF: {e}")))?;

        let page_count = doc
            .page_count()
            .map_err(|e| All2mdError::ParseError(format!("PDF page count: {e}")))?;
        let mut md = String::new();

        for i in 0..page_count {
            let text = doc
                .extract_text(i)
                .map_err(|e| All2mdError::ParseError(format!("PDF page {i}: {e}")))?;
            md.push_str(&text);
            if i + 1 < page_count {
                md.push_str("\n\n---\n\n");
            }
        }
        Ok(md)
    }
}
