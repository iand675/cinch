//! The terminal frontend for [`crate::repl::Repl`].
//!
//! The session logic stays in `repl.rs`. This module owns the screen:
//! transcript, editor, completion popup, session sidebar, and key handling.

use crate::highlight;
use crate::repl::{Repl, ReplOutcome};
use crossterm::event::{
    self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::io::{self, Stdout};
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

const ACCENT: Color = Color::Rgb(240, 172, 120);
const GREEN: Color = Color::Rgb(206, 219, 163);
const MUTED: Color = Color::Rgb(163, 173, 146);
const TEXT: Color = Color::Rgb(230, 222, 200);
const PANEL: Color = Color::Rgb(44, 49, 37);
const BG: Color = Color::Rgb(27, 34, 24);
const RED: Color = Color::Rgb(240, 160, 128);
const YELLOW: Color = Color::Rgb(241, 206, 148);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Ready,
    Error,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EntryKind {
    Input,
    Output,
    Welcome,
}

struct Entry {
    kind: EntryKind,
    prompt: &'static str,
    text: String,
}

impl Entry {
    fn input(prompt: &'static str, text: String) -> Entry {
        Entry { kind: EntryKind::Input, prompt, text }
    }

    fn output(text: String) -> Entry {
        Entry { kind: EntryKind::Output, prompt: "", text }
    }

    fn welcome(text: String) -> Entry {
        Entry { kind: EntryKind::Welcome, prompt: "", text }
    }
}

#[derive(Default)]
struct Input {
    chars: Vec<char>,
    cursor: usize,
}

impl Input {
    fn text(&self) -> String {
        self.chars.iter().collect()
    }

    fn set(&mut self, text: String) {
        self.chars = text.chars().collect();
        self.cursor = self.chars.len();
    }

    fn clear(&mut self) {
        self.chars.clear();
        self.cursor = 0;
    }

    fn insert(&mut self, ch: char) {
        self.chars.insert(self.cursor, ch);
        self.cursor += 1;
    }

    fn insert_str(&mut self, text: &str) {
        for ch in text.chars() {
            self.insert(ch);
        }
    }

    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.chars.remove(self.cursor);
        }
    }

    fn delete(&mut self) {
        if self.cursor < self.chars.len() {
            self.chars.remove(self.cursor);
        }
    }

    fn left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.chars.len());
    }

    fn home(&mut self) {
        while self.cursor > 0 && self.chars[self.cursor - 1] != '\n' {
            self.cursor -= 1;
        }
    }

    fn end(&mut self) {
        while self.cursor < self.chars.len() && self.chars[self.cursor] != '\n' {
            self.cursor += 1;
        }
    }

    fn replace_range(&mut self, start: usize, end: usize, text: &str) {
        let replacement: Vec<char> = text.chars().collect();
        let width = replacement.len();
        self.chars.splice(start..end.min(self.chars.len()), replacement);
        self.cursor = start + width;
    }

    fn line_col(&self) -> (usize, usize) {
        let mut line = 0;
        let mut col = 0;
        for ch in self.chars.iter().take(self.cursor) {
            if *ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    fn word_start(&self) -> usize {
        let mut start = self.cursor;
        while start > 0 {
            let ch = self.chars[start - 1];
            if ch.is_alphanumeric() || "_-'.:".contains(ch) {
                start -= 1;
            } else {
                break;
            }
        }
        start
    }

    fn partial(&self) -> String {
        self.chars[self.word_start()..self.cursor].iter().collect()
    }
}

struct Completion {
    start: usize,
    candidates: Vec<String>,
    selected: usize,
}

struct App {
    repl: Repl,
    input: Input,
    continuation: bool,
    entries: Vec<Entry>,
    history: Vec<String>,
    history_index: usize,
    completion: Option<Completion>,
    bindings: Vec<(String, String)>,
    scroll: u16,
    auto_scroll: bool,
    status: Status,
    quit: bool,
    root: PathBuf,
}

impl App {
    fn new(root: PathBuf) -> App {
        let mut app = App {
            repl: Repl::with_root(&root),
            input: Input::default(),
            continuation: false,
            entries: Vec::new(),
            history: Vec::new(),
            history_index: 0,
            completion: None,
            bindings: Vec::new(),
            scroll: 0,
            auto_scroll: true,
            status: Status::Ready,
            quit: false,
            root,
        };
        app.entries.push(Entry::welcome(
            "Cinch interactive. Type an expression, or :help for commands.".to_string(),
        ));
        app.entries.push(Entry::welcome(format!(
            "Session root: {}",
            app.root.display()
        )));
        app
    }

    fn submit(&mut self) {
        let text = self.input.text();
        if text.trim().is_empty() {
            return;
        }
        let prompt = if self.continuation { "...> " } else { "cinch> " };
        self.entries.push(Entry::input(prompt, text.clone()));
        self.history.push(text.clone());
        self.history_index = self.history.len();
        let outcome = self.repl.eval(&text);
        self.input.clear();
        self.completion = None;
        self.auto_scroll = true;
        match outcome {
            ReplOutcome::Output(output) => {
                self.status = if starts_with_error(&output) {
                    Status::Error
                } else {
                    Status::Ready
                };
                self.remember_bindings(&output);
                if !output.is_empty() {
                    self.entries.push(Entry::output(output));
                }
                self.continuation = false;
            }
            ReplOutcome::Incomplete => {
                self.continuation = true;
                self.status = Status::Ready;
            }
            ReplOutcome::Quit => self.quit = true,
        }
    }

    fn remember_bindings(&mut self, output: &str) {
        for line in output.lines() {
            let line = line.trim_start();
            if let Some(rest) = line.strip_prefix("defined ") {
                let (name, scheme) = match rest.split_once(" :: ") {
                    Some((name, scheme)) => (name.trim(), scheme.trim()),
                    None => (rest.trim(), ""),
                };
                if let Some(existing) = self.bindings.iter_mut().find(|(item, _)| item == name) {
                    if !scheme.is_empty() {
                        existing.1 = scheme.to_string();
                    }
                } else if !name.is_empty() {
                    self.bindings.push((name.to_string(), scheme.to_string()));
                }
            } else if let Some(name) = line.strip_prefix("noted ") {
                let name = name.split_whitespace().next().unwrap_or("");
                if !name.is_empty() && !self.bindings.iter().any(|(item, _)| item == name) {
                    self.bindings.push((name.to_string(), "(pending)".to_string()));
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release {
            return;
        }
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let alt = key.modifiers.contains(KeyModifiers::ALT);
        match key.code {
            KeyCode::Enter if alt => self.input.insert('\n'),
            KeyCode::Enter => {
                if self.completion.is_some() {
                    self.accept_completion();
                } else {
                    self.submit();
                }
            }
            KeyCode::Char('j') if ctrl => self.input.insert('\n'),
            KeyCode::Char('c') if ctrl => {
                if self.input.text().is_empty() && !self.continuation {
                    self.quit = true;
                } else {
                    self.input.clear();
                    self.completion = None;
                    self.continuation = false;
                    self.repl.abort_pending();
                }
            }
            KeyCode::Char('d') if ctrl => self.quit = true,
            KeyCode::Char('l') if ctrl => {
                self.entries.clear();
                self.entries.push(Entry::welcome(
                    "Transcript cleared. :help for commands.".to_string(),
                ));
                self.auto_scroll = true;
            }
            KeyCode::Char('u') if ctrl => self.input.clear(),
            KeyCode::Char('a') if ctrl => self.input.home(),
            KeyCode::Char('e') if ctrl => self.input.end(),
            KeyCode::Char(ch) if !ctrl && !alt => {
                self.input.insert(ch);
                self.completion = None;
                self.history_index = self.history.len();
            }
            KeyCode::Backspace => {
                self.input.backspace();
                self.completion = None;
            }
            KeyCode::Delete => {
                self.input.delete();
                self.completion = None;
            }
            KeyCode::Left => {
                self.input.left();
                self.completion = None;
            }
            KeyCode::Right => {
                self.input.right();
                self.completion = None;
            }
            KeyCode::Home => {
                self.input.home();
                self.completion = None;
            }
            KeyCode::End => {
                self.input.end();
                self.completion = None;
            }
            KeyCode::Tab => self.complete(false),
            KeyCode::BackTab => self.complete(true),
            KeyCode::Up if self.completion.is_some() => self.cycle_completion(false),
            KeyCode::Down if self.completion.is_some() => self.cycle_completion(true),
            KeyCode::Up => self.history_previous(),
            KeyCode::Down => self.history_next(),
            KeyCode::Esc => self.completion = None,
            KeyCode::PageUp => self.scroll_up(8),
            KeyCode::PageDown => self.scroll_down(8),
            _ => {}
        }
    }

    fn history_previous(&mut self) {
        if self.history.is_empty() {
            return;
        }
        self.history_index = self.history_index.saturating_sub(1);
        self.input.set(self.history[self.history_index].clone());
        self.completion = None;
    }

    fn history_next(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            if self.history_index == self.history.len() {
                self.input.clear();
            } else {
                self.input.set(self.history[self.history_index].clone());
            }
            self.completion = None;
        }
    }

    fn scroll_up(&mut self, lines: u16) {
        self.auto_scroll = false;
        self.scroll = self.scroll.saturating_sub(lines);
    }

    fn scroll_down(&mut self, lines: u16) {
        self.scroll = self.scroll.saturating_add(lines);
        self.auto_scroll = false;
    }

    fn complete(&mut self, backwards: bool) {
        if let Some(completion) = &mut self.completion {
            let len = completion.candidates.len();
            if len > 0 {
                completion.selected = if backwards {
                    (completion.selected + len - 1) % len
                } else {
                    (completion.selected + 1) % len
                };
                self.apply_completion();
            }
            return;
        }
        let start = self.input.word_start();
        let partial = self.input.partial();
        let candidates = self.repl.complete(&partial);
        if candidates.is_empty() {
            self.completion = None;
            return;
        }
        if candidates.len() == 1 {
            self.input.replace_range(start, self.input.cursor, &candidates[0]);
            self.completion = None;
            return;
        }
        self.completion = Some(Completion { start, candidates, selected: 0 });
        self.apply_completion();
    }

    fn apply_completion(&mut self) {
        if let Some(completion) = &self.completion {
            if let Some(candidate) = completion.candidates.get(completion.selected).cloned() {
                self.input.replace_range(completion.start, self.input.cursor, &candidate);
            }
        }
    }

    fn cycle_completion(&mut self, forward: bool) {
        if let Some(completion) = &mut self.completion {
            let len = completion.candidates.len();
            if len > 0 {
                completion.selected = if forward {
                    (completion.selected + 1) % len
                } else {
                    (completion.selected + len - 1) % len
                };
                self.apply_completion();
            }
        }
    }

    fn accept_completion(&mut self) {
        self.apply_completion();
        self.completion = None;
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let completion_height = self
            .completion
            .as_ref()
            .map(|completion| completion.candidates.len().min(6) as u16 + 2)
            .unwrap_or(0);
        let input_lines = self.input.text().split('\n').count().max(1) as u16;
        let input_height = (input_lines + 2).clamp(3, 8);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(4),
                Constraint::Length(input_height),
                Constraint::Length(completion_height),
                Constraint::Length(1),
            ])
            .split(area);
        self.draw_header(frame, chunks[0]);
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(34)])
            .split(chunks[1]);
        self.draw_transcript(frame, body[0]);
        self.draw_sidebar(frame, body[1]);
        self.draw_input(frame, chunks[2]);
        if completion_height > 0 {
            self.draw_completion(frame, chunks[3]);
        }
        self.draw_status(frame, chunks[4]);
    }

    fn draw_header(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MUTED))
            .style(Style::default().bg(PANEL))
            .title(Span::styled(" Cinch ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(16), Constraint::Min(10), Constraint::Length(22)])
            .split(inner);
        let title = Line::from(vec![
            Span::styled("cinch", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" / interactive", Style::default().fg(MUTED)),
        ]);
        frame.render_widget(Paragraph::new(title).style(Style::default().bg(PANEL)), columns[0]);
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                self.root.display().to_string(),
                Style::default().fg(MUTED),
            )))
            .style(Style::default().bg(PANEL)),
            columns[1],
        );
        let (label, color) = match self.status {
            Status::Ready => ("ready", GREEN),
            Status::Error => ("error", RED),
        };
        let status = Line::from(vec![
            Span::styled("● ", Style::default().fg(color)),
            Span::styled(label, Style::default().fg(color)),
            Span::styled(
                format!("  {} defs", self.bindings.len()),
                Style::default().fg(MUTED),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(status).style(Style::default().bg(PANEL)),
            columns[2],
        );
    }

    fn transcript_lines(&self, width: u16) -> (Vec<Line<'static>>, u16) {
        let mut lines = Vec::new();
        let mut total = 0u16;
        for entry in &self.entries {
            match entry.kind {
                EntryKind::Input => {
                    for (index, line) in entry.text.lines().enumerate() {
                        let prompt = if index == 0 { entry.prompt } else { "   " };
                        let mut spans = vec![Span::styled(prompt, Style::default().fg(ACCENT))];
                        spans.extend(styled_line(line, None));
                        let line = Line::from(spans);
                        total += wrapped_height(&line, width);
                        lines.push(line);
                    }
                    lines.push(Line::from(""));
                    total += 1;
                }
                EntryKind::Output => {
                    for line in entry.text.lines() {
                        let style = if line.trim_start().starts_with("error")
                            || line.contains(": error:")
                        {
                            Style::default().fg(RED)
                        } else if line.trim_start().starts_with("defined")
                            || line.trim_start().starts_with("loaded")
                            || line.trim_start().starts_with("rechecked")
                        {
                            Style::default().fg(GREEN)
                        } else if line.trim_start().starts_with("noted") {
                            Style::default().fg(YELLOW)
                        } else {
                            Style::default().fg(TEXT)
                        };
                        let rendered = Line::from(Span::styled(line.to_string(), style));
                        total += wrapped_height(&rendered, width);
                        lines.push(rendered);
                    }
                    lines.push(Line::from(""));
                    total += 1;
                }
                EntryKind::Welcome => {
                    for line in entry.text.lines() {
                        let rendered = Line::from(Span::styled(
                            line.to_string(),
                            Style::default().fg(MUTED),
                        ));
                        total += wrapped_height(&rendered, width);
                        lines.push(rendered);
                    }
                    lines.push(Line::from(""));
                    total += 1;
                }
            }
        }
        (lines, total)
    }

    fn draw_transcript(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MUTED))
            .style(Style::default().bg(BG))
            .title(Span::styled(" Transcript ", Style::default().fg(MUTED)));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let width = inner.width.max(1);
        let (lines, total) = self.transcript_lines(width);
        let viewport = inner.height.max(1);
        let max_scroll = total.saturating_sub(viewport);
        if self.auto_scroll {
            self.scroll = max_scroll;
        } else {
            self.scroll = self.scroll.min(max_scroll);
        }
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::default().bg(BG).fg(TEXT))
            .wrap(Wrap { trim: false })
            .scroll((self.scroll, 0));
        frame.render_widget(paragraph, inner);
    }

    fn draw_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MUTED))
            .style(Style::default().bg(BG))
            .title(Span::styled(" Session ", Style::default().fg(MUTED)));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let mut lines = Vec::new();
        if self.bindings.is_empty() {
            lines.push(Line::from(Span::styled(
                "No definitions yet.",
                Style::default().fg(MUTED),
            )));
        } else {
            for (name, scheme) in &self.bindings {
                let mut spans = vec![Span::styled(
                    name.clone(),
                    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                )];
                if !scheme.is_empty() {
                    spans.push(Span::styled(
                        format!(" :: {scheme}"),
                        Style::default().fg(MUTED),
                    ));
                }
                lines.push(Line::from(spans));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Commands",
            Style::default().fg(MUTED).add_modifier(Modifier::BOLD),
        )));
        for command in [
            ":type expr",
            ":kind Type",
            ":browse [module]",
            ":load file",
            ":reload",
            ":history",
            ":help",
            ":quit",
        ] {
            lines.push(Line::from(Span::styled(
                format!("  {command}"),
                Style::default().fg(TEXT),
            )));
        }
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::default().bg(BG))
            .wrap(Wrap { trim: false });
        frame.render_widget(paragraph, inner);
    }

    fn draw_input(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ACCENT))
            .style(Style::default().bg(BG))
            .title(Span::styled(" Input ", Style::default().fg(ACCENT)));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let (cursor_line, cursor_col) = self.input.line_col();
        let mut lines = Vec::new();
        let text = self.input.text();
        let text_lines: Vec<&str> = text.split('\n').collect();
        for (index, line) in text_lines.iter().enumerate() {
            let prompt = if index == 0 {
                if self.continuation { "...> " } else { "cinch> " }
            } else {
                "   "
            };
            let mut spans = vec![Span::styled(prompt, Style::default().fg(ACCENT))];
            let cursor = if index == cursor_line { Some(cursor_col) } else { None };
            spans.extend(styled_line(line, cursor));
            lines.push(Line::from(spans));
        }
        let paragraph = Paragraph::new(Text::from(lines))
            .style(Style::default().bg(BG).fg(TEXT))
            .wrap(Wrap { trim: false });
        frame.render_widget(paragraph, inner);
    }

    fn draw_completion(&self, frame: &mut Frame<'_>, area: Rect) {
        let Some(completion) = &self.completion else { return };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MUTED))
            .style(Style::default().bg(BG))
            .title(Span::styled(" Complete ", Style::default().fg(MUTED)));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let start = completion.selected.saturating_sub(5);
        let lines = completion
            .candidates
            .iter()
            .enumerate()
            .skip(start)
            .take(6)
            .map(|(index, candidate)| {
                let style = if index == completion.selected {
                    Style::default().fg(BG).bg(ACCENT)
                } else {
                    Style::default().fg(TEXT)
                };
                Line::from(Span::styled(format!(" {candidate} "), style))
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(Text::from(lines)).style(Style::default().bg(BG)), inner);
    }

    fn draw_status(&self, frame: &mut Frame<'_>, area: Rect) {
        let hints = " Enter run · Tab complete · ↑↓ history · Ctrl+J newline · Ctrl+C clear · Ctrl+D quit ";
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(hints, Style::default().fg(MUTED)),
                Span::raw(" "),
            ]))
            .style(Style::default().bg(PANEL)),
            area,
        );
    }
}

