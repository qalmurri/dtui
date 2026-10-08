pub const MAX_EXPLORER_DEPTH: usize = 4;

// ======================================================
// EXPLORER NODE
// ======================================================

#[derive(Debug, Clone)]
pub enum ExplorerNode {
    Folder {
        name: String,
        children: Vec<ExplorerNode>,
    },

    Chat {
        name: String,
    },
}

impl ExplorerNode {
    pub fn folder(name: impl Into<String>, children: Vec<ExplorerNode>) -> Self {
        Self::Folder {
            name: name.into(),
            children,
        }
    }

    pub fn chat(name: impl Into<String>) -> Self {
        Self::Chat { name: name.into() }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Folder { name, .. } => name,
            Self::Chat { name } => name,
        }
    }

    pub fn children(&self) -> Option<&[ExplorerNode]> {
        match self {
            Self::Folder { children, .. } => Some(children.as_slice()),

            Self::Chat { .. } => None,
        }
    }

    pub fn is_chat(&self) -> bool {
        matches!(self, Self::Chat { .. })
    }
}

// ======================================================
// EXPLORER STATE
// ======================================================

pub struct ExplorerState {
    pub root: Vec<ExplorerNode>,

    pub path: Vec<usize>,

    pub selected: usize,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            root: Self::create_tree(),

            path: Vec::new(),

