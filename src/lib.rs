/* Command module. */
pub mod command;

/* Response type. */
pub mod response;

/* Serialization (and deserialization) functionalities such as
 * converting bytes into commands and converting reponse into bytes.
 * This should be triggered in machine stage. */
pub mod serialization;

/* FlowType and RetFlowType. */
pub mod flow;

/* Expose Command enum. */
pub type Command = command::command_table::Command;

/* Expose CommandParser struct. */
pub type CommandParser = serialization::command_parser::CommandParser;

/* Expose Response struct. */
pub type Response = response::response::Response;

/* Expose ResponseEncoder struct. */
pub type ResponseEncoder = serialization::response_encoder::ResponseEncoder;

/* Expose FlowType. */
pub type FlowType = flow::flow_type::FlowType;

/* Expose RetFlowType. */
pub type RetFlowType = flow::flow_type::RetFlowType;

/* kyoto Error type. */
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/* Result type for kyoto operations.
 * This is defined as a convinience */
pub type Result<T> = std::result::Result<T, Error>;

/* Flow moving from network stage to machine stage. */
pub fn kyoto_network_to_machine(machine_handler: &mut dyn flow::flow_handler::FlowHandler,
                                flow: FlowType) -> crate::Result<RetFlowType> {
    machine_handler.handle_flow(flow)
}

/* Flow moving from machine stage to warehouse stage. */
pub fn kyoto_machine_to_warehouse(server: &mut dyn flow::flow_handler::FlowHandler,
                                  flow: FlowType) -> crate::Result<RetFlowType> {
    server.handle_flow(flow)
}