fn starts_with_error(output: &str) -> bool {
    output.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with("error") || line.contains(": error:")
    })
}

fn wrapped_height(line: &Line<'_>, width: u16) -> u16 {
    let width = width.max(1) as usize;
    let cells = line.width();
    if cells == 0 {
        1
    } else {
        ((cells + width - 1) / width) as u16
    }
}

fn token_style(class: &str) -> Style {
    let color = match class {
        "comment" => MUTED,
        "keyword" => ACCENT,
        "type" | "constructor" | "attribute" => YELLOW,
        "callable" => GREEN,
        "string" | "char" | "symbol" => GREEN,
        "number" => YELLOW,
        "operator" => Color::Rgb(196, 196, 167),
        _ => TEXT,
    };
    Style::default().fg(color)
}

fn styled_line(line: &str, cursor: Option<usize>) -> Vec<Span<'static>> {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return cursor
            .map(|_| vec![Span::styled(" ", Style::default().add_modifier(Modifier::REVERSED))])
            .unwrap_or_default();
    }
    let mut styles = vec![Style::default().fg(TEXT); chars.len()];
    for (start, end, class) in highlight::token_spans(line) {
        let start = line[..start].chars().count();
        let end = line[..end].chars().count();
        for style in styles.iter_mut().take(end).skip(start) {
            *style = token_style(class);
        }
    }
    if let Some(cursor) = cursor {
        if cursor < styles.len() {
            styles[cursor] = styles[cursor].add_modifier(Modifier::REVERSED);
        } else {
            styles.push(Style::default().add_modifier(Modifier::REVERSED));
        }
    }
    let mut spans = Vec::new();
    let mut run = String::new();
    let mut current = styles.first().copied().unwrap_or_default();
    for (ch, style) in chars.iter().zip(styles.iter()) {
        if *style != current && !run.is_empty() {
            spans.push(Span::styled(std::mem::take(&mut run), current));
            current = *style;
        }
        run.push(*ch);
    }
    if !run.is_empty() {
        spans.push(Span::styled(run, current));
    }
    if cursor.is_some_and(|cursor| cursor >= chars.len()) {
        let style = styles.last().copied().unwrap_or_default();
        spans.push(Span::styled(" ", style.add_modifier(Modifier::REVERSED)));
    }
    spans
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, root: PathBuf) -> io::Result<()> {
    let mut app = App::new(root);
    loop {
        terminal.draw(|frame| app.draw(frame))?;
        if !event::poll(Duration::from_millis(250))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) => app.handle_key(key),
            Event::Paste(text) => {
                app.input.insert_str(&text);
                app.completion = None;
                app.history_index = app.history.len();
            }
            Event::Resize(_, _) => {}
            _ => {}
        }
        if app.quit {
            return Ok(());
        }
    }
}

