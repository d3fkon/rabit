use chrono::{Datelike, Utc};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Cell, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use super::App;

const TABLE_WIDTH: u16 = 39;
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Split the terminal area into major chunks such that the UI is always centered Calculate the length and the width of the tracker contents to ensure the content is centered
pub fn split_area<B>(f: &mut Frame<B>, habit_count: &u16) -> Rect
where
    B: Backend,
{
    let total_height = f.size().height;
    let required_height = habit_count + 9;
    let empty_v_space = total_height - required_height;

    let total_width = f.size().width;
    let required_width = TABLE_WIDTH;
    let empty_h_space = total_width - TABLE_WIDTH;

    let h_constraints = [
        Constraint::Length(empty_h_space / 2),
        Constraint::Length(required_width),
        Constraint::Length(empty_h_space / 2),
    ];
    let v_constraints = [
        Constraint::Length(empty_v_space / 2),
        Constraint::Length(required_height),
        Constraint::Length(empty_v_space / 2),
    ];
    let sub = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(h_constraints.clone().as_ref())
        .split(f.size());
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(v_constraints.clone().as_ref())
        .split(sub[1]);
    return main[1];
}

/// The main UI function to draw the table
pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let habit_count = app.tracker.habits.len() as u16; // Habit count
    let layout = split_area(f, &habit_count); // Main Layout
    let bg_block = Block::default()
        .title("My Habits")
        .style(Style::default().fg(Color::White))
        .title_alignment(Alignment::Center);
    f.render_widget(bg_block, layout); // Render the title

    // The main chunk at the center of the grid
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(app.tracker.habits.len() as u16 + 3), // Main Table
                Constraint::Length(1),                                   // Command Bar
                Constraint::Length(1),                                   // Help Bar
            ]
            .as_ref(),
        )
        .horizontal_margin(4)
        .vertical_margin(2)
        .split(layout);

    // Rename
    let (top_chunk, cl_chunk, help_chunk) = (main_chunk[0], main_chunk[1], main_chunk[2]);

    let inner_table_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(habit_count)])
        .split(top_chunk);

    let (heading_chunk, table_chunk) = (inner_table_chunk[0], inner_table_chunk[1]);

    // Current month for tracking
    let month = MONTHS[app.tracker.start_date.month() as usize - 1].to_owned();

    let title = Paragraph::new(Text::from(month)).alignment(Alignment::Left);
    f.render_widget(title, heading_chunk);

    let table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Max(3 * 7)].as_ref())
        .split(table_chunk);
    let (label_chunk, values_chunk) = (table_chunks[0], table_chunks[1]);

    // Cell Styles
    let cell_normal_style = Style::default().fg(Color::White);
    let cell_selected_style = Style::default().fg(Color::Magenta).bg(Color::White);
    let cell_disabled_style = Style::default().bg(Color::DarkGray);
    let header_labels = app.tracker.get_header_labels();
    let header_cells = header_labels.iter().map(|h| {
        return Cell::from(h.to_owned()).style(Style::default().fg(Color::LightMagenta));
    });
    let header = Row::new(header_cells).height(1);
    let column_constraint = Constraint::Length(3);
    let column_width = &[column_constraint; 7];

    // All the tracked data
    let values = app.tracker.values_v2();
    let value_rows = values.iter().enumerate().map(|(i, row)| {
        let cells = row.iter().enumerate().map(|(j, stat)| {
            let (a, b) = match app.state.selected() {
                Some((x, y)) => (x, y),
                None => (0, 0),
            };

            let mut cell_style = cell_normal_style;
            let now = Utc::now().day().to_string();
            if (i, j) == (a, b) {
                if header_labels[j] == now {
                    cell_style = cell_selected_style
                } else {
                    cell_style = cell_disabled_style
                }
            } else if header_labels[j] == now {
                cell_style = cell_disabled_style
            }

            let text = match &*stat {
                None => String::from(" ◦ "),
                // Some(s) => format!(" {} ", s).to_string()
                Some(s) => match s.as_str() {
                    "true" => String::from(" • "),
                    s => format!(" {} ", s).to_string(),
                },
            };

            let fg_color = match &*stat {
                None => Color::Red,
                Some(_) => Color::DarkGray,
            };

            Cell::from(text).style(cell_style.fg(fg_color))
        });
        Row::new(cells)
    });

    // The table with the boolean values
    let values_table = Table::new(value_rows)
        .header(header)
        .widths(column_width)
        .column_spacing(0);

    f.render_widget(values_table, values_chunk);

    // Table for the name of the habit

    let labels = app.tracker.labels();

    // Was using tables before lists
    // let habit_rows = labels.iter().map(|h| {
    //     let cell = Cell::from(h.as_str()).style(Style::default().fg(Color::LightMagenta));
    //     Row::new([cell])
    // });
    // let habit_lables_table = Table::new(habit_rows)
    //     .header(Row::new([Cell::from("Habits")]))
    //     .widths([Constraint::Length(10)].as_ref());

    let mut habit_list_items: Vec<ListItem> = labels
        .iter()
        .enumerate()
        .map(move |(i, habit)| {
            ListItem::new(Text::from(
                [i.to_string().as_str(), habit.as_str()].join(" "),
            ))
        })
        .collect();
    habit_list_items.insert(0, ListItem::new(Text::from(" ")));
    let habit_list = List::new(habit_list_items).style(Style::default().fg(Color::LightMagenta));

    f.render_widget(habit_list, label_chunk);

    // -----

    let command_bg = Block::default().style(Style::default().bg(Color::DarkGray));

    let command = Paragraph::new(Text::from([":".to_owned(), app.input.to_owned()].join(" ")))
        .alignment(Alignment::Left)
        .block(command_bg);
    f.render_widget(command, cl_chunk);

    let mode = match app.mode {
        super::AppMode::NORMAL => "NORMAL Mode",
        super::AppMode::COMMAND => "COMMAND Mode",
        super::AppMode::HABIT => "HABIT mode",
    };

    let text = Paragraph::new(Text::from(
        [mode.to_owned(), "'q' to quit".to_owned()].join(" | "),
    ))
    .alignment(Alignment::Center);
    f.render_widget(text, help_chunk)
}
