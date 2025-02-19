use pdf_writer::PdfWriter;
use csv::Writer;

pub struct ExportManager {
    pdf_generator: Arc<PdfGenerator>,
    csv_exporter: Arc<CsvExporter>,
    json_exporter: Arc<JsonExporter>,
    html_generator: Arc<HtmlGenerator>,
}

impl ExportManager {
    pub async fn export_report(
        &self,
        report: &AdvancedReport,
        format: ExportFormat,
        config: &ExportConfig,
    ) -> Result<ExportedReport, ExportError> {
        let exported = match format {
            ExportFormat::PDF => {
                self.generate_pdf_report(report, config).await?
            },
            ExportFormat::CSV => {
                self.export_csv_report(report, config).await?
            },
            ExportFormat::JSON => {
                self.export_json_report(report, config).await?
            },
            ExportFormat::HTML => {
                self.generate_html_report(report, config).await?
            },
            ExportFormat::Interactive => {
                self.generate_interactive_report(report, config).await?
            },
        };

        Ok(exported)
    }

    async fn generate_pdf_report(
        &self,
        report: &AdvancedReport,
        config: &ExportConfig,
    ) -> Result<ExportedReport, ExportError> {
        // 1. Create PDF Document
        let mut pdf = self.pdf_generator.create_document(config)?;
        
        // 2. Add Report Content
        self.pdf_generator.add_content(&mut pdf, report)?;
        
        // 3. Add Visualizations
        self.pdf_generator.add_visualizations(&mut pdf, &report.visualizations)?;
        
        // 4. Add Analytics
        self.pdf_generator.add_analytics(&mut pdf, &report.analytics)?;
        
        // 5. Finalize PDF
        let pdf_data = self.pdf_generator.finalize(pdf)?;

        Ok(ExportedReport {
            format: ExportFormat::PDF,
            data: pdf_data,
            metadata: self.generate_export_metadata(report),
        })
    }
} 