/// Run the interactive terminal REPL. The caller supplies a TTY.
pub fn run() -> io::Result<()> {
    let root = std::env::current_dir().unwrap_or_default();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    if let Err(error) = execute!(stdout, EnterAlternateScreen, EnableBracketedPaste) {
        let _ = disable_raw_mode();
        return Err(error);
    }
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(error) => {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableBracketedPaste);
            return Err(error);
        }
    };
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| run_app(&mut terminal, root)));
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableBracketedPaste
    );
    let _ = terminal.show_cursor();
    match result {
        Ok(result) => result,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    fn rendered(app: &mut App) -> String {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| app.draw(frame)).unwrap();
        let buffer = terminal.backend().buffer();
        let mut out = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                out.push_str(buffer[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }

    #[test]
    fn draws_header_transcript_and_editor() {
        let mut app = App::new(std::env::temp_dir());
        let screen = rendered(&mut app);
        assert!(screen.contains("Cinch"), "{screen}");
        assert!(screen.contains("Transcript"), "{screen}");
        assert!(screen.contains("cinch>"), "{screen}");
        assert!(screen.contains("Session"), "{screen}");
    }

    #[test]
    fn tab_completes_a_command() {
        let mut app = App::new(std::env::temp_dir());
        app.input.set(":ty".into());
        app.complete(false);
        assert_eq!(app.input.text(), ":type");
    }

    #[test]
    fn submit_updates_transcript_and_bindings() {
        let mut app = App::new(std::env::temp_dir());
        app.input.set("x :: Int".into());
        app.submit();
        app.input.set("x = 5".into());
        app.submit();
        app.input.set("x".into());
        app.submit();
        let text: String = app
            .entries
            .iter()
            .map(|entry| entry.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(text.contains("defined x :: Int"), "{text}");
        assert!(text.contains("5 :: Int"), "{text}");
        assert!(app.bindings.iter().any(|(name, scheme)| name == "x" && scheme == "Int"));
    }
}
