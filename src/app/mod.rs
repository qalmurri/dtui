pub mod chat;
pub mod client;
pub mod dashboard;
pub mod explorer;
pub mod options;

use crate::screen::Screen;

use self::{
    chat::{ChatState, Message},
    client::ClientState,
    dashboard::{DashboardItem, DashboardState},
    explorer::{ExplorerAction, ExplorerState},
    options::OptionsState,
};

// ======================================================
// APP
// ======================================================

pub struct App {
    pub screen: Screen,

    pub dashboard: DashboardState,
    pub explorer: ExplorerState,
    pub chat: ChatState,
    pub options: OptionsState,
    pub client: ClientState,

    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Dashboard,

            dashboard: DashboardState::new(),
            explorer: ExplorerState::new(),
            chat: ChatState::new(),
            options: OptionsState::new(),
            client: ClientState::new(),

            running: true,
        }
    }

    // ======================================================
    // NEXT
    // ======================================================

    pub fn next(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                self.dashboard.next();
            }

            Screen::Explorer => {
                self.explorer.next();
            }

            Screen::Options => {
                self.options.next();
            }

            Screen::Chat | Screen::Config | Screen::JoinPrivate | Screen::CreateLobby => {}
        }
    }

    // ======================================================
    // PREVIOUS
    // ======================================================

    pub fn previous(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                self.dashboard.previous();
            }

            Screen::Explorer => {
                self.explorer.previous();
            }

            Screen::Options => {
                self.options.previous();
            }

            Screen::Chat | Screen::Config | Screen::JoinPrivate | Screen::CreateLobby => {}
        }
    }

    // ======================================================
    // ENTER
    // ======================================================

    pub fn enter(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                self.dashboard_enter();
            }

            Screen::Explorer => {
                self.explorer_enter();
            }

            Screen::Chat => {}

            Screen::Options => {
                self.options.toggle();
            }

            Screen::Config => {}

            Screen::JoinPrivate => {}

            Screen::CreateLobby => {}
        }
    }

    // ======================================================
    // DASHBOARD
    // ======================================================

    fn dashboard_enter(&mut self) {
        match self.dashboard.selected_item() {
            DashboardItem::JoinPublic => {
                self.screen = Screen::Explorer;

                self.explorer.path.clear();
                self.explorer.selected = 0;
            }

            DashboardItem::JoinPrivate => {
                self.screen = Screen::JoinPrivate;
            }

            DashboardItem::CreateLobby => {
                self.screen = Screen::CreateLobby;
            }

            DashboardItem::Options => {
                self.screen = Screen::Options;

                self.options.selected = 0;
            }

            DashboardItem::Config => {
                self.screen = Screen::Config;
            }
        }
    }

    // ======================================================
    // EXPLORER
    // ======================================================

    fn explorer_enter(&mut self) {
        match self.explorer.enter() {
            ExplorerAction::None => {}

            ExplorerAction::OpenedFolder => {}

            ExplorerAction::OpenChat(path) => {
                self.chat.path = path;

                self.load_messages();

                self.screen = Screen::Chat;
            }
        }
    }

    // ======================================================
    // BACK
    // ======================================================

    pub fn back(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                self.quit();
            }

            Screen::Explorer => {
                if !self.explorer.back() {
                    self.screen = Screen::Dashboard;
                }
            }

            Screen::Chat => {
                self.screen = Screen::Explorer;
            }

            Screen::Options | Screen::Config | Screen::JoinPrivate | Screen::CreateLobby => {
                self.screen = Screen::Dashboard;
            }
        }
    }

    // ======================================================
    // CHAT
    // ======================================================

    fn load_messages(&mut self) {
        let room = self.explorer.chat_name(&self.chat.path);

        self.chat.messages = vec![
            Message {
                sender: "Anonymous".to_string(),

                content: format!("Selamat datang di {}.", room),

                timestamp: "14:32".to_string(),
            },
            Message {
                sender: "Anonymous".to_string(),

                content: "Halo semuanya 👋".to_string(),

                timestamp: "14:33".to_string(),
            },
            Message {
                sender: "Anonymous".to_string(),

                content: "Ini masih client prototype.".to_string(),

                timestamp: "14:34".to_string(),
            },
            Message {
                sender: self.client.alias.clone(),

                content: "Oh, nice.".to_string(),

                timestamp: "14:35".to_string(),
            },
        ];
    }

    // ======================================================
    // CHAT BREADCRUMB
    // ======================================================

    pub fn chat_breadcrumb(&self) -> String {
        let mut names = Vec::new();

        let mut current = &self.explorer.root[..];

        for &index in &self.chat.path {
            let Some(node) = current.get(index) else {
                break;
            };

            names.push(node.name());

            if let Some(children) = node.children() {
                current = children;
            }
        }

        if names.is_empty() {
            "Chat".to_string()
        } else {
            format!("{} > Chat", names.join(" > "))
        }
    }

    // ======================================================
    // QUIT
    // ======================================================

    pub fn quit(&mut self) {
        self.running = false;
    }
}
