use std::borrow::Cow;
use tokio::fs;

pub struct Frontend {
    pub(crate) title: String,
    pub(crate) append_to_head: String,
    pub(crate) body_content: Cow<'static, str>,
}

impl Default for Frontend {
    fn default() -> Self {
        Self {
            title: String::new(),
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
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
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

    pub(crate) async fn render(self, cache_busting: bool) -> String {
        let cache_busting_string = if cache_busting {
            Cow::from(format!("_{}", Self::build_id().await))
        } else {
            Cow::from("")
        };

        format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
          <title>{title}</title>
          <link rel="preload" href="/_api/pkg/frontend_bg{cache_busting_string}.wasm" as="fetch" type="application/wasm" crossorigin>
          <link rel="modulepreload" href="/_api/pkg/frontend{cache_busting_string}.js" crossorigin>
          {append_to_head}
        </head>
    
        <body>
          {body_content}
    
          <script type="text/javascript">
            {reconnecting_event_source}
            {sse}
          </script>
    
          <script type="module">
            import init from '/_api/pkg/frontend{cache_busting_string}.js';
            init('/_api/pkg/frontend_bg{cache_busting_string}.wasm');
          </script>
        </body>
        
        </html>"#,
            title = self.title,
            append_to_head = self.append_to_head,
            body_content = self.body_content,
            reconnecting_event_source = include_str!("../js/ReconnectingEventSource.min.js"),
            sse = include_str!("../js/sse.js"),
            cache_busting_string = cache_busting_string,
        )
    }
}