            selected: 0,
        }
    }

    // ==================================================
    // TREE DATA
    // ==================================================

    fn create_tree() -> Vec<ExplorerNode> {
        vec![
            // ==========================================
            // ROOT CHAT
            // ==========================================
            ExplorerNode::chat("Ngobrol Umum"),
            // ==========================================
            // INDONESIA
            // ==========================================
            ExplorerNode::folder(
                "Indonesia",
                vec![
                    ExplorerNode::chat("Santai"),
                    ExplorerNode::chat("Tanya Jawab"),
                    ExplorerNode::folder(
                        "Teknologi",
                        vec![
                            ExplorerNode::chat("Linux"),
                            ExplorerNode::chat("Rust"),
                            ExplorerNode::chat("AI"),
                            ExplorerNode::folder(
                                "Programming",
                                vec![
                                    ExplorerNode::chat("Rust Beginner"),
                                    ExplorerNode::chat("Rust Advanced"),
                                    ExplorerNode::chat("Go"),
                                ],
                            ),
                        ],
                    ),
                    ExplorerNode::folder(
                        "Gaming",
                        vec![
                            ExplorerNode::chat("Game PC"),
                            ExplorerNode::chat("Game Console"),
                            ExplorerNode::folder(
                                "Retro",
                                vec![
                                    ExplorerNode::chat("PlayStation 2"),
                                    ExplorerNode::chat("Nintendo"),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
            // ==========================================
            // PROGRAMMING
            // ==========================================
            ExplorerNode::folder(
                "Programming",
                vec![
                    ExplorerNode::chat("General Programming"),
                    ExplorerNode::folder(
                        "Language",
                        vec![
                            ExplorerNode::chat("Rust"),
                            ExplorerNode::chat("Go"),
                            ExplorerNode::chat("TypeScript"),
                        ],
                    ),
                    ExplorerNode::folder(
                        "Backend",
                        vec![
                            ExplorerNode::chat("Django"),
                            ExplorerNode::chat("Go Backend"),
                            ExplorerNode::chat("Node.js"),
                        ],
                    ),
                ],
            ),
            // ==========================================
            // GAMING
            // ==========================================
            ExplorerNode::folder(
                "Gaming",
                vec![
                    ExplorerNode::chat("Gaming General"),
                    ExplorerNode::folder(
                        "PC",
                        vec![
                            ExplorerNode::chat("RPG"),
                            ExplorerNode::chat("FPS"),
                            ExplorerNode::chat("Strategy"),
                        ],
                    ),
                    ExplorerNode::folder(
                        "Console",
                        vec![
                            ExplorerNode::chat("PlayStation"),
                            ExplorerNode::chat("Xbox"),
                            ExplorerNode::chat("Nintendo"),
                        ],
                    ),
                ],
            ),
            // ==========================================
            // MUSIK
            // ==========================================
            ExplorerNode::folder(
                "Musik",
                vec![
                    ExplorerNode::chat("Musik Umum"),
                    ExplorerNode::folder(
                        "Genre",
                        vec![
                            ExplorerNode::chat("Rock"),
                            ExplorerNode::chat("Pop"),
                            ExplorerNode::chat("Jazz"),
                        ],
                    ),
                ],
            ),
        ]
    }

    // ==================================================
    // NAVIGATION
    // ==================================================

    pub fn next(&mut self) {
        let len = self.current_nodes().len();

        if len > 0 && self.selected + 1 < len {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    // ==================================================
    // CURRENT NODES
    // ==================================================

    pub fn current_nodes(&self) -> &[ExplorerNode] {
        Self::nodes_at_path(&self.root, &self.path)
    }

    fn nodes_at_path<'a>(nodes: &'a [ExplorerNode], path: &[usize]) -> &'a [ExplorerNode] {
        let mut current: &'a [ExplorerNode] = nodes;

        for &index in path {
            let Some(node) = current.get(index) else {
                return &[];
            };

            let Some(children) = node.children() else {
                return &[];
            };

            current = children;
        }

        current
    }

    // ==================================================
    // ENTER
    // ==================================================

    pub fn selected_node(&self) -> Option<&ExplorerNode> {
        self.current_nodes().get(self.selected)
    }

    pub fn enter(&mut self) -> ExplorerAction {
        let node = match self.selected_node() {
            Some(node) => node,
            None => {
                return ExplorerAction::None;
            }
        };

        match node {
            ExplorerNode::Chat { .. } => {
                let mut chat_path = self.path.clone();

                chat_path.push(self.selected);

                ExplorerAction::OpenChat(chat_path)
            }

            ExplorerNode::Folder { children, .. } => {
                let next_depth = self.path.len() + 1;

                if next_depth >= MAX_EXPLORER_DEPTH {
                    return ExplorerAction::None;
                }

                if children.is_empty() {
                    return ExplorerAction::None;
                }

                let selected = self.selected;

                self.path.push(selected);
                self.selected = 0;

                ExplorerAction::OpenedFolder
            }
        }
    }

    // ==================================================
    // BACK
    // ==================================================

    pub fn back(&mut self) -> bool {
        if self.path.is_empty() {
            return false;
        }

        self.path.pop();

        self.selected = 0;

        true
    }

    // ==================================================
    // BREADCRUMB
    // ==================================================

    pub fn breadcrumb(&self) -> String {
        let mut names = Vec::new();

        let mut current: &[ExplorerNode] = &self.root;

        for &index in &self.path {
            let Some(node) = current.get(index) else {
                break;
            };

            names.push(node.name());

            let Some(children) = node.children() else {
                break;
            };

            current = children;
        }

        if names.is_empty() {
            "Explorer".to_string()
        } else {
            names.join(" > ")
        }
    }

    // ==================================================
    // CHAT NAME
    // ==================================================

    pub fn chat_name(&self, chat_path: &[usize]) -> String {
        Self::node_at_path(&self.root, chat_path)
            .map(|node| node.name().to_string())
            .unwrap_or_else(|| "Chat".to_string())
    }

    fn node_at_path<'a>(nodes: &'a [ExplorerNode], path: &[usize]) -> Option<&'a ExplorerNode> {
        if path.is_empty() {
            return None;
        }

        let mut current: &'a [ExplorerNode] = nodes;

        for (position, &index) in path.iter().enumerate() {
            let node = current.get(index)?;

            if position == path.len() - 1 {
                return Some(node);
            }

            current = node.children()?;
        }

        None
    }
}

// ======================================================
// EXPLORER ACTION
// ======================================================

pub enum ExplorerAction {
    None,
    OpenedFolder,
    OpenChat(Vec<usize>),
}
