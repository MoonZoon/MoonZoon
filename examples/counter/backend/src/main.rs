use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Counter example")
        .append_to_head("<style>html {
            background-color: black;
            color: lightgray;
        }
        
        #app * {
            padding: 5px;
        }
            
        .button {
            cursor: pointer;
            background-color: darkgreen;
            user-select: none;
        }
        
        .button:hover {
            background-color: green;
        }
        
        .column {
            display: flex;
            flex-direction: column;
        }
        
        .row {
            display: flex;
        }
        </style>")
}

async fn up_msg_handler(_: UpMsgRequest) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_|{}).await
}
