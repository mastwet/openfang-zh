//! Security screen: security feature dashboard and chain verification.

use crate::tui::theme;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

// ── Data types ──────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct SecurityFeature {
    pub name: String,
    pub active: bool,
    pub description: String,
    pub section: SecuritySection,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SecuritySection {
    Core,
    Configurable,
    Monitoring,
}

impl SecuritySection {
    fn label(self) -> &'static str {
        match self {
            Self::Core => "核心安全",
            Self::Configurable => "可配置",
            Self::Monitoring => "监控",
        }
    }
}

// ── Built-in feature definitions ────────────────────────────────────────────

fn builtin_features() -> Vec<SecurityFeature> {
    vec![
        // Core (8)
        SecurityFeature {
            name: "路径遍历防护".into(),
            active: true,
            description: "safe_resolve_path 阻止 ../../ 攻击".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "SSRF 防护".into(),
            active: true,
            description: "在 HTTP 获取中阻止私有 IP 和元数据端点".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "子进程隔离".into(),
            active: true,
            description: "对子进程执行 env_clear() + 选择性变量分配".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "WASM 双重计量".into(),
            active: true,
            description: "Fuel + epoch 中断与看门狗线程".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "能力继承验证".into(),
            active: true,
            description: "validate_capability_inheritance 防止提权".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "密钥清零".into(),
            active: true,
            description: "Zeroizing<String> 自动从内存中擦除 API 密钥".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "Ed25519 清单签名".into(),
            active: true,
            description: "使用 Ed25519 验证已签名的智能体清单".into(),
            section: SecuritySection::Core,
        },
        SecurityFeature {
            name: "污点追踪".into(),
            active: true,
            description: "跨工具边界的信息流追踪".into(),
            section: SecuritySection::Core,
        },
        // Configurable (4)
        SecurityFeature {
            name: "OFP 线路认证".into(),
            active: true,
            description: "带有 Nonce 的 HMAC-SHA256 双向认证".into(),
            section: SecuritySection::Configurable,
        },
        SecurityFeature {
            name: "RBAC 多用户".into(),
            active: true,
            description: "带有用户层级的基于角色的访问控制".into(),
            section: SecuritySection::Configurable,
        },
        SecurityFeature {
            name: "速率限制".into(),
            active: true,
            description: "具有成本感知的 GCRA 速率限制器".into(),
            section: SecuritySection::Configurable,
        },
        SecurityFeature {
            name: "安全响应头".into(),
            active: true,
            description: "CSP, X-Frame-Options, HSTS 中间件".into(),
            section: SecuritySection::Configurable,
        },
        // Monitoring (3)
        SecurityFeature {
            name: "Merkle 审计追踪".into(),
            active: true,
            description: "具有篡改检测功能的哈希链审计日志".into(),
            section: SecuritySection::Monitoring,
        },
        SecurityFeature {
            name: "心跳监控".into(),
            active: true,
            description: "带有重启限制的后台健康检查".into(),
            section: SecuritySection::Monitoring,
        },
        SecurityFeature {
            name: "提示词注入扫描".into(),
            active: true,
            description: "检测覆盖尝试和数据外泄".into(),
            section: SecuritySection::Monitoring,
        },
    ]
}


// ── State ───────────────────────────────────────────────────────────────────

pub struct SecurityState {
    pub features: Vec<SecurityFeature>,
    pub chain_verified: Option<bool>,
    pub verify_result: String,
    pub scroll: u16,
    pub loading: bool,
    pub tick: usize,
}

pub enum SecurityAction {
    Continue,
    Refresh,
    VerifyChain,
}

impl SecurityState {
    pub fn new() -> Self {
        Self {
            features: builtin_features(),
            chain_verified: None,
            verify_result: String::new(),
            scroll: 0,
            loading: false,
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> SecurityAction {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return SecurityAction::Continue;
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll = self.scroll.saturating_add(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll = self.scroll.saturating_sub(1);
            }
            KeyCode::PageUp => {
                self.scroll = self.scroll.saturating_add(10);
            }
            KeyCode::PageDown => {
                self.scroll = self.scroll.saturating_sub(10);
            }
            KeyCode::Char('v') => return SecurityAction::VerifyChain,
            KeyCode::Char('r') => return SecurityAction::Refresh,
            _ => {}
        }
        SecurityAction::Continue
    }
}

// ── Drawing ─────────────────────────────────────────────────────────────────

pub fn draw(f: &mut Frame, area: Rect, state: &mut SecurityState) {
    let block = Block::default()
        .title(Line::from(vec![Span::styled(
            " 安全 ",
            theme::title_style(),
        )]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ACCENT))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Min(4),    // features
        Constraint::Length(2), // verify result
        Constraint::Length(1), // hints
    ])
    .split(inner);

    // ── Features list ──
    let mut lines: Vec<Line> = Vec::new();
    let mut current_section: Option<SecuritySection> = None;

    for feat in &state.features {
        if current_section != Some(feat.section) {
            if current_section.is_some() {
                lines.push(Line::raw(""));
            }
            lines.push(Line::from(vec![Span::styled(
                format!(
                    "  \u{2501}\u{2501} {} \u{2501}\u{2501}",
                    feat.section.label()
                ),
                Style::default()
                    .fg(theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            )]));
            current_section = Some(feat.section);
        }

        let (badge, badge_style) = if feat.active {
            ("\u{2714} 已开启", Style::default().fg(theme::GREEN))
        } else {
            ("\u{25cb} 已关闭", Style::default().fg(theme::RED))
        };

        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:<30}", feat.name),
                Style::default()
                    .fg(theme::CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" {:<12}", badge), badge_style),
            Span::styled(format!(" {}", feat.description), theme::dim_style()),
        ]));
    }

    let total = lines.len() as u16;
    let visible = chunks[0].height;
    let max_scroll = total.saturating_sub(visible);
    let scroll = max_scroll.saturating_sub(state.scroll).min(max_scroll);

    f.render_widget(Paragraph::new(lines).scroll((scroll, 0)), chunks[0]);

    // ── Verify result ──
    match state.chain_verified {
        None => {
            if state.loading {
                let spinner = theme::SPINNER_FRAMES[state.tick % theme::SPINNER_FRAMES.len()];
                f.render_widget(
                    Paragraph::new(Line::from(vec![
                        Span::styled(format!("  {spinner} "), Style::default().fg(theme::CYAN)),
                        Span::styled("正在验证审计链\u{2026}", theme::dim_style()),
                    ])),
                    chunks[1],
                );
            } else {
                f.render_widget(
                    Paragraph::new(Line::from(vec![Span::styled(
                        "  按 [v] 键验证审计链完整性",
                        theme::dim_style(),
                    )])),
                    chunks[1],
                );
            }
        }
        Some(true) => {
            f.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![Span::styled(
                        "  \u{2714} 审计链验证成功",
                        Style::default().fg(theme::GREEN),
                    )]),
                    Line::from(vec![Span::styled(
                        format!("  {}", state.verify_result),
                        theme::dim_style(),
                    )]),
                ]),
                chunks[1],
            );
        }
        Some(false) => {
            f.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![Span::styled(
                        "  \u{2718} 审计链验证失败",
                        Style::default().fg(theme::RED),
                    )]),
                    Line::from(vec![Span::styled(
                        format!("  {}", state.verify_result),
                        Style::default().fg(theme::RED),
                    )]),
                ]),
                chunks[1],
            );
        }
    }

    // ── Hints ──
    f.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            "  [\u{2191}\u{2193}] 滚动  [v] 验证链  [r] 刷新",
            theme::hint_style(),
        )])),
        chunks[2],
    );
}

