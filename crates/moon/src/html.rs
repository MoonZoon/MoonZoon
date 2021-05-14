pub fn html(
    title: &str,
    backend_build_id: u128,
    frontend_build_id: u128,
    append_to_head: &str,
    body_content: &str,
) -> String {
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
      {html_debug_info}
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
        title = title,
        html_debug_info = html_debug_info(backend_build_id),
        body_content = body_content,
        reconnecting_event_source = include_str!("../js/ReconnectingEventSource.min.js"),
        frontend_build_id = frontend_build_id.to_string(),
        append_to_head = append_to_head
    )
}

pub fn html_debug_info(_backend_build_id: u128) -> String {
    String::new()
    // format!("<h1>MoonZoon is running!</h1>
    //     <h2>Backend build id: {backend_build_id}</h2>
    //     <h2>Random id: {random_id}</h2>",
    //     backend_build_id = backend_build_id.to_string(),
    //     random_id = Uuid::new_v4().to_string()
    // )
}
