use std::{io, io::stdout};
use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Overview")]
    Overview,
    #[strum(to_string = "Attack")]
    Attack,
    #[strum(to_string = "Inventory")]
    Inventory,
    #[strum(to_string = "Description")]
    Description,
}

fn main() -> Result<()> {
    init_error_hooks()?;
    let mut terminal = init_terminal()?;
    App::default().run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('l') | Right => self.next_tab(),
                    Char('h') | Left  => self.previous_tab(),
                    Char('q') | Esc   => self.quit(),
                    Char('1')         => self.tab_n(0),
                    Char('2')         => self.tab_n(1),
                    Char('3')         => self.tab_n(2),
                    Char('4')         => self.tab_n(3),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn tab_n(&mut self, tab: u8) {
        if tab <= 3 {
            self.selected_tab = self.selected_tab.switch_to_tab_n(tab);
        }
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    fn previous(&self) -> Self {
        let current_index: usize = *self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(*self)
    }

    fn next(&self) -> Self {
        let current_index = *self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(*self)
    }

    fn switch_to_tab_n(&self, tab: u8) -> Self {
        Self::from_repr(tab.into()).unwrap_or(*self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, inner_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
    }
}

impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        "Dungeoneer".bold().render(area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(|tab| tab.title());
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            SelectedTab::Overview    => self.render_tab_overview(area, buf),
            SelectedTab::Attack      => self.render_tab_attack(area, buf),
            SelectedTab::Inventory   => self.render_tab_inventory(area, buf),
            SelectedTab::Description => self.render_tab_description(area, buf),
        }
    }
}

impl SelectedTab {
    fn title(&self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    // TODO: Add Widgets here in seperate files to clean it all up a little bit
    // https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html
    fn render_tab_overview(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Overview")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab_attack(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Attack")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab_inventory(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Inventory")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab_description(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Description")
            .block(self.block())
            .render(area, buf)
    }

    fn block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    fn palette(&self) -> tailwind::Palette {
        match self {
            SelectedTab::Overview    => tailwind::NEUTRAL,
            SelectedTab::Attack      => tailwind::NEUTRAL,
            SelectedTab::Inventory   => tailwind::NEUTRAL,
            SelectedTab::Description => tailwind::NEUTRAL,
        }
    }
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
