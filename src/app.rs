use crate::scp::ScpManager;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub enum AppMode {
    Normal,
    Input,
    Loading,
}

pub struct App {
    pub running: bool,
    pub mode: AppMode,
    pub scp_manager: ScpManager,
    pub current_scp_number: Option<i32>,
    pub content: String,
    pub scroll: u16,
    pub input_buffer: String,
    pub error_msg: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            running: true,
            mode: AppMode::Normal,
            scp_manager: ScpManager::new()?,
            current_scp_number: None,
            content: "Welcome to SCP Reader.\nPress 'i' or '/' to enter an SCP number.\nPress 'r' for a random SCP.\nPress 'q' to quit.".to_string(),
            scroll: 0,
            input_buffer: String::new(),
            error_msg: None,
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                self.handle_input(key)?;
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        match self.mode {
            AppMode::Normal => match key.code {
                KeyCode::Char('q') => self.running = false,
                KeyCode::Char('i') | KeyCode::Char('/') => {
                    self.mode = AppMode::Input;
                    self.input_buffer.clear();
                }
                KeyCode::Char('r') => self.load_random_scp(),
                KeyCode::Char('j') | KeyCode::Down => self.scroll_down(1),
                KeyCode::Char('k') | KeyCode::Up => self.scroll_up(1),
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.scroll_down(10)
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.scroll_up(10)
                }
                KeyCode::Char('g') => self.scroll = 0, // Simplified 'gg'
                KeyCode::Char('G') => self.scroll = self.max_scroll(),
                _ => {}
            },
            AppMode::Input => match key.code {
                KeyCode::Esc => self.mode = AppMode::Normal,
                KeyCode::Enter => {
                    if let Ok(num) = self.input_buffer.parse::<i32>() {
                        self.load_scp(num);
                    } else {
                        self.error_msg = Some("Invalid number".to_string());
                    }
                    self.mode = AppMode::Normal;
                }
                KeyCode::Char(c) => {
                    if c.is_digit(10) {
                        self.input_buffer.push(c);
                    }
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            },
            AppMode::Loading => {} // Block input while loading if we were async, but we are blocking currently
        }
        Ok(())
    }

    fn load_scp(&mut self, number: i32) {
        self.mode = AppMode::Loading;
        self.error_msg = None;
        // In a real TUI we'd want this async or in a thread to keep UI responsive
        // For now, blocking is acceptable as per requirement scope.
        match self.scp_manager.get_scp(number) {
            Ok(text) => {
                self.content = text;
                self.current_scp_number = Some(number);
                self.scroll = 0;
            }
            Err(e) => {
                self.error_msg = Some(e.to_string());
            }
        }
        self.mode = AppMode::Normal;
    }

    fn load_random_scp(&mut self) {
        self.mode = AppMode::Loading;
        match self.scp_manager.get_random_scp() {
            Ok((num, text)) => {
                self.current_scp_number = Some(num);
                self.content = text;
                self.scroll = 0;
            }
            Err(e) => {
                self.error_msg = Some(e.to_string());
            }
        }
        self.mode = AppMode::Normal;
    }

    fn scroll_down(&mut self, amount: u16) {
        let max_scroll = self.max_scroll();
        self.scroll = (self.scroll.saturating_add(amount)).min(max_scroll);
    }

    fn scroll_up(&mut self, amount: u16) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    fn max_scroll(&self) -> u16 {
        let line_count = self.content.lines().count() as u16;
        line_count.saturating_sub(1)
    }
}
