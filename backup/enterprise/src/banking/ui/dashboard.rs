use iced::{
    button, container, Column, Element, Length, Row, Text,
    Alignment, Color, Command,
};

pub struct EnterpriseDashboard {
    // State
    state: DashboardState,
    
    // UI Components
    account_summary: AccountSummaryWidget,
    transaction_list: TransactionListWidget,
    analytics_panel: AnalyticsPanelWidget,
    alerts_panel: AlertsPanelWidget,
    
    // Controls
    refresh_button: button::State,
    export_button: button::State,
}

impl EnterpriseDashboard {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .max_width(1200)
            .align_items(Alignment::Center)
            .push(self.header())
            .push(self.main_content())
            .push(self.footer())
            .into()
    }

    fn main_content(&mut self) -> Element<Message> {
        Row::new()
            .spacing(20)
            .push(self.account_summary.view())
            .push(self.transaction_list.view())
            .push(self.analytics_panel.view())
            .into()
    }
} 