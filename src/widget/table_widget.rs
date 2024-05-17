use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Clipboard, Layout, Shell, Widget,
    },
    alignment::{Horizontal, Vertical},
    mouse::Cursor, Element,Length, Size
};

use crate::widget::table_widget_style::{Theme, Appearance, StyleSheet,};

pub struct Table<'a, Message, Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
    Theme: StyleSheet,
{
    data: Option<Vec<Element<'a, Message, Renderer>>>,
    headers: Vec<Element<'a, Message, Renderer>>,
    style: <Theme as StyleSheet>::Style,
    hovered_row: Option<usize>,
    hovered_column: Option<usize>,
    column_widths: Vec<Length>,
}

impl<'a, Message, Theme, Renderer> Table<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: renderer::Renderer,
    Theme: StyleSheet
{
    pub fn new(data: Vec<Vec<Element<'a, Message, Renderer>>>, headers: Vec<Element<'a, Message, Renderer>>) -> Self {
        let column_widths = vec![Length::Fill; headers.len()];
        Self {
            data: None,
            headers,
            style: <Theme as StyleSheet>::Style::default(), // Use the default table style
            hovered_row: None,
            hovered_column: None,
            column_widths,
        }
    }

    pub fn style(mut self, style: <Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> 
    for Table<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        let mut children = Vec::new();
        children.extend(self.headers.iter().map(Tree::new));
        for row in &self.data {
            children.extend(row.iter().map(Tree::new));
        }
        children
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(cell) = self.data.as_ref() {
            tree.diff_children(&[&self.headers, &self.data])
        } else {
            tree.diff_children(&[&self.headers])        
        }
//        let mut iterator = tree.children;
//        for header in &self.headers {
//            if let Some(child) = iterator.next() {
//                header.as_widget().diff(child);
//            }
//        }
//        for row in &self.data {
//            for cell in row {
//                if let Some(child) = iterator.next() {
//                    cell.as_widget().diff(child);
//                }
//            }
//        }
    }

    fn size(&self) -> Size {
        let mut height = 0.0;
        let mut column_widths = Vec::new();

        for row in &self.data {
            let row_height = row.iter().map(|cell| cell.as_widget().size().height).sum();
            height += row_height;
        }
        let header_height = self.headers.iter().map(|header| header.as_widget().size().height).sum();
        height += header_height;

        // Calculate column widths based on headers
        for header in &self.headers {
            let header_size = header.as_widget().size();
            if column_widths.len() < self.headers.len() {
                column_widths.push(header_size.width);
            } else {
                column_widths[column_widths.len() - 1] = column_widths[column_widths.len() - 1].max(header_size.width);
            }
        }

        let width = self.headers.iter().map(|header| header.as_widget().size().width).sum();

        Size { width: width, height: height }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.max_width(self.column_widths.iter().sum::<Length>());
        let header_layout = self.layout_headers(renderer, &limits, tree);
        let row_layouts = self.layout_rows(renderer, &limits, tree);
        let mut node = Node::new(self.size());
        let mut children = Vec::new();
        children.push(header_layout);
        children.extend(row_layouts);
        node.set_children(children);
        node
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::event::Status {
        match event {
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => {
                let (row, column) = Self::get_hovered_cell(layout, cursor);
                self.hovered_row = row;
                self.hovered_column = column;
            }
            _ => {}
        }
        let mut children = layout.children();
        let header_layout = children.next().expect("Header layout missing");
        let mut header_children = header_layout.children();
        for header in &mut self.headers {
            let header_child_layout = header_children.next().expect("Header child layout missing");
            header.as_widget_mut().on_event(
                &mut state.children.next().unwrap(),
                event.clone(),
                header_child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
        let mut row_iter = self.data.iter().zip(children);
        for state_child in state.children.iter_mut().skip(self.headers.len()) {
            if let Some((row, row_layout)) = row_iter.next() {
                let mut cell_layouts = row_layout.children();
                for cell in row {
                    let cell_layout = cell_layouts.next().expect("Cell layout missing");
                    cell.as_widget_mut().on_event(
                        state_child,
                        event.clone(),
                        cell_layout,
                        cursor,
                        renderer,
                        clipboard,
                        shell,
                        viewport,
                    );
                }
            }
        }
        iced::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::mouse::Interaction {
        let mut children = layout.children();
        let header_layout = children.next().expect("Header layout missing");
        let mut header_children = header_layout.children();
        let mut interaction = iced::mouse::Interaction::default();
        for header in &self.headers {
            let header_child_layout = header_children.next().expect("Header child layout missing");
            interaction = interaction.max(header.as_widget().mouse_interaction(
                &state.children.next().unwrap(),
                header_child_layout,
                cursor,
                viewport,
                renderer,
            ));
        }
        let mut row_iter = self.data.iter().zip(children);
        for state_child in state.children.iter().skip(self.headers.len()) {
            if let Some((row, row_layout)) = row_iter.next() {
                let mut cell_layouts = row_layout.children();
                for cell in row {
                    let cell_layout = cell_layouts.next().expect("Cell layout missing");
                    interaction = interaction.max(cell.as_widget().mouse_interaction(
                        state_child,
                        cell_layout,
                        cursor,
                        viewport,
                        renderer,
                    ));
                }
            }
        }
        interaction
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &iced::Rectangle,
    ) {
        let mut children = layout.children();
        let header_layout = children.next().expect("Header layout missing");
        let mut header_children = header_layout.children();

        // Use the theme to get the header appearance
        let header_appearance = theme.header_appearance(&self.style);

        for (index, header) in self.headers.iter().enumerate() {
            let header_child_layout = header_children.next().expect("Header child layout missing");
            let header_style = if Some(index) == self.hovered_column {
                theme.highlighted_header_appearance(&self.style)
            } else {
                header_appearance
            };
            header.as_widget().draw(
                &state.children[index],
                renderer,
                theme,
                &header_style,
                header_child_layout,
                cursor,
                viewport,
            );
        }
    
        let mut row_iter = self.data.iter().enumerate().zip(children);
        for (row_index, state_child) in state.children.iter().enumerate().skip(self.headers.len()) {
            if let Some(((row_idx, row), row_layout)) = row_iter.next() {
                let mut cell_layouts = row_layout.children();
                for (column_index, cell) in row.iter().enumerate() {
                    let cell_layout = cell_layouts.next().expect("Cell layout missing");
                    let cell_style = if Some(row_idx) == self.hovered_row && Some(column_index) == self.hovered_column {
                        theme.active(&HighlightedCellStyle)
                    } else if Some(row_idx) == self.hovered_row {
                        theme.active(&HighlightedRowStyle)
                    } else if Some(column_index) == self.hovered_column {
                        theme.active(&HighlightedColumnStyle)
                    } else {
                        theme.active(&DefaultCellStyle)
                    };
                    cell.as_widget().draw(
                        &state.children[row_index * self.headers.len() + column_index],
                        renderer,
                        theme,
                        &cell_style,
                        cell_layout,
                        cursor,
                        viewport,
                    );
                }
            }
        }
    }
}