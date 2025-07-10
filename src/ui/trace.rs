use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Styled,
    text::{Line, Text},
    widgets::{Block, List, Paragraph},
};

use crate::app::App;

pub fn render_trace_screen(app: &mut App, frame: &mut Frame, area: Rect) {
    let [tracepoint_area, call_stack_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Percentage(100)])
            .margin(1)
            .spacing(1)
            .areas(area);

    // Draw the box with the current tracepoint in it.
    let [tracepoint_area_inner] = Layout::vertical([Constraint::Percentage(100)])
        .margin(1)
        .areas(tracepoint_area);
    let label = Text::from(format!(
        "Current tracepoint: {}",
        app.trace()
            .tracepoint()
            .unwrap_or(&"<no tracepoint provided>".to_string())
    ));
    frame.render_widget(Block::bordered(), tracepoint_area);
    frame.render_widget(label, tracepoint_area_inner);

    // Render a "no call stack provided" message if the call stack is missing.
    if app.trace().call_stack().is_none() {
        frame.render_widget(
            Block::bordered()
                .title(" Call Stack ")
                .title_alignment(Alignment::Center),
            call_stack_area,
        );

        let text = Text::from("<no call stack provided>");
        let [center_horiz] = Layout::horizontal([Constraint::Length(text.width() as u16 + 2)])
            .flex(Flex::Center)
            .areas(call_stack_area);
        let [center_vert] = Layout::vertical([Constraint::Length(1)])
            .flex(Flex::Center)
            .margin(1)
            .areas(center_horiz);
        frame.render_widget(text, center_vert);
        return;
    }

    // Render the call stack with a list on the left and the source on the right.
    // SAFETY: Since we know the call stack exists at this point, it's safe to
    //         unwrap it from now on.
    let [list_area, call_site_area] =
        Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)])
            .spacing(1)
            .areas(call_stack_area);

    // This renders the list of stack frames.
    let n_frames = app.trace().call_stack().unwrap().frames.len();
    let frame_titles = (0..n_frames)
        .map(|i| format!("Frame #{i}"))
        .collect::<Vec<_>>();
    let list = List::new(frame_titles)
        .block(Block::bordered().title(" Call Stack "))
        .highlight_style(app.theme().highlighted_text);
    frame.render_stateful_widget(list, list_area, app.trace_mut().list_state().unwrap());

    // This divides up the source view area into the actual source view
    // and call site information.
    let [info_area, source_area] = Layout::vertical([Constraint::Length(5), Constraint::Fill(1)])
        .spacing(1)
        .areas(call_site_area);

    // Render the call site information.
    frame.render_widget(Block::bordered().title(" Call Site Info "), info_area);
    let [file_area, line_area, function_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .margin(1)
    .areas(info_area);
    {
        let idx = app.trace_mut().list_state().unwrap().selected().unwrap();
        let stack_frame = app.trace().call_stack().unwrap().frames.get(idx).unwrap();
        frame.render_widget(
            format!("File: {}", stack_frame.ctx.file.display()),
            file_area,
        );
        frame.render_widget(format!("Line: {}", stack_frame.ctx.line), line_area);
        frame.render_widget(
            format!("Function: {}", stack_frame.ctx.function),
            function_area,
        );
    }

    // This renders the source view for the highlighted stack frame.
    let [source_area_inner] = Layout::vertical([Constraint::Percentage(100)])
        .margin(1)
        .areas(source_area);
    frame.render_widget(Block::bordered().title(" Call Site "), source_area);
    let visible_lines = source_area_inner.height as usize;
    let idx = app.trace_mut().list_state().unwrap().selected().unwrap();
    let stack_frame = app.trace().call_stack().unwrap().frames.get(idx).unwrap();
    let line_number = stack_frame.ctx.line;
    let (lines, call_line) = if line_number < (visible_lines / 2) {
        (
            stack_frame.lines.as_ref().unwrap()[..visible_lines].to_vec(),
            line_number,
        )
    } else {
        let offset = line_number - (visible_lines / 2);
        let call_line = visible_lines / 2 + 1;
        let lines = stack_frame.lines.as_ref().unwrap()[offset..(offset + visible_lines)].to_vec();
        (lines, call_line)
    };
    let style = app.theme().highlighted_text;
    let lines = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            if i == call_line {
                Line::from(line.set_style(style))
            } else {
                Line::from(line)
            }
        })
        .collect::<Vec<_>>();
    let source = Paragraph::new(lines);
    frame.render_widget(source, source_area_inner);
}
