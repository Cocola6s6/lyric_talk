use anyhow::{Result, Context};
use printpdf::*;
use std::fs::{File, self};
use std::path::Path;
use std::io::BufWriter;

pub struct Download;

impl Download {
    // 将歌词内容下载为 PDF 文件
    pub fn download_lyrics_as_pdf(lyrics: &str, filename: &str) -> Result<()> {
        // 创建文件来保存 PDF
        let file = File::create(filename).context("Failed to create file")?;

        // 创建 PDF 文档
        let (mut pdf_document, page1, layer1) = PdfDocument::new("Lyrics PDF", Mm(210.0), Mm(297.0), "Layer 1");

        // 获取第一页的层
        let first_layer = pdf_document.get_page(page1).get_layer(layer1);

        // 设置中文字体，使用自定义字体文件（需替换为你自己的 .ttf 字体文件）
        let font = pdf_document.add_external_font(File::open("wryh.ttf")?)?;

        // 设置字体大小
        let font_size = 12.0;

        // 设置起始 y 位置
        let mut y_position = 280.0;

        // 将歌词分行显示
        for line in lyrics.lines() {
            if y_position < 20.0 {
                // 如果页面已满，添加新页面
                let (new_page, new_layer) = pdf_document.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                let new_layer = pdf_document.get_page(new_page).get_layer(new_layer);
                new_layer.use_text("New Page", font_size, Mm(10.0), Mm(290.0), &font);
                y_position = 280.0;  // 重置 y 位置
            }

            // 将每行歌词添加到 PDF 页面
            first_layer.use_text(line, font_size, Mm(10.0), Mm(y_position), &font);

            // 下一行的位置
            y_position -= font_size * 1.5;
        }

        // 保存 PDF 文件
        let mut file = BufWriter::new(file);
        pdf_document.save(&mut file).context("Failed to save PDF")?;
        println!("PDF saved to {}", filename);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use std::path::Path;

    #[test]
    fn test_download_lyrics_as_pdf() -> Result<()> {
        let lyrics = "这是中文歌词的示例。\n这是第二行歌词。\n这是第三行歌词。";
        let filename = "lyrics.pdf"; // 保存的 PDF 文件路径

        Download::download_lyrics_as_pdf(lyrics, filename)?; // 调用生成 PDF

        // 验证生成的 PDF 是否存在
        let path = Path::new(filename);
        assert!(path.exists(), "PDF file should exist");

        // 进一步验证文件是否不为空
        let metadata = fs::metadata(path).context("Failed to retrieve file metadata")?;
        assert!(metadata.len() > 0, "PDF file should not be empty");

        Ok(())
    }
}
