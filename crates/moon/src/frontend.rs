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

    pub(crate) async fn render(self) -> String {
        let frontend_build_id: u128 = fs::read_to_string("frontend/pkg/build_id")
            .await
            .ok()
            .and_then(|uuid| uuid.parse().ok())
            .unwrap_or_default();

        format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
          <title>{title}</title>
          <link rel="preload" href="/pkg/frontend_bg_{frontend_build_id}.wasm" as="fetch" type="application/wasm" crossorigin>
          <link rel="modulepreload" href="/pkg/frontend_{frontend_build_id}.js" crossorigin>
          {append_to_head}
        </head>
    
        <body>
          {body_content}
    
          <script type="text/javascript">
            {reconnecting_event_source}
            var uri = location.protocol + '//' + location.host + '/sse';
            var sse = new ReconnectingEventSource(uri);
            var backendBuildId = null;
            sse.addEventListener("backend_build_id", function(msg) {{
                var newBackendBuildId = msg.data;
                if(backendBuildId === null) {{
                    backendBuildId = newBackendBuildId;
                }} else if(backendBuildId !== newBackendBuildId) {{
                    sse.close();
                    location.reload();
                }}
              }});
            sse.addEventListener("reload", function(msg) {{
              sse.close();
              location.reload();
            }});
          </script>
    
          <script type="module">
            import init from '/pkg/frontend_{frontend_build_id}.js';
            init('/pkg/frontend_bg_{frontend_build_id}.wasm');
          </script>
        </body>
        
        </html>"#,
            title = self.title,
            body_content = self.body_content,
            reconnecting_event_source = include_str!("../js/ReconnectingEventSource.min.js"),
            frontend_build_id = frontend_build_id,
            append_to_head = self.append_to_head
        )
    }
}
