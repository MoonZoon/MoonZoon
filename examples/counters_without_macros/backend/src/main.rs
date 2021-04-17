use moon::*;

async fn init() {}

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Counters without macros example")
        .style("html {
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
        ")
}

async fn up_msg_handler(_: UpMsgRequest) {}

fn main() {
    start!(init, frontend, up_msg_handler).unwrap();
}
