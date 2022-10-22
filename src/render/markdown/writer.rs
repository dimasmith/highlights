use std::io::Write;

pub struct MarkdownWriter<W> {
    writer: W,
}

impl<W> MarkdownWriter<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        MarkdownWriter { writer }
    }

    pub fn heading(&mut self, title: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("# {}\n", title))
    }

    pub fn blockquote(&mut self, quote: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("> {}\n", quote))
    }

    pub fn text(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("{}\n", text))
    }

    pub fn italic(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("*{}*\n", text))
    }

    pub fn link(&mut self, title: &str, url: &str) -> std::io::Result<()> {
        self.writer
            .write_fmt(format_args!("[{}]({})\n", title, url))
    }

    pub fn line(&mut self) -> std::io::Result<()> {
        self.writer.write_all("---\n".as_bytes())
    }

    pub fn lf(&mut self) -> std::io::Result<()> {
        self.writer.write_all("\n".as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn render_heading() {
        let mut buf = BufWriter::new(Vec::new());
        let mut md = MarkdownWriter::new(&mut buf);

        md.heading("Book Title").unwrap();

        let markdown = stringify(buf);
        assert_eq!(markdown, "# Book Title\n");
    }

    #[test]
    fn render_blockquote() {
        let mut buf = BufWriter::new(Vec::new());
        let mut md = MarkdownWriter::new(&mut buf);

        md.blockquote("This is rather nice quote I want to highlight")
            .unwrap();

        let markdown = stringify(buf);
        assert_eq!(
            markdown,
            "> This is rather nice quote I want to highlight\n"
        );
    }

    #[test]
    fn render_text() {
        let mut buf = BufWriter::new(Vec::new());
        let mut md = MarkdownWriter::new(&mut buf);

        md.text("Just a plain text").unwrap();

        let markdown = stringify(buf);
        assert_eq!(markdown, "Just a plain text\n");
    }

    #[test]
    fn render_line() {
        let mut buf = BufWriter::new(Vec::new());
        let mut md = MarkdownWriter::new(&mut buf);

        md.line().unwrap();

        let markdown = stringify(buf);
        assert_eq!(markdown, "---\n");
    }

    #[test]
    fn render_lf() {
        let mut buf = BufWriter::new(Vec::new());
        let mut md = MarkdownWriter::new(&mut buf);

        md.lf().unwrap();

        let markdown = stringify(buf);
        assert_eq!(markdown, "\n");
    }

    fn stringify(buffer: BufWriter<Vec<u8>>) -> String {
        let bytes = buffer.into_inner().unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
