use crate::CONFIG;
use lang::Lang;
use std::borrow::Cow;
use tokio::fs;

pub struct Frontend {
    pub(crate) lang: Lang,
    pub(crate) index_by_robots: bool,
    pub(crate) title: Cow<'static, str>,
    pub(crate) default_styles: bool,
    pub(crate) append_to_head: String,
    pub(crate) body_content: Cow<'static, str>,
}

impl Default for Frontend {
    fn default() -> Self {
        Self {
            lang: Lang::English,
            index_by_robots: true,
            title: Cow::from("MoonZoon app"),
            default_styles: true,
            append_to_head: String::new(),
            body_content: Cow::from(r#"<section id="app"></section>"#),
        }
    }
}

impl Frontend {
    pub(crate) async fn build_id() -> u128 {
        fs::read_to_string("frontend/pkg/build_id")
            .await
            .ok()
            .and_then(|uuid| uuid.parse().ok())
            .unwrap_or_default()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    pub fn index_by_robots(mut self, allow: bool) -> Self {
        self.index_by_robots = allow;
        self
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn default_styles(mut self, enabled: bool) -> Self {
        self.default_styles = enabled;
        self
    }

    pub fn append_to_head(mut self, html: &str) -> Self {
        self.append_to_head.push_str(html);
        self
    }

    pub fn body_content(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        self.body_content = content.into();
        self
    }

    pub async fn into_html(self) -> String {
        let Frontend {
            lang,
            index_by_robots,
            title,
            default_styles,
            append_to_head,
            body_content,
        } = self;

        let cache_busting_string = if CONFIG.cache_busting {
            Cow::from(format!("_{}", Self::build_id().await))
        } else {
            Cow::from("")
        };

        let meta_robots = if index_by_robots {
            ""
        } else {
            r#"<meta name="robots" content="noindex">"#
        };

        let default_styles = if default_styles {
            concat!(
                "<style>",
                include_str!("../css/modern-normalize.min.css"),
                "</style>",
                "<style>",
                include_str!("../css/basic.css"),
                "</style>"
            )
        } else {
            ""
        };

        let scripts = if CONFIG.frontend_dist {
            String::new()
        } else {
            let reconnecting_event_source_js_code =
                include_str!("../js/ReconnectingEventSource.min.js");
            let sse_js_code = include_str!("../js/sse.js");
            format!(
                r#"<script type="text/javascript">
                    {reconnecting_event_source_js_code}
                    {sse_js_code}
                </script>"#
            )
        };

        format!(
            r#"<!DOCTYPE html>
        <html lang="{lang}">
        
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
          {meta_robots}
          <title>{title}</title>
          <link rel="preload" href="/_api/pkg/frontend_bg{cache_busting_string}.wasm" as="fetch" type="application/wasm" crossorigin>
          <link rel="modulepreload" href="/_api/pkg/frontend{cache_busting_string}.js" crossorigin>
          {default_styles}
          {append_to_head}
        </head>
    
        <body>
          {body_content}
    
          {scripts}
    
          <script type="module">
            import init from '/_api/pkg/frontend{cache_busting_string}.js';
            init('/_api/pkg/frontend_bg{cache_busting_string}.wasm');
          </script>
        </body>
        
        </html>"#
        )
    }
}
